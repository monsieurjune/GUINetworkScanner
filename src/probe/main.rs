use ipnet::Ipv4AddrRange;
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
use serde::Serialize;
use serde_json::from_str;
use serde_json::to_string_pretty;
extern crate json;
// mod format;
mod prober;

fn main() -> Result<(), std::io::Error>
{
    let argv: Vec<String> = env::args().collect();
    let obj: prober::Prober;

    if argv.len() != 2 {
        process::exit(255);
    }
    obj = match from_str(&argv[1]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    match obj.probe() {
        Ok(val) => {
            println!("{}", val);
        }
        Err(_) => {
            process::exit(255);
        }
    }
    Ok(())
}