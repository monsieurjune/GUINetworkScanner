use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use std::io::ErrorKind::ConnectionRefused;

pub fn scan(ip: &IpAddr, subset: Vec<u16>) -> Vec<(u16, String)> {
    let mut ip_with_port: SocketAddr;
    let mut port_status: Vec<(u16, String)>  = Vec::new();
    let time: Duration = Duration::new(0, 040_000_000);

    for port in subset {
        ip_with_port = SocketAddr::new(ip.clone(), port);
        match TcpStream::connect_timeout(&ip_with_port, time) {
            Ok(mut _sock) => {
                port_status.push((port, String::from("Open")));
            }
            Err(ref err) if err.kind() == ConnectionRefused => {}
            Err(_) => {
                // port_status.push((port, String::from("Filtered")))
            }
        }
    }

    return port_status;
}
