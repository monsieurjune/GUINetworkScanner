use std::{
    process,
    env
};
use std::net::Ipv4Addr;
use std::str::FromStr;
mod parser;
mod format;

fn argv_check(argv: &Vec<String>) -> Result<(json::JsonValue, usize), ()>
{
    let json_input: json::JsonValue;

    if argv.len() != 4 {
        return Err(());
    };
    json_input = match json::parse(&argv[1]) {
        Ok(json_val) => json_val,
        Err(_) => {
            return Err(());
        }
    };
    match usize::from_str(&argv[3]) {
        Ok(val) => Ok((json_input, val)),
        Err(_) => Err(())
    }
}

// ifsubset [interfaces's json] [specific interface's name] [amout of subset's member]
// EXAMPLE
// ifsubset (interfaces json variable) wlan0 256

fn main() -> Result<(), std::io::Error>
{
    let argv: Vec<String> = env::args().collect();
    let host_set: Vec<Vec<Ipv4Addr>>;
    let json_input: json::JsonValue;
    let subset_no: usize;

    match argv_check(&argv) {
        Ok(val) => {
            json_input = val.0;
            subset_no = val.1;
        }
        Err(_) => {
            process::exit(255);
        }
    };
    host_set = match parser::get_host_set(&json_input, &argv[2], subset_no) {
        Some(val) => val,
        None => {
            process::exit(255);
        }
    };
    println!("{}", format::host_set_to_json(host_set, &argv[2]));
    Ok(())
}