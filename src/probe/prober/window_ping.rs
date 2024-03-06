use std::time::Duration;
use std::net::{
    TcpStream,
    SocketAddr,
    IpAddr,
    Ipv4Addr
};
use std::io::ErrorKind::ConnectionRefused;

fn msrpc_ping(ipaddr: Ipv4Addr) -> bool
{
    let sockaddr: SocketAddr = SocketAddr::new(IpAddr::V4(ipaddr), 135);
	let time: Duration = Duration::new(0, 100_000_000);

    match TcpStream::connect_timeout(&sockaddr, time) {
        Ok(_) => true,
        Err(ref e) if e.kind() == ConnectionRefused => true,
        Err(_) => false
    }
}

fn msds_ping(ipaddr: Ipv4Addr) -> bool
{
    let sockaddr: SocketAddr = SocketAddr::new(IpAddr::V4(ipaddr), 445);
	let time: Duration = Duration::new(0, 100_000_000);

    match TcpStream::connect_timeout(&sockaddr, time) {
        Ok(_) => true,
        Err(ref e) if e.kind() == ConnectionRefused => true,
        Err(_) => false
    }
}

pub fn ping(ipaddr: Ipv4Addr) -> Option<Ipv4Addr>
{
    if msrpc_ping(ipaddr.clone()) {
        return Some(ipaddr);
    };
    if msds_ping(ipaddr.clone()) {
        return Some(ipaddr);
    }
    None
}

