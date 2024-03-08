mod map;
mod scanmode;
mod tcp_syn;

use self::map::Map;
use lazy_static::lazy_static;
use pnet::packet::tcp::{TcpFlags, TcpPacket};
use scanmode::ScanMode;
use serde::{Deserialize, Serialize};
use std::net::{
    IpAddr,
    Ipv4Addr
};
use rand::{
    thread_rng, 
    Rng
};
use std::sync::mpsc::{
    channel, 
    Sender
};
use pnet::datalink::{
    self, 
    DataLinkReceiver, 
    NetworkInterface
};
use std::time::SystemTime;
use std::process;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{
    Packet,
    ipv4::Ipv4Packet
};
use pnet::packet::ethernet::EthernetPacket;
use std::thread::{
	Builder,
	JoinHandle
};

#[allow(dead_code)]
pub struct Host {
    ip: Ipv4Addr,
    inter_ip: Ipv4Addr,
    tcp_mode: Option<ScanMode>,
    udp_mode: Option<ScanMode>,
}

#[derive(Serialize, Deserialize)]
pub struct ScanResultInfo {
    port: u16,
    status: String
}

#[derive(Serialize, Deserialize)]
pub struct ScanResult {
    ipaddr: Ipv4Addr,
    tcp_ports: Vec<ScanResultInfo>,
}

impl Host {
    pub fn new(host_ip: Ipv4Addr, inter_ip: Ipv4Addr, mode: &str, choice: &str) -> Result<Host, ()> 
    {
        let tcp_mode: Option<ScanMode>;
        let udp_mode: Option<ScanMode>;

        lazy_static! {
            static ref TCP_MAP: Map = map::tcp_map_create();
            static ref UDP_MAP: Map = map::udp_map_create();
        }
        if mode == "tcp" {
            tcp_mode = match ScanMode::new(choice, &TCP_MAP) {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(())
            }?;
            udp_mode = None;
        } else if mode == "udp" {
            udp_mode = match ScanMode::new(choice, &UDP_MAP) {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(())
            }?;
            tcp_mode = None;
        } else {
            return Err(());
        }
        Ok(
            Host {
                ip: host_ip,
                inter_ip: inter_ip,
                tcp_mode,
                udp_mode
            }
        )
    }

    fn thread_builder(
        &self,
        subset: Vec<u16>, 
        func: fn(Ipv4Addr, Ipv4Addr, Vec<u16>, u16)
    ) -> JoinHandle<()>
    {
        let builder: Builder = Builder::new()
                    .stack_size(64 * 1024);
        let mut rng = thread_rng();
        let src_port = rng.gen_range(49152..=65535);
        let ip = self.ip.clone();
        let inter_ip = self.inter_ip.clone();

        match builder.spawn(
            move || {
                func(ip, inter_ip, subset, src_port)
            }
        )
        {
            Ok(thread_hander) => thread_hander,
            Err(_) => {
                process::exit(12);
            }
        }
    }

    fn thread_joiner(thread_hd_list: Vec<JoinHandle<()>>)
    {
        for handler in thread_hd_list {
            let _ = handler.join();
        }
    }

