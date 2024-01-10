use icmp_socket::IcmpSocket;
use icmp_socket::{
	socket,
	Icmpv4Packet
};
use std::net::{
	IpAddr,
	SocketAddr,
	SocketAddrV4,
	UdpSocket
};
use std::time::{
	SystemTime,
	Duration
};
use std::str::FromStr;

fn timestamp_check(start: &SystemTime, offset_ms: u64) -> bool
{
	let t_stop: Duration = Duration::from_millis(offset_ms);

	match start.elapsed()
	{
		Ok(diff) => diff >= t_stop,
		Err(_) => panic!("TIME ERROR -> ABORT!!!")
	}
}

fn icmp_is_port_close(ip_a: &IpAddr, pkg_b: Icmpv4Packet, ptr_sock_b: Option<SocketAddrV4>) -> bool
{
	match ptr_sock_b {
		Some(sock_b) => {
			if ip_a != &IpAddr::V4(sock_b.ip().clone()) {
				return false;
			}
			return pkg_b.typ == 3 && pkg_b.code == 3;
		}
		None => false
	}
}

fn wait_for_icmp(ip: &IpAddr) -> bool
{
	let mut icmp_socket: socket::IcmpSocket4;

	match socket::IcmpSocket4::new() {
		Ok(sock) => { icmp_socket = sock; }
		Err(_) => { return false; }
	}
	if icmp_socket.bind([127, 0, 0, 1]).is_err() {
		return false;
	}
	match icmp_socket.rcv_from() {
		Ok(icmp_info) => {
			if icmp_is_port_close(ip, icmp_info.0, icmp_info.1.as_socket_ipv4()) {
				return true;
			}
		}
		Err(_) => {
			return false;
		}
	}
	return false;
}

fn udpsocket_dynamic_binder() -> Result<UdpSocket, ()>
{
	let mut ip_with_port: SocketAddr;

	for port in 49152..65535
	{
		ip_with_port = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), port);
		match UdpSocket::bind(ip_with_port) {
			Ok(udp_socket) => {
				return Ok(udp_socket);
			}
			Err(_) => { continue; }
		}
	}
	Err(())
}

fn udp_connect(ip: &IpAddr, port_no: u16, socket_ref: &UdpSocket) -> bool
{
	let ip_with_port: SocketAddr = SocketAddr::new(ip.clone(), port_no);
	let now: SystemTime = SystemTime::now();

	if socket_ref.connect(ip_with_port).is_err() {
		return false;
	}
	match socket_ref.send(&[0]) {
		Ok(_) => {}
		Err(_) => { return false; }
	}
	while timestamp_check(&now, 50)
	{
		if wait_for_icmp(ip) {
			return true;
		}
	}
	return false;
}

pub fn scan(ip: &IpAddr, subset: Vec<u16>) -> Vec<u16>
{
	let udp_socket: UdpSocket;
	let mut port_res: Vec<u16> = Vec::new();

	match udpsocket_dynamic_binder() {
		Ok(sock) => { udp_socket = sock; }
		Err(_) => { return Vec::new(); }
	}

	for port in subset
	{
		if !udp_connect(ip, port, &udp_socket) {
			port_res.push(port);
		}
	}
	return port_res;
}