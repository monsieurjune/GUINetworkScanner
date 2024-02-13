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
mod parser;
extern crate json;

fn remove_windows(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface> {
    let mut cleaned_info1: Vec<NetworkInterface> = Vec::new();
    let mut cleaned_info2: Vec<NetworkInterface> = Vec::new();

    for interface in interface_info {
        match interface.name.contains("Bluetooth") || interface.name.contains("Loopback") {
            true => {}
            false => {
                cleaned_info1.push(interface.clone());
            }
        }
    }

    for interface in cleaned_info1 {
        match interface.addr[0] {
            V4(v4) => match v4.broadcast {
                Some(_) => {
                    cleaned_info2.push(interface.clone());
                }
                None => {}
            },
            V6(_) => {}
        }
    }

    cleaned_info2
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

    cleaned_info
}

fn main() {
    let mut info = NetworkInterface::show().unwrap();

    info = parser::parser(&info);
    println!("{:?}", info);
}
