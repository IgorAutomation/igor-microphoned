
use structopt::StructOpt;

use igormdlib::util::find_host_id_by_name;
use igormdlib::util::list_host_names;
use igormdlib::util::find_device_by_name;

use crate::cmdline::Opt;
use cpal::traits::{HostTrait, DeviceTrait};

pub fn launch() {

    let opt: Opt = Opt::from_args();

    // get host
    let selected_host = if let Some(ref host_name) = opt.host {
        let host_id = find_host_id_by_name(host_name)
            .expect(format!("ERROR: host not found: {}", host_name).as_str());
        let host = cpal::host_from_id(host_id)
            .expect(format!("ERROR: host not found: {}", host_name).as_str());
        Some(host)
    } else {
        None
    };

    // get device
    let selected_device = if let (Some(ref host), Some(ref device_name)) = (&selected_host, opt.device) {
        let device = find_device_by_name(host, device_name)
            .expect(format!("ERROR: device not found: {} on host {}", device_name, host.id().name()).as_str());
        device
    } else {
        None
    };

    println!();

    // list hosts
    if opt.list_hosts {
        println!("Detected audio hosts:");
        for host_name in list_host_names() {
            println!("\t- {}", host_name);
        }
        println!();
    }

    // list devices
    match (opt.list_devices, &selected_host, opt.host) {
        (true, Some(ref host), Some(ref host_name)) => {
            println!("Detected devices:");
            let devices = host.input_devices()
                .expect(format!("ERR: unable to get input devices for host {} ", host_name).as_str());
            for device in devices {
                println!("\t- {}", device.name().unwrap_or("Unknown".to_string()));
            }
            println!();
        },
        (true, None, _) => {
            panic!("ERROR: A host must be specified");
        },
        _ => {}
    }

    // show selected host
    if let Some(host) = &selected_host {
        println!("Selected host: {}", host.id().name());
    }

    // show selected device
    if let Some(device) = &selected_device {
        let name = device.name()
            .unwrap_or_else(|_e| "Unknown".to_string());
        println!("Selected device: {}", name);
    }

    

}

