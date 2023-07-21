use ::libhca;

#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let hcas = libhca::list_pci_devices()?;

    for hca in hcas {
        println!("----------------------------------------------");

        println!("{:<15}: {}", "ID", hca.id);
        println!("{:<15}: {}", "SubSys ID", hca.subsys_id);
        println!("{:<15}: {}", "Model", hca.model_name);
        println!("{:<15}: {}", "Vendor", hca.vendor_name);

        println!("");

        println!(
            "    {:<15}{:<25}{:<15}{:<15}{:<25}{:<15}{:<15}{:<15}",
            "Name", "Node GUID", "Port Num", "LID", "GUID", "LinkType", "State", "PhysState"
        );

        for dev in hca.ib_devices {
            for port in dev.ib_ports {
                println!(
                    "    {:<15}{:<25}{:<15}{:<15}{:<25}{:<15}{:<15}{:<15}",
                    dev.name,
                    dev.node_guid,
                    port.port_num,
                    port.lid,
                    port.guid,
                    port.link_type.to_string(),
                    port.state.to_string(),
                    port.phys_state.to_string(),
                );
            }
        }

        println!("");
        println!("");
    }

    //    let subscriber = tracing_subscriber::FmtSubscriber::new();
    //    // use that subscriber to process traces emitted after this point
    //    tracing::subscriber::set_global_default(subscriber)?;
    //
    //    let context = libudev::Context::new()?;
    //
    //    let device_debug_log = |device: &Device| {
    //        // let device = device.parent().unwrap();
    //        tracing::info!("SysPath - {:?}", device.syspath());
    //        for p in device.properties() {
    //            tracing::info!("Property - {:?} - {:?}", p.name(), p.value());
    //        }
    //        for a in device.attributes() {
    //            tracing::info! {"attribute - {:?} - {:?}", a.name(), a.value()}
    //        }
    //    };
    //
    //    let mut enumerator = libudev::Enumerator::new(&context)?;
    //    enumerator.match_subsystem("infiniband")?;
    //    let devices = enumerator.scan_devices()?;
    //
    //    for device in devices {
    //        device_debug_log(&device);
    //    }
    //
    /*
    let args: Vec<String> = env::args().collect();
    let subsystem: String = {
        if args.len() > 1 {
            args[1].clone()
        } else {
            "infiniband".to_string()
        }
    };

    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem(subsystem)?;
    let devices = enumerator.scan_devices()?;

    for device in devices {
        log::info!("SysPath - {:?}", device.syspath());
        for p in device.properties() {
            log::info!("Property - {:?} - {:?}", p.name(), p.value());
        }
        for a in device.attributes() {
            log::info! {"attribute - {:?} - {:?}", a.name(), a.value()}
        }
    }
    */

    Ok(())
}
