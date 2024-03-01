use serde_json::from_str;
use std::{env, process};

extern crate json;

mod prober;

fn main() -> Result<(), std::io::Error> {
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
