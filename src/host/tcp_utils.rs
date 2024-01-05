use std::vec::Vec;
use std::collections::HashMap;
use std::net::{
	IpAddr,
	TcpStream
};

pub fn create_map() -> HashMap<String, (u16, u16, Option<Vec<u16>>)>
{
	HashMap::from([
		("full".to_string(), (0, 65535, None)),
		("full".to_string(), (0, 65535, None)),
	])
}

pub fn scan(ip: &IpAddr, subset: Vec<u16>) -> Vec<u16>
{
	let ip_with_prefix: String = ip.to_string() + ":";
	let mut port_status: Vec<u16> = Vec::new();
	let mut ip_with_port: String;

	for port in subset
	{
		ip_with_port = ip_with_prefix.clone() + &port.to_string();
		match TcpStream::connect(ip_with_port) {
			Ok(_) => { port_status.push(port); }
			Err(_) => {}
		}
	}
	return port_status;
}