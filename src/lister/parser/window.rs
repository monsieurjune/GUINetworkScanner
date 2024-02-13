use network_interface::NetworkInterface;
use std::vec::Vec;

pub fn remove_loopback(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface>
{
    let mut cleaned_info: Vec<NetworkInterface> = Vec::new();

    for interface in interface_info {
        match interface.name.contains("Bluetooth") || interface.name.contains("Loopback")
        {
            true => {}
            false => {
                cleaned_info.push(interface.clone());
            }
        }
    }
    return cleaned_info;
}

pub fn remove_disconnect(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface>
{
    let mut cleaned_info: Vec<NetworkInterface> = Vec::new();

    for interface in interface_info {
        
    }
    return cleaned_info;
}