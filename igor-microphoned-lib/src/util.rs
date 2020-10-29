use cpal::{Device, Host, HostId};
use cpal::traits::{DeviceTrait, HostTrait};

use crate::errors::Error;

pub fn list_host_names() -> impl Iterator<Item = &'static str> {
    cpal::available_hosts()
        .into_iter()
        .map(|h| h.name())
}

pub fn find_host_id_by_name(name: &str) -> Option<HostId> {
    cpal::available_hosts()
        .into_iter()
        .find(|h| h.name().eq_ignore_ascii_case(name))
}

pub fn find_host_by_name(host_name: &str) -> Result<Option<Host>, Error> {
    if let Some(host_id) = find_host_id_by_name(host_name) {
        cpal::host_from_id(host_id)
            .map(Option::from)
            .map_err(Error::HostNotFoundError)
    } else {
        Ok(None)
    }
}

pub fn find_device_by_name(host: &Host, device_name: &str) -> Result<Option<Device>, Error> {
    host.input_devices()
        .map_err(Error::DeviceError)?
        .find(|d| d.name()
            .map_or(false, |name| name.eq_ignore_ascii_case(device_name)))
        .map(|d| Some(d))
        .ok_or(Error::DeviceNotFoundError)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_enumerate_host_names() {
        let mut host_count = 0;
        for _host_name in list_host_names() {
            host_count = host_count + 1;
        }
        assert!(host_count > 0);
    }

    #[test]
    fn test_cam_find_enumerated_hosts_by_id() {
        let mut host_count = 0;
        for host_name in list_host_names() {
            let host_id = find_host_id_by_name(host_name).expect("Unable to find_host_id_by_name");
            let h = cpal::host_from_id(host_id).expect("Unable to host_from_id");
            assert_eq!(host_id, h.id());
            host_count = host_count + 1;
        }
        assert!(host_count > 0);
    }

    #[test]
    fn test_cam_find_enumerated_hosts_by_name() {
        let mut host_count = 0;
        for host_name in list_host_names() {
            let h = find_host_by_name(host_name).unwrap().unwrap();
            assert_eq!(host_name, h.id().name());
            host_count = host_count + 1;
        }
        assert!(host_count > 0);
    }

    #[test]
    fn test_host_not_found_error() {
        let result = find_host_id_by_name("penis");
        assert!(result.is_none());

        let result = find_host_by_name("penis");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        let default_host = cpal::default_host();

        let result = find_device_by_name(&default_host, "penis");

        default_host.id();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_device_not_found_error() {
        let host = cpal::default_host();
        let result = find_device_by_name(&host, "penis");
        assert!(result.is_err());
        assert_eq!(Error::DeviceNotFoundError, result.err().unwrap());
    }

    #[test]
    fn test_cam_find_enumerated_hosts_and_their_devices() {

        let mut host_count = 0;
        let mut device_count = 0;
        for host_name in list_host_names() {

            let host_id = find_host_id_by_name(host_name).expect("Unable to find_host_id_by_name");
            let host = cpal::host_from_id(host_id).expect("Unable to host_from_id");
            assert_eq!(host_id, host.id());

            for d in host.input_devices().expect("Unable to find devices") {

                if !d.name().is_ok() {
                    continue;
                }

                let device_name = d.name().expect("Unable to get name");
                let device = find_device_by_name(&host, &device_name)
                    .expect("Unable to find host and device")
                    .unwrap();

                assert_eq!(d.name().expect("d name"), device.name().expect("device name"));

                device_count = device_count + 1;
            }


            host_count = host_count + 1;
        }
        assert!(host_count > 0);
        assert!(device_count > 0);

    }
}
