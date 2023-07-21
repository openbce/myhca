/*
Copyright 2023 The xflops Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod wrappers;

use std::alloc::{self, Layout};
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Display;
use std::os::raw::c_int;
use std::ptr::NonNull;
use std::{fmt, slice};
use std::{io, vec};

use libudev;
use libudev::Device;
use numeric_cast::NumericCast;

use wrappers::ibverbs::{
    self, ibv_device, ibv_device_attr, /*ibv_get_device_index,*/ ibv_get_device_list,
    ibv_open_device, ibv_port_attr, ibv_query_device, ibv_query_port,
};

#[derive(Clone)]
pub struct PciDevice {
    pub id: String,
    pub subsys_id: String,
    pub model_name: String,
    pub vendor_name: String,
    pub vendor: String,
    pub ib_devices: Vec<IbDevice>,
}

impl TryFrom<Device> for PciDevice {
    type Error = io::Error;
    fn try_from(dev: Device) -> Result<Self, Self::Error> {
        Ok(Self {
            id: get_property(&dev, "PCI_ID")?.to_string(),
            subsys_id: get_property(&dev, "PCI_SUBSYS_ID")?.to_string(),
            model_name: get_property(&dev, "ID_MODEL_FROM_DATABASE")?.to_string(),
            vendor_name: get_property(&dev, "ID_VENDOR_FROM_DATABASE")?.to_string(),
            vendor: get_sysattr(&dev, "vendor")?.to_string(),
            ib_devices: vec![],
        })
    }
}

#[derive(Clone)]
pub struct IbDevice {
    pub name: String,
    pub slot_name: String,
    pub node_guid: String,
    pub node_desc: String,
    pub sys_image_guid: String,
    pub fw_ver: String,
    pub board_id: String,
    pub ib_ports: Vec<IbPort>,
}

impl TryFrom<Device> for IbDevice {
    type Error = io::Error;
    fn try_from(dev: Device) -> Result<Self, Self::Error> {
        let slot_name = match dev.parent() {
            Some(p) => get_property(&p, "PCI_SLOT_NAME")?.to_string(),
            None => String::new(),
        };
        Ok(Self {
            name: get_property(&dev, "NAME")?.to_string(),
            slot_name,
            node_guid: get_sysattr(&dev, "node_guid")?.to_string(),
            node_desc: get_sysattr(&dev, "node_desc")?.to_string(),
            sys_image_guid: get_sysattr(&dev, "sys_image_guid")?.to_string(),
            fw_ver: get_sysattr(&dev, "fw_ver")?.to_string(),
            board_id: get_sysattr(&dev, "board_id")?.to_string(),
            ib_ports: vec![],
        })
    }
}

#[derive(Clone)]
pub enum IbPortLinkType {
    Ethernet,
    Infiniband,
}

impl TryFrom<u8> for IbPortLinkType {
    type Error = io::Error;
    fn try_from(v: u8) -> io::Result<Self> {
        match v {
            1 => Ok(Self::Infiniband),
            2 => Ok(Self::Ethernet),
            _ => Err(io::Error::last_os_error()),
        }
    }
}

impl Display for IbPortLinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ethernet => f.write_str("Eth"),
            Self::Infiniband => f.write_str("IB"),
        }
    }
}

#[derive(Clone)]
pub enum IbPortState {
    Initializing,
    Active,
    Down,
}

impl Display for IbPortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initializing => write!(f, "{}", "Initializing"),
            Self::Active => write!(f, "{}", "Active"),
            Self::Down => write!(f, "{}", "Down"),
        }
    }
}

impl TryFrom<u32> for IbPortState {
    type Error = io::Error;
    fn try_from(v: u32) -> io::Result<Self> {
        match v {
            ibverbs::ibv_port_state::IBV_PORT_INIT => Ok(Self::Initializing),
            ibverbs::ibv_port_state::IBV_PORT_ACTIVE => Ok(Self::Active),
            ibverbs::ibv_port_state::IBV_PORT_DOWN => Ok(Self::Down),

            _ => Err(io::Error::last_os_error()),
        }
    }
}

#[derive(Clone)]
pub enum IbPortPhysState {
    Polling,
    LinkUp,
    Disabled,
}

impl Display for IbPortPhysState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Polling => f.write_str("Polling"),
            Self::LinkUp => f.write_str("LinkUp"),
            Self::Disabled => f.write_str("Disabled"),
        }
    }
}

impl TryFrom<u8> for IbPortPhysState {
    type Error = io::Error;
    fn try_from(v: u8) -> io::Result<Self> {
        match v {
            2 => Ok(Self::Polling),
            3 => Ok(Self::Disabled),
            5 => Ok(Self::LinkUp),

            _ => Err(io::Error::last_os_error()),
        }
    }
}

