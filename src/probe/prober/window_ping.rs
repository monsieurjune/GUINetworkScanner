extern crate pnet;
use std::net::{
    IpAddr,
    Ipv4Addr
};
use rand::{
    thread_rng,
    Rng
};
use pnet::transport::{
    transport_channel,
    TransportChannelType
};
use pnet::packet::{
    ip::IpNextHeaderProtocols,
    ipv4::{
        self,
        Ipv4Packet,
        MutableIpv4Packet,
        Ipv4Flags
    },
    tcp::{
        self,
        MutableTcpPacket,
        TcpFlags
    }
};

const IPV4_HEADER_SIZE: usize = 20;
const TCP_HEADER_SIZE: usize = 20;
const TCP_DATA_SIZE: usize = 0;

fn create_ipv4_header(mut ip_buff: &mut [u8], dst: Ipv4Addr, src: Ipv4Addr)
{
    let mut rng = thread_rng();
    let mut ip_header = MutableIpv4Packet::new(&mut ip_buff).unwrap();

    ip_header.set_version(4);
    ip_header.set_header_length(5);
    ip_header.set_source(src);
    ip_header.set_destination(dst);
    ip_header.set_total_length((IPV4_HEADER_SIZE + TCP_HEADER_SIZE + TCP_DATA_SIZE) as u16);
    ip_header.set_identification(rng.gen());
    ip_header.set_flags(Ipv4Flags::DontFragment);
    ip_header.set_ttl(255);
    ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
    ip_header.set_checksum(ipv4::checksum(&ip_header.to_immutable()));
}

fn create_tcp_syn(
                    ip_buff: &mut [u8], 
                    dst: Ipv4Addr,
                    src: Ipv4Addr,
                    dst_port: u16, 
                    src_port: u16
                )
{
    let mut rng = thread_rng();
    let mut tcp_header = MutableTcpPacket::new(&mut ip_buff[IPV4_HEADER_SIZE..]).unwrap();
    let checksum: u16;

    tcp_header.set_source(src_port);
    tcp_header.set_destination(dst_port);
    tcp_header.set_sequence(rng.gen());
    tcp_header.set_acknowledgement(rng.gen());
    tcp_header.set_reserved(0);
    tcp_header.set_flags(TcpFlags::SYN);
    tcp_header.set_urgent_ptr(0);
    tcp_header.set_window(1024);
    tcp_header.set_data_offset(5);
    checksum = tcp::ipv4_checksum(&tcp_header.to_immutable(), &src, &dst);
    tcp_header.set_checksum(checksum);
} 

fn tcp_ping(dst: Ipv4Addr, src: Ipv4Addr, dst_port: u16, src_port: u16)
{
    let mut buff = [0u8; IPV4_HEADER_SIZE + TCP_HEADER_SIZE + TCP_DATA_SIZE];
    let protocal = IpNextHeaderProtocols::Ipv4;
    let type1 = TransportChannelType::Layer3(protocal);
    let payload: Ipv4Packet<'_>;

    if let Ok((mut tx, mut _rx)) = transport_channel(1024, type1) {
        create_ipv4_header(&mut buff, dst, src);
        create_tcp_syn(&mut buff, dst, src, dst_port, src_port);
        payload = Ipv4Packet::new(&buff).unwrap();
        let _ = tx.send_to(payload, IpAddr::V4(dst));
    } 
}

pub fn ping(dst: Ipv4Addr, src: Ipv4Addr, src_port: u16)
{
    // let mut rng = rand::thread_rng();
    // let port_list: Vec<u16> = (0..3).map(|_| rng.gen_range(0..65535)).collect();

    tcp_ping(dst, src, 135, src_port);
    tcp_ping(dst, src, 445, src_port);

    // for port in port_list {
    //     tcp_ping(dst, src, port, src_port);
    // }
}

