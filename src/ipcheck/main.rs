use std::net::{
	IpAddr,
	TcpStream,
	SocketAddr,
};
use std::io::ErrorKind::ConnectionRefused;
use std::time::Duration;
use std::str::FromStr;

fn microsoft_tcp_ping(addr: &IpAddr) -> bool
{
    let msrpc_port: SocketAddr = SocketAddr::new(addr.clone(), 135);
    let time: Duration = Duration::new(0, 030_000_000);

    match TcpStream::connect_timeout(&msrpc_port, time) {
        Ok(_) => true,
        Err(ref e) if e.kind() == ConnectionRefused => true,
        Err(_) => false
    }
}

fn main()
{
    let addr = IpAddr::from_str("127.0.0.1").unwrap();
}