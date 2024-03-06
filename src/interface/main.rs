use network_interface::{
    NetworkInterface,
    NetworkInterfaceConfig
};
use std::process;
mod parser;
mod interface;

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
    json_info = match interface::create_json(info) {
        Ok(val) => val,
        Err(_) => {
            process::exit(255);
        }
    };
    println!("{}", json_info);
    Ok(())
}
