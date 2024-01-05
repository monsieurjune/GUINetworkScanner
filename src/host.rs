mod scanmode;
mod scan;
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

pub struct Host
{
    ipv4: IpAddr,
    scan_mode: ScanMode
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
    pub fn new(mode: &String, ipv4_str: &String) -> Result<Host, HostError>
    {
		match ScanMode::new(mode) {
			Ok(data) => {
				match IpAddr::from_str(ipv4_str) {
					Ok(ip) => Ok(Host{ipv4: ip, scan_mode: data }),
					Err(e2) => Err(HostError::AddrParseError(e2))
				}
			}
			Err(e1) => Err(HostError::HashNotFound(e1))
		}
    }

	fn thread_build(thread_name: String, subset: Vec<u16>, func: fn(Vec<u16>) -> Vec<u16>) -> Result<JoinHandle<Vec<u16>>, std::io::Error>
	{
		let builder: Builder = Builder::new().name(thread_name).stack_size(32 * 1024);

		return builder.spawn(move || { return func(subset); });
	}

	fn thread_handler_helper(subset_name: String, subset: Vec<u16>, func: fn(Vec<u16>) -> Vec<u16>) -> JoinHandle<Vec<u16>>
	{
		match Host::thread_build(subset_name, subset, func)
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
			port_str = port.to_string();
			format += &port_str;
		}
		return format;
	}

	pub fn tcp_scan(&self) -> String
	{
		let n: u16 = self.scan_mode.subset_len();
		let port_result: Vec<u16>;
		let thread_result: Vec<Result<Vec<u16>, Box<(dyn Any + Send + 'static)>>>;
		let log: String;
		let mut subset: Vec<u16>;
		let mut subset_thread: JoinHandle<Vec<u16>>;
		let mut thread_handler_list: Vec<JoinHandle<Vec<u16>>> = Vec::new();

		for subset_no in 0..n
		{
			subset = self.scan_mode.get_subset(subset_no);
			subset_thread = Host::thread_handler_helper(subset_no.to_string(), subset, dump);
			thread_handler_list.push(subset_thread);
		}
		thread_result = Host::thread_joiner(thread_handler_list);
		port_result = Host::get_thread_result(thread_result);
		log = Host::portlist_to_json(port_result);
		return log;
	}
}