use network_interface::NetworkInterface;
use std::vec::Vec;
mod unix;
mod window;

pub fn parser(interface_info: &Vec<NetworkInterface>) -> Vec<NetworkInterface>
{
    let mut cleaned_info: Vec<NetworkInterface>;

    if cfg!(target_os = "windows")
    {
        cleaned_info = window::remove_loopback(interface_info);
        cleaned_info = window::remove_disconnect(&cleaned_info);
    }
    else
    {
        cleaned_info = unix::remove_loopback(interface_info);
        cleaned_info = unix::remove_disconnect(&cleaned_info);
    }
    return cleaned_info;
}