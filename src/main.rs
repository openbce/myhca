use std::env;

use libudev;
use log;
use uname::uname;

#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;
    env_logger::init();

    let context = libudev::Context::new()?;

    // uname to detect type
    let info = uname()?;
    log::info!("uname - {:?}", info);

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

    Ok(())
}
