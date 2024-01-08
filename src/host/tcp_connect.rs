use std::net::{
	IpAddr,
	TcpStream,
	SocketAddr,
};
use std::time::Duration;

pub fn scan(ip: &IpAddr, subset: Vec<u16>) -> Vec<u16>
{
	let mut ip_with_port: SocketAddr;
	let mut port_status: Vec<u16> = Vec::new();
	let time: Duration = Duration::new(0, 030_000_000);

	for port in subset
	{
		ip_with_port = SocketAddr::new(ip.clone(), port);
		match TcpStream::connect_timeout(&ip_with_port, time) {
			Ok(_) => port_status.push(port),
			Err(_) => {}
		}
	}
	return port_status;
}