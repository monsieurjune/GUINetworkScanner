use std::env;
use std::process;
use std::net::Ipv4Addr;
use std::str::FromStr;
use host::Host;
mod host;

// port tcp fast 192.168.1.1 interface

fn main() -> std::io::Result<()> {
    // let list: Vec<(String, String, String)>;
    let argv: Vec<String> = env::args().collect();
    let host: Host;
    let inter_addr: Ipv4Addr;
    let host_ip: Ipv4Addr;
    // let mut host_list: Vec<Host> = Vec::new();

    if argv.len() != 5 {
        process::exit(255)
    }
    host_ip = match Ipv4Addr::from_str(&argv[3]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    inter_addr = match Ipv4Addr::from_str(&argv[4]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    host = match Host::new(host_ip, inter_addr, &argv[1], &argv[2]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    if let Ok(res) = host.tcp_connect_scan() {
        if let Some(res1) = res {
            println!("{}", res1);
        }
    }
    Ok(())
}
