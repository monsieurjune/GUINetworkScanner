use network_interface::{
    NetworkInterface,
    NetworkInterfaceConfig,
};
mod parser;
extern crate json;

fn create_json(interface_info: Vec<NetworkInterface>) -> String
{
    let init: json::JsonValue = json::object!(

    );
}

fn main() -> Result<(), network_interface::Error>
{
    let mut info: Vec<NetworkInterface> = NetworkInterface::show()?;

    info = parser::parser(&info);
    Ok(())
}
