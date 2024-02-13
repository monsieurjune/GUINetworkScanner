use network_interface::{
    Addr::{
        V4,
        V6
    },
    NetworkInterface,
    NetworkInterfaceConfig,
    V4IfAddr,
    V6IfAddr
};
mod parser;
mod lexer;
extern crate json;

fn main() -> Result<(), network_interface::Error>
{
    let mut info = NetworkInterface::show()?;

    info = parser::parser(&info);
    println!("{:?}", info);
    Ok(())
}
