/*
Copyright 2023 The openBCE Authors.

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

use std::alloc::{alloc, dealloc, Layout};

mod wrappers;

use wrappers::pci;

/// HCA
pub struct HCA {
    pub description: String,
    pub serial_number: String,
    pub vendor_id: u32,
    pub driver: String,
    pub functions: Vec<Function>,
}

/// Function is the interal concept for Node/Port on the host.
pub struct Function {
    pub guid: String,
    pub lid: i32,
}

pub struct PciDevice {
    pub vendor_id: u32,
    pub device_id: u32,
}

unsafe fn scan_device(p: *mut pci::pci_dev) {}

pub fn list_pci_device() -> Vec<PciDevice> {
    let mut pcidev = vec![];

    unsafe {
        let pacc = pci::pci_alloc();
        pci::pci_init(pacc);

        pci::pci_scan_bus(pacc);

        let mut dev = (*pacc).devices;

        loop {
            if dev.is_null() {
                break;
            }

            // Start to fill device info.
            let cache_layout = Layout::array::<u8>(64).unwrap();

            let mut config_cached = 64;
            let mut config_bufsize = 64;

            let config: *mut u8 = alloc(cache_layout) as *mut u8;
            let present: *mut u8 = alloc(cache_layout) as *mut u8;

            if pci::pci_read_block(dev, 0, config, 64) == 0 {
                dealloc(config, cache_layout);
                dealloc(present, cache_layout);

                continue;
            }

            pci::pci_setup_cache(dev, config, config_cached);
            pci::pci_fill_info(dev, (pci::PCI_FILL_IDENT | pci::PCI_FILL_CLASS) as i32);
            // -- fill device end.

            // let htype = config[pci::PCI_HEADER_TYPE] & 0x7f;
            // if htype == pci::PCI_HEADER_TYPE_NORMAL {

            // }

            pcidev.push(PciDevice {
                vendor_id: (*dev).vendor_id as u32,
                device_id: (*dev).device_id as u32,
            });

            dev = (*dev).next;

            // Clearup memory.
            dealloc(config, cache_layout);
            dealloc(present, cache_layout);
        }

        pci::pci_cleanup(pacc);
    };

    pcidev
}

/// List the HCAs on the host.
pub fn list_hca() -> Vec<HCA> {
    let mut hcas = vec![];

    let _pcidev = list_pci_device();

    hcas
}
