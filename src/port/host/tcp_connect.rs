use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

pub fn scan(ip: &IpAddr, subset: Vec<u16>) -> Vec<(u16, String, String)> {
    let mut ip_with_port: SocketAddr;
    let mut port_status: Vec<(u16, String, String)>  = Vec::new();
    let time: Duration = Duration::new(0, 040_000_000);
    let mut recv_str: String = String::new();

    for port in subset {
        ip_with_port = SocketAddr::new(ip.clone(), port);
        match TcpStream::connect_timeout(&ip_with_port, time.clone()) {
            Ok(mut _sock) => {
                let _ = _sock.set_read_timeout(Some(time.clone()));
                let _ = _sock.write_fmt(format_args!("3hbrbcod3423"));
                match _sock.read_to_string(&mut recv_str) {
                    Ok(_) => {
                        port_status.push((port, String::from("Open"), recv_str.clone()));
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
    port_status
}
