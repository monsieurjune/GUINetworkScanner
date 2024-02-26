use ipnet::Ipv4AddrRange;
use network_interface::{
    Addr,
    V4IfAddr,
    NetworkInterface,
    NetworkInterfaceConfig
};
use std::{
    time::Duration,
    any::Any,
    process,
    env
};
use std::sync::mpsc::{
    Sender,
    Receiver,
    channel
};
use std::net::Ipv4Addr;
use serde::{
    Serialize,
    Deserialize
};
use serde_json::{from_str, to_string_pretty};
use std::thread::{
    Builder,
    JoinHandle
};
use xenet::net::interface;
mod icmp_ping;
mod window_ping;
mod rand_ping;

#[derive(Serialize, Deserialize)]
pub struct Prober
{
    name: String,
    addr_set: Vec<Ipv4Addr>
}

impl Prober
{
    // fn name_to_interface(&self) -> interface::Interface
    // {
    //     let interfaces: Vec<interface::Interface> = xenet::net::interface::get_interfaces();
        
    //     match interfaces.into_iter().find(|inter| inter.name == self.name)
    //     {
    //         Some(val) => val,
    //         None => {
    //             process::exit(255);
    //         }
    //     }
    // }

    fn thread_builder(&self, addr: Ipv4Addr, thread_id: usize, tx: Sender<Ipv4Addr>) -> JoinHandle<()>
    {
        let builder: Builder = Builder::new()
                                .name(thread_id.to_string())
                                .stack_size(32 * 1024);

        match builder.spawn(move || {
            window_ping::ping(addr, &tx);
        }) {
            Ok(handler) => handler,
            Err(e) => {
                eprintln!("Thread Allocation failed due to {:?}", e);
                process::exit(12);
            }
        }
    }

    fn thread_listener(length: usize, rx: Receiver<Ipv4Addr>) -> Vec<Ipv4Addr>
    {
        let mut result: Vec<Ipv4Addr> = Vec::new();
        let time: Duration = Duration::new(0, 030_000_000);

        for _ in 0..length
        {
            if let Ok(val) = rx.recv_timeout(time) {
                result.push(val);
            }
        }
        result
    }

    fn thread_joiner(handler_list: Vec<JoinHandle<()>>)
    {
        for handler in handler_list {
            match handler.join() {
                Ok(_) => {},
                Err(_) => {}
            }
        }
    }

    pub fn probe(&self) -> Result<String, serde_json::Error>
    {
        let length: usize = self.addr_set.len();
        let (tx, rx): (Sender<Ipv4Addr>, Receiver<Ipv4Addr>) = channel();
        let prober_res: Prober;
        let mut handler_list: Vec<JoinHandle<()>> = Vec::new();
        let mut handler: JoinHandle<()>;

        for i in 0..length
        {
            handler = Prober::thread_builder(&self, 
                                                self.addr_set[i].clone(), 
                                                i, 
                                                tx.clone());
            handler_list.push(handler)
        }
        prober_res = Prober {
            name: self.name.clone(),
            addr_set: Prober::thread_listener(length, rx)
        };
        Prober::thread_joiner(handler_list);
        to_string_pretty(&prober_res)
    }
}