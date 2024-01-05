mod tcp_utils;
mod udp_utils;
mod scanmode;
mod scan;


// use json::{JsonResult, Error};
use scanmode::{
	ScanMode,
	HashNotFound
};

use std::net::{
	IpAddr,
	AddrParseError
};
use std::thread::{
	Builder,
	JoinHandle
};
use std::str::FromStr;
use std::any::Any;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct Host
{
    ip: IpAddr,
    tcp_mode: (bool, Option<ScanMode>),
	udp_mode: (bool, Option<ScanMode>)
}

#[derive(Debug, Clone)]
pub enum HostError
{
	AddrParseError(AddrParseError),
	HashNotFound(HashNotFound)
}

fn dump(a: Vec<u16>) -> Vec<u16>
{
	vec![0]
}

impl Host
{
	fn choice_to_scanmode(choice: &String, map: &HashMap<String, (u16, u16, Option<Vec<u16>>)>, scan: bool) -> Result<(bool, Option<ScanMode>), HostError>
	{
		if scan {
			match ScanMode::new(choice, map) {
				Ok(res) => Ok((true, Some(res))),
				Err(e) => Err(HostError::HashNotFound(e))
			}
		}
		else {
			Ok((false, None))
		}
	}

    pub fn new(ip_str: &String, tcp_choice: &String, tcp_scan: bool, udp_choice: &String, udp_scan: bool) -> Result<Host, HostError>
    {
		let tcp_result: (bool, Option<ScanMode>);
		let udp_result: (bool, Option<ScanMode>);
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

	fn thread_build(&self, thread_name: String, subset: Vec<u16>, func: fn(&IpAddr, Vec<u16>) -> Vec<u16>) -> Result<JoinHandle<Vec<u16>>, std::io::Error>
	{
		let builder: Builder = Builder::new().name(thread_name).stack_size(32 * 1024);
		let ip = self.ip.clone();

		return builder.spawn(move || { return func(&ip, subset); });
	}

	fn thread_handler_helper(&self, subset_name: String, subset: Vec<u16>, func: fn(&IpAddr, Vec<u16>) -> Vec<u16>) -> JoinHandle<Vec<u16>>
	{
		match Host::thread_build(self, subset_name, subset, func)
		{
			Ok(thread_handler) => thread_handler,
			Err(e) => panic!("Thread Allocation failed {}", e)
		}
	}

	fn thread_joiner(thead_handler_list: Vec<JoinHandle<Vec<u16>>>) -> Vec<Result<Vec<u16>, Box<(dyn Any + Send + 'static)>>>
	{
		let mut result: Result<Vec<u16>, Box<(dyn Any + Send + 'static)>>;
		let mut handler_result: Vec<Result<Vec<u16>, Box<(dyn Any + Send + 'static)>>> = Vec::new();

		for handler in thead_handler_list
		{
			result = handler.join();
			handler_result.push(result);
		}
		return handler_result;
	}

	fn get_thread_result(thread_result: Vec<Result<Vec<u16>, Box<(dyn Any + Send + 'static)>>>) -> Vec<u16>
	{
		let mut port_result: Vec<u16> = Vec::new();

		for result in thread_result
		{
			match result
			{
				Ok(ports) => {
					port_result.extend(ports);
				}
				Err(_) => {}
			}
		}
		return port_result;
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

	pub fn tcp_scan(&self) -> String
	{
		let n: u16 = self.tcp_mode.1.as_ref().unwrap().subset_len();
		let port_result: Vec<u16>;
		let thread_result: Vec<Result<Vec<u16>, Box<(dyn Any + Send + 'static)>>>;
		let log: String;
		let mut subset: Vec<u16>;
		let mut subset_thread: JoinHandle<Vec<u16>>;
		let mut thread_handler_list: Vec<JoinHandle<Vec<u16>>> = Vec::new();

		for subset_no in 0..n
		{
			subset = self.tcp_mode.1.as_ref().unwrap().get_subset(subset_no);
			subset_thread = Host::thread_handler_helper(self, subset_no.to_string(), subset, tcp_utils::scan);
			thread_handler_list.push(subset_thread);
		}
		thread_result = Host::thread_joiner(thread_handler_list);
		port_result = Host::get_thread_result(thread_result);
		log = Host::portlist_to_json(port_result);
		return log;
	}
}