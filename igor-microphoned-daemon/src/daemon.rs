
use structopt::StructOpt;

use igormdlib::util::{find_host_id_by_name, convert_sample_for_deepspeech};
use igormdlib::util::list_host_names;
use igormdlib::util::find_device_by_name;

use crate::cmdline::Opt;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::{StreamConfig, BufferSize, SampleFormat, SupportedBufferSize};

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


        if let Ok(sc) = device.supported_input_configs() {
            println!("Selected device: {}", name);
            for config in sc {
                println!("\t========================================");
                println!("\tchannel_count: {}", config.channels());
                println!("\tmin_sample_rate: {}", config.min_sample_rate().0);
                println!("\tmax_sample_rate: {}", config.max_sample_rate().0);
                match config.buffer_size() {
                    SupportedBufferSize::Range { min, max } => {
                        println!("\tbuffer_size: {} to {}", min, max);
                    },
                    SupportedBufferSize::Unknown => (),
                }
                match config.sample_format() {
                    SampleFormat::I16 => println!("\tsample_format: I16"),
                    SampleFormat::U16 => println!("\tsample_format: U16"),
                    SampleFormat::F32 => println!("\tsample_format: F32"),
                }
            }

        } else {
            eprintln!("Unable to enumerate supported configuration");
        }


    }

    // read words
    match (opt.read_words, &selected_device) {
        (true, Some(ref device)) => {

            let mut supported_configs_range = device.supported_input_configs()
                .expect("Supported configs");

            let supported_config = supported_configs_range.next()
                .expect("no supported config?!")
                .with_max_sample_rate();

            let stream_config = StreamConfig {
                channels: 1,
                buffer_size: BufferSize::Fixed(1234),
                sample_rate: supported_config.sample_rate()
            };

            let stream = device.build_input_stream_raw(
                &stream_config,
                supported_config.sample_format(),
                move |data, &_ | {
                    let converted_data = convert_sample_for_deepspeech(data);
                    println!("data in size: {}", converted_data.len().to_string());
                },
                move |_| eprintln!("an error occurred on stream"))
                .expect("Unable to build raw input stream from device");

            // TODO: ass fuck
            stream.play();
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        _ => {}
    };


}