#[derive(Clone)]
pub struct IbPort {
    pub port_num: u8,
    pub guid: u64,
    pub lid: u16,
    pub link_type: IbPortLinkType,
    pub state: IbPortState,
    pub phys_state: IbPortPhysState,
}

#[allow(missing_copy_implementations)] // This type can not copy
#[repr(transparent)]
struct DevicePtr(NonNull<ibv_device>);

impl DevicePtr {
    fn ffi_ptr(&self) -> *mut ibv_device {
        return self.0.as_ptr();
    }
}

#[allow(missing_copy_implementations)] // This type can not copy
#[repr(transparent)]
struct DeviceAttrPtr(NonNull<ibv_device_attr>);

impl DeviceAttrPtr {
    fn ffi_ptr(&self) -> *mut ibv_device_attr {
        return self.0.as_ptr();
    }
}

/// List the HCAs on the host.
pub fn list_pci_devices() -> io::Result<Vec<PciDevice>> {
    let mut ibv_devs = HashMap::<String, Vec<IbPort>>::new();

    unsafe {
        let mut num_devices: c_int = 0;
        let device_list = ibv_get_device_list(&mut num_devices);
        if device_list.is_null() {
            return Err(io::Error::last_os_error());
        }

        let device_list: NonNull<DevicePtr> = NonNull::new_unchecked(device_list.cast());
        let len: usize = num_devices.numeric_cast();

        let devices = slice::from_raw_parts(device_list.as_ptr(), len);

        for devptr in devices {
            let ctx = ibv_open_device(devptr.ffi_ptr());
            if ctx.is_null() {
                return Err(io::Error::last_os_error());
            }

            let dev_attr_ptr =
                alloc::alloc(Layout::new::<ibv_device_attr>()) as *mut ibv_device_attr;

            if ibv_query_device(ctx, dev_attr_ptr) != 0 {
                return Err(io::Error::last_os_error());
            };

            let mut ports = vec![];

            for i in 1..=(*dev_attr_ptr).phys_port_cnt {
                let port_attr_ptr =
                    alloc::alloc(Layout::new::<ibv_port_attr>()) as *mut ibv_port_attr;

                if ibv_query_port(ctx, i, port_attr_ptr as *mut _) != 0 {
                    return Err(io::Error::last_os_error());
                };

                ports.push(IbPort {
                    port_num: i,
                    lid: (*port_attr_ptr).lid,
                    link_type: IbPortLinkType::try_from((*port_attr_ptr).link_layer)?,
                    guid: (*dev_attr_ptr).node_guid,
                    state: IbPortState::try_from((*port_attr_ptr).state)?,
                    phys_state: IbPortPhysState::try_from((*port_attr_ptr).phys_state)?,
                });
            }

            ibv_devs.insert(cstr_to_string((*devptr.ffi_ptr()).name.as_ptr()), ports);
        }
    };

    let context = libudev::Context::new()?;

    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("infiniband")?;
    let devices = enumerator.scan_devices()?;

    let mut pci_devs = HashMap::<String, PciDevice>::new();
    for device in devices {
        if let Some(parent) = device.parent() {
            let pci_dev = PciDevice::try_from(parent)?;
            let pci_dev = pci_devs.entry(pci_dev.id.clone()).or_insert(pci_dev);

            let mut ib_dev = IbDevice::try_from(device)?;
            ib_dev.ib_ports = ibv_devs
                .get(&ib_dev.name)
                .unwrap_or(&Vec::<IbPort>::new())
                .to_vec();

            pci_dev.ib_devices.push(ib_dev);
        }
    }

    Ok(pci_devs.into_values().collect())
}

unsafe fn cstr_to_string(s: *const i8) -> String {
    CStr::from_ptr(s)
        .to_str()
        .expect("not an utf8 string")
        .to_string()
}

fn get_property<'a>(device: &'a Device, name: &'a str) -> io::Result<&'a str> {
    match device.property_value(name) {
        None => Err(io::Error::last_os_error()),
        Some(p) => p
            .to_str()
            .map(|s| s.trim())
            .ok_or_else(|| io::Error::last_os_error()),
    }
}

fn get_sysattr<'a>(device: &'a Device, name: &'a str) -> io::Result<&'a str> {
    match device.attribute_value(name) {
        None => Err(io::Error::last_os_error()),
        Some(p) => p
            .to_str()
            .map(|s| s.trim())
            .ok_or_else(|| io::Error::last_os_error()),
    }
}
