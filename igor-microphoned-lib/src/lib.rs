use cpal::{Device, Host, HostId, DevicesError, HostUnavailable};
use cpal::traits::{DeviceTrait, HostTrait};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error(transparent)]
    DeviceError(#[from] DevicesError),
    #[error("host not found")]
    DeviceNotFoundError,
    #[error("host not found")]
    HostNotFoundError(#[from] HostUnavailable),
}

pub fn list_host_names() -> impl Iterator<Item = &'static str> {
    return cpal::available_hosts()
        .into_iter()
        .map(|h| h.name())
}

pub fn find_host_id_by_name(name: &str) -> Result<HostId, AudioError> {
    return cpal::available_hosts()
        .into_iter()
        .find(|h| h.name().eq(name))
        .ok_or(AudioError::HostNotFoundError(HostUnavailable))
}

pub fn find_host_and_device(host_name: &str, device_name: &str) -> Result<(Host, Device), AudioError> {
    let host_id = find_host_id_by_name(host_name)?;
    let host = cpal::host_from_id(host_id)?;
    return host.input_devices()?
        .find(|d| d.name().ok().unwrap().eq(device_name))
        .map(|d| (host, d))
        .ok_or(AudioError::DeviceNotFoundError);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_find_those_iterated() {

        let mut x = 0;
        for name in list_host_names() {
            println!("Found host name: {}", name);
            let host_id = find_host_id_by_name(name).expect("Unable to find_host_id_by_name");
            let host = cpal::host_from_id(host_id).expect("Unable to host_from_id");
            assert_eq!(host_id, host.id());
            x = x + 1;
        }
        assert!(x > 0)

    }
}
