use cpal::{Device, Host, HostId, HostUnavailable};
use cpal::traits::{DeviceTrait, HostTrait};
use crate::errors::Error;

pub fn list_host_names() -> impl Iterator<Item = &'static str> {
    cpal::available_hosts()
        .into_iter()
        .map(|h| h.name())
}

pub fn find_host_id_by_name(name: &str) -> Result<HostId, Error> {
    cpal::available_hosts()
        .into_iter()
        .find(|h| h.name().eq(name))
        .ok_or(Error::HostNotFoundError(HostUnavailable))
}

pub fn find_host_and_device(host_name: &str, device_name: &str) -> Result<(Host, Device), Error> {
    let host_id = find_host_id_by_name(host_name)?;
    let host = cpal::host_from_id(host_id)?;
    host.input_devices()?
        .filter(|d| d.name().is_ok())
        .find(|d| d.name().unwrap().eq(device_name))
        .map(|d| (host, d))
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
    fn test_cam_find_enumerated_hosts() {
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
    fn test_host_not_found_error() {
        let result = find_host_id_by_name("penis");
        assert!(result.is_err());
        assert_eq!(Error::HostNotFoundError(HostUnavailable), result.err().unwrap());

        let result = find_host_and_device("ass fucker", "penis");
        assert_eq!(Error::HostNotFoundError(HostUnavailable), result.err().unwrap());
    }

    #[test]
    fn test_device_not_found_error() {
        let host_name = list_host_names().find(|_d| true).expect("first host_name");
        let result = find_host_and_device(host_name, "penis");
        assert!(result.is_err());
        assert_eq!(Error::DeviceNotFoundError, result.err().unwrap());
    }

    #[test]
    fn test_cam_find_enumerated_hosts_and_their_devices() {

        let mut host_count = 0;
        let mut device_count = 0;
        for host_name in list_host_names() {

            let host_id = find_host_id_by_name(host_name).expect("Unable to find_host_id_by_name");
            let h = cpal::host_from_id(host_id).expect("Unable to host_from_id");
            assert_eq!(host_id, h.id());

            for d in h.input_devices().expect("Unable to find devices") {

                if !d.name().is_ok() {
                    continue;
                }

                let device_name = d.name().expect("Unable to get name");
                let (host, device) = find_host_and_device(host_name, &device_name)
                    .expect("Unable to find host and device");

                assert_eq!(d.name().expect("d name"), device.name().expect("device name"));
                assert_eq!(h.id(), host.id());

                device_count = device_count + 1;
            }


            host_count = host_count + 1;
        }
        assert!(host_count > 0);
        assert!(device_count > 0);

    }
}