mod scanmode;
mod scan;

use std::net::IpAddr;
use std::str::FromStr;
use std::thread;
use std::any::Any;
use scanmode::ScanMode;

pub struct Host
{
    ipv4: IpAddr,
    scan_mode: scanmode::ScanMode
}

impl Host
{
    pub fn new(mode: usize, ipv4_str: &String) -> Result<Host, ()>
    {
        match IpAddr::from_str(ipv4_str) {
            Ok(ipv4_res) => {
                return Ok (
                    Host {
                        ipv4: ipv4_res,
                        scan_mode: ScanMode::new(mode)
                    }
                )
            }
            Err(_) => {
                return Err(());
            }
        }
    }

    fn thread_build(port_no: u16) -> Result<thread::JoinHandle<()>, std::io::Error>
    {
        let builder: thread::Builder;

        builder = thread::Builder::new().name(port_no.to_string()).stack_size(32 * 1024);
        return builder.spawn( || {
            println!("Here");
        });
    }

	fn thread_handler_helper(port_no: u16, thread_handler: &mut Vec<thread::JoinHandle<()>>) -> bool
	{
		match Host::thread_build(port_no)
		{
			Ok(handler) => {
				thread_handler.push(handler);
				return false;
			}
			Err(_) => {
				return true;
			}
		}
	}

    fn thread_joiner(thread_handler: Vec<thread::JoinHandle<()>>) -> Vec<Result<(), Box<(dyn Any + Send + 'static)>>>
    {
        let mut res:Result<(), Box<(dyn Any + Send + 'static)>>;
        let mut handler_result: Vec<Result<(), Box<(dyn Any + Send + 'static)>>> = Vec::new();

        for handler in thread_handler
        {
            res = handler.join();
            handler_result.push(res);
        }
        return handler_result;
    }

    pub fn scan(&self)
    {
        let port_limit: (u16, u16);
        let mut thread_handlers: Vec<thread::JoinHandle<()>> = Vec::new();
        let mut _thread_results: Vec<Result<(), Box<(dyn Any + Send + 'static)>>>;

        match self.scan_mode.get_portlist()
        {
            Some(portlist) => {
                for port in portlist
				{
					if Host::thread_handler_helper(port, &mut thread_handlers) {
						break;
					}
				}
            }
            None => {
                port_limit = self.scan_mode.get_limit();
                for i in port_limit.0..port_limit.1
                {
					if Host::thread_handler_helper(i, &mut thread_handlers) {
						break;
					}
                }
            }
        }
		_thread_results = Host::thread_joiner(thread_handlers);
    }
}