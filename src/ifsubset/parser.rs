use std::str::FromStr;
use ipnet::{
    Ipv4Net,
    Ipv4AddrRange
};
use std::net::Ipv4Addr;
extern crate json;

fn get_interface(json_input: &json::JsonValue, name: &str) -> Option<json::JsonValue>
{
    let len: usize = json_input["length"].as_usize()?;

    for i in 0..len
    {
        if json_input["interface"][i]["name"] == name {
            return Some(json_input["interface"][i].clone());
        }
    }
    None
}

fn json_to_ipv4(json_interface: &json::JsonValue, key: &str) -> Option<Ipv4Addr>
{
    let val: String = json_interface[key].to_string();

    match Ipv4Addr::from_str(&val) {
        Ok(v4) => Some(v4),
        Err(_) => None
    }
}

fn get_ipv4_iter(json_input: &json::JsonValue, name: &str) -> Option<Ipv4AddrRange>
{
    let json_interface: json::JsonValue = get_interface(json_input, name)?;
    let v4_addr: Ipv4Addr = json_to_ipv4(&json_interface, "addr")?;
    let v4_netmask: Ipv4Addr = json_to_ipv4(&json_interface, "netmask")?;

    match Ipv4Net::with_netmask(v4_addr, v4_netmask) {
        Ok(v4if_val) => Some(v4if_val.hosts()),
        Err(_) => None
    }
}

// fn create_host_subset(host: Ipv4Addr)

pub fn get_host_set(json_input: &json::JsonValue, name: &str, subset_no: usize) -> Option<Vec<Vec<Ipv4Addr>>>
{
    let host_iter: Ipv4AddrRange = get_ipv4_iter(json_input, name)?;
    let host_vec: Vec<Ipv4Addr> = host_iter.collect();
    let host_len: usize = host_vec.len();
    let mut host_set: Vec<Vec<Ipv4Addr>> = Vec::new();
    let mut subset: Vec<Ipv4Addr> = Vec::new();

    for i in 0..host_len
    {
        subset.push(host_vec[i]);
        if subset.len() == subset_no {
            host_set.push(subset);
            subset = Vec::new();
        }
    }
    host_set.push(subset);
    Some(host_set)
}