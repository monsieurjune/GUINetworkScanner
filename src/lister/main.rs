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
extern crate json;

fn main() {
    let mut info = NetworkInterface::show().unwrap();

    info = parser::parser(&info);
    println!("{:?}", info);
}
