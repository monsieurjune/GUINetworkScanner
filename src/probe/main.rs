use serde_json::from_str;
use std::{env, net::Ipv4Addr, process, str::FromStr};

mod prober;

fn main() -> Result<(), std::io::Error> {
    let argv: Vec<String> = env::args().collect();
    let inter_addr: Ipv4Addr;
    let obj: prober::Prober;

    if argv.len() != 3 {
        process::exit(255);
    }
    obj = match from_str(&argv[1]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    inter_addr = match Ipv4Addr::from_str(&argv[2]) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    match obj.probe(inter_addr) {
        Ok(val) => {
            println!("{}", val);
        }
        Err(_) => {
            process::exit(255);
        }
    }
    Ok(())
}
