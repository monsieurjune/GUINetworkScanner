use network_interface::{
    Addr::{
        V4,
        V6
    },
    NetworkInterface,
    NetworkInterfaceConfig,
    V4IfAddr,
    V6IfAddr
};
use std::vec::Vec;

pub fn remove_loopback(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface>
{
    let mut cleaned_info: Vec<NetworkInterface> = Vec::new();

    for interface in interface_info {
        match interface.name.find("lo") {
            Some(pos) => {
                if pos != 0 {
                    cleaned_info.push(interface.clone());
                }
            }
            None => {
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
        if interface.addr.len() >= 1 {
            cleaned_info.push(interface.clone());
        }
    }
    return cleaned_info;
}