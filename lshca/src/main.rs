use uname::uname;

use ::libhca;

#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    // uname to detect type
    let info = uname()?;
    print!("{:?}\n\n", info);

    let hcas = libhca::list_hca();
    for hca in hcas {
        println!("{:<15}: {}", "Description", hca.description);
        println!("{:<15}: {}", "SerialNumber", hca.serial_number);
    }

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
