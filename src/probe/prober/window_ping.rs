use std::time::Duration;
use std::sync::mpsc::Sender;
use std::net::{
    TcpStream,
    SocketAddr,
    IpAddr,
    Ipv4Addr
};
use std::io::ErrorKind::ConnectionRefused;

fn tx_handler(tx: &Sender<Ipv4Addr>, ipaddr: Ipv4Addr) -> bool
{
    match tx.send(ipaddr) {
        Ok(_) => true,
        Err(_) => false
    }
}

fn msrpc_ping(sockaddr: &SocketAddr) -> bool
{
	let time: Duration = Duration::new(0, 030_000_000);

    match TcpStream::connect_timeout(sockaddr, time) {
        Ok(_) => true,
        Err(ref e) if e.kind() == ConnectionRefused => true,
        Err(_) => false
    }
}

fn msds_ping(sockaddr: &SocketAddr) -> bool
{
	let time: Duration = Duration::new(0, 030_000_000);

    match TcpStream::connect_timeout(sockaddr, time) {
        Ok(_) => true,
        Err(ref e) if e.kind() == ConnectionRefused => true,
        Err(_) => false
    }
}

pub fn ping(ipaddr: Ipv4Addr, tx: &Sender<Ipv4Addr>) -> bool
{
    let ip_with_port: SocketAddr = SocketAddr::new(IpAddr::V4(ipaddr), 135);

    if msrpc_ping(&ip_with_port) {
        return tx_handler(&tx, ipaddr)
    };
    if msds_ping(&ip_with_port) {
        return tx_handler(&tx, ipaddr);
    }
    false
}

