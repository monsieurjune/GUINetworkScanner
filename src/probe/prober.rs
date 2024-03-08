use serde::{
    Deserialize,
    Serialize
};
use std::thread::{
    Builder, 
    JoinHandle
};
use serde_json::to_string;
use std::process;

extern crate pnet;
use std::net::{IpAddr, Ipv4Addr, TcpListener};
use rand::{thread_rng, Rng};
use pnet::transport::{
    ipv4_packet_iter, tcp_packet_iter, transport_channel, TransportChannelType
};
use pnet::packet::{
    ip::{
        IpNextHeaderProtocol,
        IpNextHeaderProtocols
    },
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

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

mod icmp_ping;
mod rand_ping;
mod window_ping;

#[derive(Serialize, Deserialize)]
pub struct Prober {
    name: String,
    addr_set: Vec<Ipv4Addr>,
}

impl Prober {
    fn thread_builder(
        &self,
        addr: Ipv4Addr,
        thread_id: usize,
    ) -> JoinHandle<Option<Ipv4Addr>> {
        let builder: Builder = Builder::new()
            .name(thread_id.to_string())
            .stack_size(32 * 1024);

        match builder.spawn(move || {
            window_ping::ping(addr)
        }) {
            Ok(handler) => handler,
            Err(e) => {
                eprintln!("Thread Allocation failed due to {:?}", e);
                process::exit(12);
            }
        }
    }

    fn join_result(handler: JoinHandle<Option<Ipv4Addr>>) -> Option<Ipv4Addr>
    {
        match handler.join() {
            Ok(val) => val,
            Err(_) => None
        }
    }

    fn thread_joiner(handler_list: Vec<JoinHandle<Option<Ipv4Addr>>>) -> Vec<Ipv4Addr>
    {
        let mut result_vec: Vec<Ipv4Addr> = Vec::with_capacity(256);

        for handler in handler_list {
            match Prober::join_result(handler) {
                Some(val) => {
                    result_vec.push(val);
                }
                None => {}
            }
        }
        result_vec
    }

    fn find_match_interface(inter_addr: Ipv4Addr) -> Result<NetworkInterface, ()>
    {
        let interfaces: Vec<NetworkInterface> = datalink::interfaces();
        let interface_names_match =
            |iface: &NetworkInterface| iface.ips[0].contains(IpAddr::V4(inter_addr));
        
        match interfaces
                .into_iter()
                .filter(interface_names_match)
                .next() {
            Some(val) => Ok(val),
            None => Err(())
        }
    }

    pub fn probe(&self, inter_addr: Ipv4Addr) -> Result<String, ()> {
        let length: usize = self.addr_set.len();
        let result_list: Vec<Ipv4Addr>;
        let prober_res: Prober;
        let mut handler: JoinHandle<Option<Ipv4Addr>>;
        let mut handler_list: Vec<JoinHandle<Option<Ipv4Addr>>> = Vec::new();
        let interface: NetworkInterface = Prober::find_match_interface(inter_addr)?;

        let (mut e_tx, mut e_rx) = match datalink::channel(&interface, Default::default()){
            Ok(Ethernet(tx, rx)) => Ok((tx, rx)),
            Ok(_) => Err(()),
            Err(_) => Err(())
        }?;

        for i in 0..length {
            handler = Prober::thread_builder(&self, self.addr_set[i].clone(), i);
            handler_list.push(handler)
        }
        result_list = Prober::thread_joiner(handler_list);
        prober_res = Prober {
            name: self.name.clone(),
            addr_set: result_list
        };
        match to_string(&prober_res) {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }
}
