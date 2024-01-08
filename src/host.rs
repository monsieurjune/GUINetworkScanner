mod tcp_utils;
mod udp_utils;
mod scanmode;
mod thread_utils;

// use json::{JsonResult, Error};
use scanmode::{
	ScanMode,
	HashNotFound
};

use std::net::{
	IpAddr,
	AddrParseError
};
use std::str::FromStr;
use std::collections::HashMap;
use lazy_static::lazy_static;

use self::thread_utils::JoinHd;

type Mode = Option<ScanMode>;

pub struct Host
{
    ip: IpAddr,
    tcp_mode: Mode,
	udp_mode: Mode
}

#[derive(Debug, Clone)]
pub enum HostError
{
	AddrParseError(AddrParseError),
	HashNotFound(HashNotFound)
}

impl Host
{
	fn choice_to_scanmode(choice: &String, map: &HashMap<String, (u16, u16, Option<Vec<u16>>)>, scan: bool) -> Result<Mode, HostError>
	{
		if scan {
			match ScanMode::new(choice, map) {
				Ok(res) => Ok(Some(res)),
				Err(e) => Err(HostError::HashNotFound(e))
			}
		}
		else {
			Ok(None)
		}
	}

    pub fn new(ip_str: &String, tcp_choice: &String, tcp_scan: bool, udp_choice: &String, udp_scan: bool) -> Result<Host, HostError>
    {
		let tcp_result: Mode;
		let udp_result: Mode;
		let ip_res: IpAddr;

		lazy_static! {
			static ref TCP_MAP: HashMap<String, (u16, u16, Option<Vec<u16>>)> = tcp_utils::create_map();
			static ref UDP_MAP: HashMap<String, (u16, u16, Option<Vec<u16>>)> = udp_utils::create_map();
		}

		ip_res = match IpAddr::from_str(ip_str) {
			Ok(ip) => Ok(ip),
			Err(e) => Err(HostError::AddrParseError(e))
		}?;
		tcp_result = Host::choice_to_scanmode(tcp_choice, &TCP_MAP, tcp_scan)?;
		udp_result = Host::choice_to_scanmode(udp_choice, &UDP_MAP, udp_scan)?;
		return Ok(
			Host {
				ip: ip_res,
				tcp_mode: tcp_result,
				udp_mode: udp_result
			}
		);
    }

	fn portlist_to_json(port_result: Vec<u16>) -> String
	{
		let mut format: String = String::from("");
		let mut port_str: String;

		for port in port_result
		{
			port_str = " ,".to_string() + &port.to_string();
			format += &port_str;
		}
		return format;
	}

	fn scanner_helper(&self, obj_mode: &Option<ScanMode>, func: thread_utils::ScanFunc) -> Vec<u16>
	{
		let ip = self.ip.clone();
		let mode: &ScanMode = &obj_mode.as_ref().unwrap();
		let n: u16 = mode.subset_len();
		let mut subset: Vec<u16>;
		let mut subset_thread: JoinHd;
		let mut thread_handler_list: Vec<JoinHd> = Vec::new();

		for subset_no in 0..n
		{
			subset = mode.get_subset(subset_no);
			subset_thread = thread_utils::thread_builder(ip, subset, subset_no.to_string(), func);
			thread_handler_list.push(subset_thread);
		}
		return thread_utils::thread_joiner(thread_handler_list);
	}

	pub fn tcp_scan(&self) -> String
	{
		let ports_list: Vec<u16>;
		
		if self.tcp_mode.is_none() {
			return String::from("");
		}

		ports_list = Host::scanner_helper(self, &self.tcp_mode, tcp_utils::scan);
		return Host::portlist_to_json(ports_list);
	}
}