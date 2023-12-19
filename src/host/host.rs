// STD module
use std::net::{
    IpAddr,
    TcpStream
};
use std::str::FromStr;

// USER module
mod scanmode;

pub struct Host
{
    ipv4: IpAddr,
    mode: scanmode::ScanMode
}

impl Host
{
    pub fn new()
}

// struct Host
// {
//     ipv4: IpAddr,
//     start_port: u16,
//     end_port: u16
// }

// pub struct TcpHost
// {
//     data: Host
// }


// fn host_new(ipv4_str: &String, set_mode:bool) -> Result<Host, ()>
// {
//     let start: u16 = 1;
//     let end: u16 = if set_mode {1023} else {65535};

//     match IpAddr::from_str(ipv4_str)
//     {
//         Ok(ipv4_res) =>
//         {
//             return Ok
//             (
//                 Host
//                 {
//                     ipv4:ipv4_res,
//                     start_port:start,
//                     end_port:end
//                 }
//             );
//         }
//         Err(_) =>
//         {
//             return Err(());
//         }
//     }
// }

// impl TcpHost
// {
//     pub fn new(ipv4_str: &String, set_mode:bool) -> Result<TcpHost, ()>
//     {
//         match host_new(ipv4_str, set_mode)
//         {
//             Ok(tcphost) =>
//             {
//                 return Ok(TcpHost{ data:tcphost });
//             }
//             Err(_) =>
//             {
//                 return Err(());
//             }
//         }
//     }

//     pub fn scan(&self)
//     {
//     }
// }
