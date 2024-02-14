use network_interface::{
    NetworkInterface,
    NetworkInterfaceConfig,
};
mod parser;

fn main() -> Result<(), network_interface::Error>
{
    let mut info = NetworkInterface::show()?;

    info = parser::parser(&info);

    Ok(())
}
