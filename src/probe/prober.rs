use pnet::packet::tcp::TcpPacket;
use serde::{
    Deserialize,
    Serialize
};
use std::thread::{
    Builder, 
    JoinHandle
};
use std::time::Duration;
use serde_json::to_string;
use std::process;

extern crate pnet;
use std::net::{IpAddr, Ipv4Addr, TcpListener};
use rand::{thread_rng, Rng};
use pnet::transport::{
    ipv4_packet_iter, tcp_packet_iter, transport_channel, TransportChannelType, TransportReceiver, TransportSender
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
use std::time::SystemTime;

use std::sync::mpsc::{
    self, channel, Receiver, Sender
};
use pnet::datalink::{self, DataLinkReceiver, NetworkInterface};
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
    fn thread_builder(&self, dst: Ipv4Addr, src: Ipv4Addr, src_port: u16) -> JoinHandle<()> 
    {
        let builder: Builder = Builder::new()
            .stack_size(64 * 1024);

        match builder.spawn(move || {
            window_ping::ping(dst, src, src_port);
        }) 
        {
            Ok(handler) => handler,
            Err(_) => {
                process::exit(12);
            }
        }
    }

    fn thread_joiner(handler_list: Vec<JoinHandle<()>>)
    {
        for handler in handler_list {
            let _ = handler.join();
        }
    }

    fn get_probe_result(mut e_rx: Box<dyn DataLinkReceiver>, 
                        input_subset: Vec<Ipv4Addr>, 
                        res_t: Sender<Vec<Ipv4Addr>>,
                    ) -> Result<JoinHandle<()>, ()>
    {
        let builder: Builder = Builder::new().stack_size(32 * 1024);

        match builder.spawn(
            move || {
                let mut result: Vec<Ipv4Addr> = Vec::new();
                let start = SystemTime::now();
                let mut now = SystemTime::now();
                while now.duration_since(start).unwrap().as_millis() <= 1000 {
                    match e_rx.next() {
                        Ok(ether_packet) => {
                            let packet = EthernetPacket::new(ether_packet).unwrap();
                            let mut ip_pack = packet.payload().to_owned();
                            let ip_pack1 = Ipv4Packet::new(&mut ip_pack).unwrap();
                            let source_addr = ip_pack1.get_source();
                            if input_subset.contains(&source_addr) {
                                result.push(source_addr);
                            }
                        }
                        Err(_) => {}
                    }
                    now = SystemTime::now();
                }
                let _ = res_t.send(result);
            }
        ) {
            Ok(handler) => Ok(handler),
            Err(_) => Err(())
        }
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

    fn probe_to_json(name: String, addr_set: Vec<Ipv4Addr>) -> Result<String, ()>
    {
        let res: Prober = Prober {
            name,
            addr_set
        };
        match to_string(&res) {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    pub fn probe(&self, inter_addr: Ipv4Addr) -> Result<String, ()> {
        let result_list: Vec<Ipv4Addr>;
        let reciever_handler: JoinHandle<()>;
        let mut handler: JoinHandle<()>;
        let mut handler_list: Vec<JoinHandle<()>> = Vec::new();
        let interface: NetworkInterface = Prober::find_match_interface(inter_addr)?;
        let (res_t, res_r) = channel();
        let mut rng = thread_rng();

        let (_, e_rx) = match datalink::channel(&interface, Default::default()){
            Ok(Ethernet(tx, rx)) => Ok((tx, rx)),
            _ => Err(())
        }?;

        reciever_handler = Prober::get_probe_result(e_rx, self.addr_set.clone(), res_t.clone())?;
        for host in &self.addr_set {
            handler = Prober::thread_builder(&self, host.clone(), inter_addr.clone(),rng.gen_range(49152..=65535));
            handler_list.push(handler);
        }
        Prober::thread_joiner(handler_list);
        let _ = reciever_handler.join();
        result_list = match res_r.recv() {
            Ok(mut val) => {
                val.sort();
                val.dedup();
                Ok(val)
            },
            Err(_) => Err(())
        }?;
        Prober::probe_to_json(self.name.clone(), result_list)
    }
}
