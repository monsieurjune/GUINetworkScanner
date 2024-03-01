mod map;
mod scanmode;
mod tcp_connect;
mod thread_utils;
mod udp_connect;

use self::map::Map;
use self::thread_utils::JoinHd;
use lazy_static::lazy_static;
use scanmode::{HashNotFound, ScanMode};
use serde::{Deserialize, Serialize};
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;

type Mode = Option<ScanMode>;

pub struct Host {
    ip: IpAddr,
    tcp_mode: Mode,
    udp_mode: Mode,
}

#[derive(Debug, Clone)]
pub enum HostError {
    AddrParseError(AddrParseError),
    HashNotFound(HashNotFound),
}

#[derive(Serialize, Deserialize)]
pub struct ScanResult {
    ipaddr: IpAddr,
    tcp_port: Vec<u16>,
}

impl Host {
    fn choice_to_scanmode(choice: &String, map: &Map, scan: bool) -> Result<Mode, HostError> {
        if choice == &"No" {
            return Ok(None);
        }
        if scan {
            match ScanMode::new(choice, map) {
                Ok(res) => Ok(Some(res)),
                Err(e) => Err(HostError::HashNotFound(e)),
            }
        } else {
            Ok(None)
        }
    }

    pub fn new(
        ip_str: &String,
        tcp_choice: &String,
        udp_choice: &String,
    ) -> Result<Host, HostError> {
        let tcp_result: Mode;
        let udp_result: Mode;
        let ip_res: IpAddr;

        lazy_static! {
            static ref TCP_MAP: Map = map::tcp_map_create();
            static ref UDP_MAP: Map = map::udp_map_create();
        }

        ip_res = match IpAddr::from_str(ip_str) {
            Ok(ip) => Ok(ip),
            Err(e) => Err(HostError::AddrParseError(e)),
        }?;
        tcp_result = Host::choice_to_scanmode(tcp_choice, &TCP_MAP, tcp_choice != &"No")?;
        udp_result = Host::choice_to_scanmode(udp_choice, &UDP_MAP, udp_choice != &"No")?;
        return Ok(Host {
            ip: ip_res,
            tcp_mode: tcp_result,
            udp_mode: udp_result,
        });
    }

    pub fn get_ipaddr(&self) -> IpAddr {
        self.ip
    }

    fn portlist_to_json(&self, port_result: Vec<u16>) -> String {
        let format = ScanResult {
            ipaddr: self.ip.clone(),
            tcp_port: port_result,
        };
        serde_json::to_string(&format).unwrap()
    }

    fn scanner_helper(
        &self,
        obj_mode: &Option<ScanMode>,
        func: thread_utils::ScanFunc,
    ) -> Vec<u16> {
        let ip = self.ip.clone();
        let mode: &ScanMode = &obj_mode.as_ref().unwrap();
        let n: u16 = mode.subset_len();
        let mut subset: Vec<u16>;
        let mut subset_thread: JoinHd;
        let mut thread_handler_list: Vec<JoinHd> = Vec::new();

        for subset_no in 0..n {
            subset = mode.get_subset(subset_no);
            subset_thread = thread_utils::thread_builder(ip, subset, subset_no.to_string(), func);
            thread_handler_list.push(subset_thread);
        }
        return thread_utils::thread_joiner(thread_handler_list);
    }

    pub fn tcp_connect_scan(&self) -> String {
        let ports_list: Vec<u16>;

        if self.tcp_mode.is_none() {
            return String::from("");
        }
        ports_list = Host::scanner_helper(self, &self.tcp_mode, tcp_connect::scan);
        return Host::portlist_to_json(self, ports_list);
    }

    pub fn udp_connect_scan(&self) -> String {
        let ports_list: Vec<u16>;

        if self.udp_mode.is_none() {
            return String::from("");
        }
        ports_list = Host::scanner_helper(self, &self.udp_mode, udp_connect::scan);
        return Host::portlist_to_json(self, ports_list);
    }
}
