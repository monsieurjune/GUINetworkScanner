#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use local_ip_address::{list_afinet_netifas, local_ip};
use network_interface::{
    Addr::{V4, V6},
    NetworkInterface, NetworkInterfaceConfig, V4IfAddr, V6IfAddr,
};
extern crate json;

fn remove_windows(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface> {
    let mut cleaned_info: Vec<NetworkInterface> = Vec::new();

    for interface in interface_info {
        match interface.name.contains("Bluetooth") || interface.name.contains("Loopback") {
            true => {}
            false => {
                cleaned_info.push(interface.clone());
            }
        }
    }

    return cleaned_info;
}

// fn get_wlan(info: Vec<NetworkInterface>) -> NetworkInterface
fn remove_loopback(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface> {
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

fn main() {
    let mut info = NetworkInterface::show().unwrap();
    // println!("{:?}", info);
    let info2 = remove_windows(&info);
    println!("{:?}", info2);

    let info1 = info.pop().unwrap();
    let addr = &info2[0].addr;

    match addr[0] {
        V4(v4) => {
            println!("{:?}", v4);
            println!("{:?}", v4.netmask.unwrap());
        }
        V6(v6) => {
            println!("{:?}", v6);
        }
    };
    // let parsed = json::parse(info1.to_string());
    // println!("{:?}", addr_v4);
}
