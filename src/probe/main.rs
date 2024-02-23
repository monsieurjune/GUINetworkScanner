use network_interface::{
    Addr,
    V4IfAddr,
    NetworkInterface,
    NetworkInterfaceConfig
};
use std::{
    process,
    env
};
use std::net::Ipv4Addr;
extern crate json;

fn get_interface(json_input: json::JsonValue, name: &str) -> Option<json::JsonValue>
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

fn main() -> Result<(), std::io::Error>
{
    let json_input: json::JsonValue;
    let interface_info: json::JsonValue;
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 3 {
        process::exit(1);
    }
    json_input = match json::parse(&argv[2]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(254);
        }
    };
    interface_info = match get_interface(json_input, &argv[1]) {
        Some(val) => val,
        None => {
            process::exit(255);
        }
    };
    println!("{}", interface_info);
    Ok(())
}