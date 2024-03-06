use std::net::Ipv4Addr;
use network_interface::{
    Addr,
    NetworkInterface,
    V4IfAddr
};
use serde::{
    Serialize,
    Deserialize
};
use serde_json::to_string;

/*
    JSON FORMAT

    {
    "length": 2, <- length
    "interface": [ <- array
        {
            "index": 12,
            "name": "WLAN",
            "addr": "10.18.7.232",
            "broadcast": "10.18.15.255",
            "netmask": "255.255.240.0",
            "mac": "FA:52:B4:35:DC:55"
        },
        {
            "index": 37,
            "name": "vEthernet (Default Switch)",
            "addr": "172.27.64.1",
            "broadcast": "172.27.79.255",
            "netmask": "255.255.240.0",
            "mac": "00:15:5D:0F:B4:00"
        }
    ]
}
*/

#[derive(Serialize, Deserialize)]
struct Interface
{
    index: usize,
    name: String,
    addr: Ipv4Addr,
    broadcast: Ipv4Addr,
    netmask: Ipv4Addr,
    mac: String
}

#[derive(Serialize, Deserialize)]
struct InterfaceInfo
{
    length: usize,
    interface: Vec<Interface>
}

fn interface_wrapper(interface: &NetworkInterface, i: usize) -> Option<Interface>
{
    let mac_addr: String = match interface.mac_addr.clone() {
        Some(val) => val.to_uppercase(),
        None => String::from("(unknown)")
    };
    let addr: V4IfAddr = match interface.addr[0] {
        Addr::V4(v4) => v4,
        Addr::V6(_) => {
            return None;
        }
    };
    Some(
        Interface {
            index: i,
            name: interface.name.clone(),
            addr: addr.ip,
            broadcast: addr.broadcast?,
            netmask: addr.netmask?,
            mac: mac_addr
        }
    )
}

fn create_json_array(interface_info: Vec<NetworkInterface>) -> Vec<Interface>
{
    let mut i: usize = 0;
    let mut interface_vec: Vec<Interface> = Vec::new();

    for interface in interface_info {
        match interface_wrapper(&interface, i) {
            Some(val) => {
                interface_vec.push(val);
                i += 1;
            }
            None => {}
        }
    }
    interface_vec
}

pub fn create_json(interface_info: Vec<NetworkInterface>) -> Result<String, serde_json::Error>
{
    let intf_vec: Vec<Interface> = create_json_array(interface_info);
    let intf: InterfaceInfo = InterfaceInfo {
        length: intf_vec.len(),
        interface: intf_vec
    };
    to_string(&intf)
}