    fn get_tcp_probe_result(
        host_ip: Ipv4Addr,
        inter_ip: Ipv4Addr,
        limit: (u16, u16),
        list: Option<Vec<u16>>,
        mut e_rx: Box<dyn DataLinkReceiver>, 
        res_t: Sender<Vec<(u16, String)>>,
    ) -> Result<JoinHandle<()>, ()>
    {
        let builder: Builder = Builder::new().stack_size(32 * 1024);

        match builder.spawn(
            move || {
                let mut result: Vec<(u16, String)> = Vec::new();
                let start = SystemTime::now();
                let mut now = SystemTime::now();
                while now.duration_since(start).unwrap().as_millis() <= 2000 {
                    match e_rx.next() {
                        Ok(ether_packet) => {
                            let packet = EthernetPacket::new(ether_packet).unwrap();
                            let mut ip_pack = packet.payload().to_owned();
                            let ip_pack1 = Ipv4Packet::new(&mut ip_pack).unwrap();
                            let source_addr = ip_pack1.get_source();
                            let dest_addr = ip_pack1.get_destination();
                            let mut tcp_pack = ip_pack1.payload().to_owned();

                            if source_addr == host_ip && dest_addr == inter_ip {
                                match TcpPacket::new(&mut tcp_pack) {
                                    Some(tcp_pack1) => {
                                        let source_port = tcp_pack1.get_source();
                                        if tcp_pack1.get_flags() == (TcpFlags::ACK | TcpFlags::SYN) {
                                            println!("{}:{}", source_addr, source_port);
                                            match list {
                                                Some(ref val) => {
                                                    if val.contains(&source_port) {
                                                        result.push((source_port, String::from("Open")));
                                                    }
                                                }
                                                None => {
                                                    if source_port >= limit.0  && source_port <= limit.1 {
                                                        result.push((source_port, String::from("Open")))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    None => {}
                                }
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

    fn portlist_to_scaninfo(port_result: Vec<(u16, String)>) -> Vec<ScanResultInfo>
    {
        let mut infos: Vec<ScanResultInfo> = Vec::new();
        let mut info: ScanResultInfo;

        for res in port_result
        {
            info = ScanResultInfo {
                port: res.0,
                status: res.1
            };
            infos.push(info);
        }
        infos
    }

    fn portlist_to_json(&self, 
                        port_result: Vec<(u16, String)>
                    ) -> Result<String, serde_json::Error> 
    {
        let format = ScanResult {
            ipaddr: self.ip.clone(),
            tcp_ports: Host::portlist_to_scaninfo(port_result),
        };
        serde_json::to_string(&format)
    }

    fn scanner_helper(
        &self,
        obj_mode: &Option<ScanMode>,
        func: fn(Ipv4Addr, Ipv4Addr, Vec<u16>, u16)
    )
    {
        let ip = self.ip.clone();
        let mode: &ScanMode = &obj_mode.as_ref().unwrap();
        let n: u16 = mode.subset_len();
        let mut subset: Vec<u16>;
        let mut subset_thread: JoinHandle<()>;
        let mut thread_handler_list: Vec<JoinHandle<()>> = Vec::new();

        for subset_no in 0..n {
            subset = mode.get_subset(subset_no);
            subset_thread = Host::thread_builder(self, subset, func);
            thread_handler_list.push(subset_thread);
        }
        Host::thread_joiner(thread_handler_list)
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

    pub fn tcp_connect_scan(&self) -> Result<Option<String>, ()>
    {
        let ports_list: Vec<(u16, String)>;
        let reciever_handler: JoinHandle<()>;
        let interface: NetworkInterface = Host::find_match_interface(self.inter_ip)?;
        let (res_t, res_r) = channel();

        if self.tcp_mode.is_none() {
            return Ok(None);
        }
        let (_, e_rx) = match datalink::channel(&interface, Default::default()){
            Ok(Ethernet(tx, rx)) => Ok((tx, rx)),
            _ => Err(())
        }?;
        reciever_handler = Host::get_tcp_probe_result(
                                            self.ip.clone(), 
                                                    self.inter_ip.clone(), 
                                                    self.tcp_mode.as_ref().unwrap().get_limit(),
                                                    self.tcp_mode.as_ref().unwrap().get_portlist().clone(), 
                                                    e_rx, 
                                                    res_t.clone()
                                                    )?;
        Host::scanner_helper(&self, &self.tcp_mode, tcp_syn::tcp_ping);
        let _ = reciever_handler.join();
        ports_list = match res_r.recv() {
            Ok(mut val) => {
                val.sort();
                val.dedup();
                Ok(val)
            }
            Err(_) => Err(())
        }?;
        match Host::portlist_to_json(&self, ports_list) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Err(())
        }
    }
}
