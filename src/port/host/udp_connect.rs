use std::net::IpAddr;

// fn timestamp_check(start: &SystemTime, offset_ms: u64) -> bool {
//     let t_stop: Duration = Duration::from_millis(offset_ms);

//     match start.elapsed() {
//         Ok(diff) => diff < t_stop,
//         Err(_) => panic!("TIME ERROR -> ABORT!!!"),
//     }
// }

// fn icmp_is_port_close(
//     ip_a: &IpAddr,
//     pkg_b: Icmpv4Packet,
//     ptr_sock_b: Option<SocketAddrV4>,
//     port_no: u16,
// ) -> bool {
//     // println!("WTF1");
//     match ptr_sock_b {
//         Some(sock_b) => {
//             println!("port {}", sock_b.port());
//             if ip_a != &IpAddr::V4(sock_b.ip().clone()) || port_no != sock_b.port() {
//                 return false;
//             }
//             return pkg_b.typ == 3 && pkg_b.code == 3;
//         }
//         None => false,
//     }
// }

// fn wait_for_icmp(ip: &IpAddr, port_no: u16) -> bool {
//     let mut icmp_socket: socket::IcmpSocket4;

//     match socket::IcmpSocket4::new() {
//         Ok(sock) => {
//             icmp_socket = sock;
//         }
//         Err(_) => {
//             return false;
//         }
//     }
//     if icmp_socket.bind([127, 0, 0, 1]).is_err() {
//         return false;
//     }
//     println!("test reciever");
//     match icmp_socket.rcv_from() {
//         Ok(icmp_info) => {
//             // println!("test reciever");
//             if icmp_is_port_close(ip, icmp_info.0, icmp_info.1.as_socket_ipv4(), port_no) {
//                 println!("Here");
//                 return true;
//             }
//         }
//         Err(_) => {
//             // println!("test reciever");
//             return false;
//         }
//     }
//     return false;
// }

// fn udpsocket_dynamic_binder() -> Result<UdpSocket, ()> {
//     let mut ip_with_port: SocketAddr;

//     for port in 49152..65535 {
//         ip_with_port = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), port);
//         match UdpSocket::bind(ip_with_port) {
//             Ok(udp_socket) => {
//                 return Ok(udp_socket);
//             }
//             Err(_) => {
//                 continue;
//             }
//         }
//     }
//     Err(())
// }

// fn udp_connect(ip: &IpAddr, port_no: u16, socket_ref: &UdpSocket) -> bool {
//     let ip_with_port: SocketAddr = SocketAddr::new(ip.clone(), port_no);
//     let now: SystemTime = SystemTime::now();

//     if socket_ref.connect(ip_with_port).is_err() {
//         return false;
//     }
//     match socket_ref.send(&[0]) {
//         Ok(_) => {}
//         Err(_) => {
//             return false;
//         }
//     }
//     while timestamp_check(&now, 50) {
//         // println!("test thread");
//         if wait_for_icmp(ip, port_no) {
//             println!("Gotcha");
//             return true;
//         }
//     }
//     return false;
// }

pub fn scan(_ip: &IpAddr, _subset: Vec<u16>) -> Vec<u16> {
    // 	let udp_socket: UdpSocket;
    // 	let mut port_res: Vec<u16> = Vec::new();

    // 	println!("test thread");
    // 	match udpsocket_dynamic_binder() {
    // 		Ok(sock) => { udp_socket = sock; }
    // 		Err(_) => { return Vec::new(); }
    // 	}

    // 	println!("test thread");
    // 	for port in subset
    // 	{
    // 		if !udp_connect(ip, port, &udp_socket) {
    // 			port_res.push(port);
    // 		}
    // 	}
    return Vec::new();
}
