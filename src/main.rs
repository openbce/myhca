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

    println!("{:?}", info);

    // Nics
    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("infiniband")?;
    // enumerator.match_subsystem("net")?;
    let devices = enumerator.scan_devices()?;

    for device in devices {
        if !device
            .property_value("ID_PCI_SUBCLASS_FROM_DATABASE")
            .filter(|v| v.eq_ignore_ascii_case("Infiniband controller"))
            .is_some() {
                continue
        }
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
