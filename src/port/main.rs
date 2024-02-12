mod host;
use std::env;
use std::process;
use std::str::Split;

use host::Host;


fn argv_to_list(argv: &[String]) -> Vec<(String, String, String)>
{
	let mut list: Vec<(String, String, String)> = Vec::new();
	let mut val1: String;
	let mut val2: String;
	let mut val3: String;
	let mut str_array: Split<&'static str>;

	for arg in argv
	{
		str_array = arg.split(",");
		val1 = match str_array.nth(0) {
			Some(str) => str.to_string(),
			None => String::from("")
		};
		val2 = match str_array.nth(0) {
			Some(str) => str.to_string(),
			None => String::from("")
		};
		val3 = match str_array.nth(0) {
			Some(str) => str.to_string(),
			None => String::from("")
		};
		list.push((val1, val2, val3));
	}
	return list;
}

fn main() -> std::io::Result<()>
{
	let list: Vec<(String, String, String)>;
	let argv: Vec<String> = env::args().collect();
	let mut host_list: Vec<Host> = Vec::new();

	if argv.len() <= 0 { process::exit(1) }

	list = argv_to_list(&argv[1..]);
	for val in list
	{
		println!("{}, {}, {}", val.0, val.1, val.2);
		match Host::new(&val.0, &val.1, &val.2)
		{
			Ok(host) => host_list.push(host),
			Err(e) => panic!("Error !!! : {:?}", e)
		}
	}
	println!("------------------------------------------");
	for a_host in &host_list
	{
		println!("{} : {}", a_host.get_ipaddr(), a_host.tcp_connect_scan());
	}
	println!("------------------------------------------");
	for a_host in &host_list
	{
		println!("{} : {}", a_host.get_ipaddr(), a_host.udp_connect_scan());
	}
	Ok(())
}