use network_interface::{
    Addr,
    V4IfAddr,
    NetworkInterface,
    NetworkInterfaceConfig
};
use std::process;
use std::net::Ipv4Addr;
mod parser;
extern crate json;

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

fn ipv4_to_string(ipv4: Option<Ipv4Addr>) -> String
{
    let mut ip_str: String = String::new();
    let binary: [u8; 4] = ipv4.unwrap().octets();

    for i in 0..4
    {
        ip_str.push_str(&binary[i].to_string());
        if i != 3 {
            ip_str.push('.');
        }
    }
    return ip_str;
}

fn json_array_wrapper(interface: &NetworkInterface, i: usize) -> Option<String>
{
    let addr: V4IfAddr;
    let mac_addr: String;

    mac_addr = match interface.mac_addr.clone() {
        Some(m) => m.to_uppercase(),
        None => String::from("00:00:00:00:00:00")
    };
    match interface.addr[0] {
        Addr::V4(v4) => { addr = v4 },
        Addr::V6(_) => { return None; }
    };
    Some(
        json::stringify_pretty (
            json::object! {
                index: i,
                name: interface.name.clone(),
                addr: ipv4_to_string(Some(addr.ip)),
                broadcast: ipv4_to_string(addr.broadcast),
                netmask: ipv4_to_string(addr.netmask),
                mac: mac_addr
            },
            4
        )
    )
}

fn create_json_array(interface_info: &Vec<NetworkInterface>) -> (String, usize)
{
    let length: usize = interface_info.len();
    let mut final_len: usize = 0;
    let mut json_array: String = String::from("[");

    for i in 0..length
    {
        match json_array_wrapper(&interface_info[i], i) {
            Some(val) => {
                json_array.push_str(&val);
                if i < interface_info.len() - 1 {
                    json_array.push(',');
                }
                final_len += 1;
            }
            None => {}
        }
    }
    json_array.push(']');
    return (json_array, final_len);
}

fn create_json(interface_info: Vec<NetworkInterface>) -> Result<String, json::Error>
{
    let json_array_info: (String, usize);
    let json_array: json::JsonValue;
    let json_final: json::JsonValue;

    json_array_info = create_json_array(&interface_info);
    json_array = json::parse(&json_array_info.0)?;
    json_final = json::object! {
        length: json_array_info.1,
        interface: json_array
    };
    return Ok(
        json::stringify_pretty(json_final, 4)
    );
}

fn main() -> Result<(), std::io::Error>
{
    let mut info: Vec<NetworkInterface>;
    let json_info: String;

    info = match NetworkInterface::show() {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };

    info = parser::parser(&info);
    json_info = match create_json(info) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255)
        }
    };
    println!("{}", json_info);
    Ok(())
}
