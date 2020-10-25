use cpal::{Device, Host, HostId};
use cpal::traits::{DeviceTrait, HostTrait};

pub fn list_host_names() -> impl Iterator<Item = &'static str> {
    return cpal::available_hosts()
        .into_iter()
        .map(|h| h.name())
}

pub fn find_host_id_by_name(name: &str) -> Option<HostId> {
    return cpal::available_hosts()
        .into_iter()
        .find(|h| h.name().eq(name))
}

pub fn find_host_and_device(host_name: &str, device_name: &str) -> Option<(Host, Device)> {
    return find_host_id_by_name(host_name)
        .map(|host_id| cpal::host_from_id(host_id)
            .unwrap())
        .map( |host| host.input_devices()
            .unwrap()
            .find(|d| d.name().unwrap().eq(device_name))
            .map(|d| (host, d)));
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
