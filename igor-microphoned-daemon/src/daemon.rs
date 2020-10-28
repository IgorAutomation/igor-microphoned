
use structopt::StructOpt;

use igormdlib::util::find_host_id_by_name;
use igormdlib::util::list_host_names;

use crate::cmdline::Opt;
use cpal::traits::{HostTrait, DeviceTrait};

pub fn launch() {

    let opt: Opt = Opt::from_args();

    let host = if let Some(ref host_name) = opt.host {
        let host_id = find_host_id_by_name(host_name)
            .expect(format!("ERROR: host not found: {}", host_name).as_str());
        let host = cpal::host_from_id(host_id)
            .expect(format!("ERROR: host not found: {}", host_name).as_str());
        Some(host)
    } else {
        None
    };

    println!();

    if opt.list_hosts {
        println!("Detected audio hosts:");
        for host_name in list_host_names() {
            println!("\t- {}", host_name);
        }
        println!();
    }

    match (opt.list_devices, host, opt.host) {
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

}

