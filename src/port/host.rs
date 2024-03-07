mod map;
mod scanmode;
mod tcp_connect;
mod thread_utils;

use self::map::Map;
use self::thread_utils::JoinHd;
use lazy_static::lazy_static;
use scanmode::{HashNotFound, ScanMode};
use serde::{Deserialize, Serialize};
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;

type Mode = Option<ScanMode>;

#[allow(dead_code)]
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
pub struct ScanResultInfo {
    port: u16,
    status: String
}

#[derive(Serialize, Deserialize)]
pub struct ScanResult {
    ipaddr: IpAddr,
    tcp_ports: Vec<ScanResultInfo>,
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
        Ok(
            Host {
                ip: ip_res,
                tcp_mode: tcp_result,
                udp_mode: udp_result,
            }
        )
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
        func: thread_utils::ScanFunc,
    ) -> Vec<(u16, String)> {
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
        thread_utils::thread_joiner(thread_handler_list)
    }

    pub fn tcp_connect_scan(&self) -> Result<Option<String>, serde_json::Error>  {
        let ports_list: Vec<(u16, String)>;
        // let nothing: ScanResult;

        if self.tcp_mode.is_none() {
            return Ok(None);
        }
        ports_list = Host::scanner_helper(self, &self.tcp_mode, tcp_connect::scan);
        match Host::portlist_to_json(self, ports_list) {
            Ok(val) => Ok(Some(val)),
            Err(e) => Err(e)
        }
    }
}
