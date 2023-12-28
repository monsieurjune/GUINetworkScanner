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

    fn thread_spawner(port_no: u16) -> Result<thread::JoinHandle<()>, std::io::Error>
    {
        let builder: thread::Builder;

        builder = thread::Builder::new().name(port_no.to_string());
        return builder.spawn( || {
            todo!();
        });
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
        let mut thread_results: Vec<Result<(), Box<(dyn Any + Send + 'static)>>>;

        match self.scan_mode.get_portlist()
        {
            Some(portlist) => {
                // for port in portlist
                // {
                //     handler = thread::spawn(move ||{
                //         println!("{port}");
                //     });
                //     thread_handlers.push(handler);
                // }
            }
            None => {
                port_limit = self.scan_mode.get_limit();
                for i in port_limit.0..port_limit.1
                {
                    match Host::thread_spawner(i)
                    {
                        Ok(handler) => {
                            thread_handlers.push(handler);
                        }
                        Err(_) => {
                            eprintln!("Thread allocation failed, stop making more thread");
                            break;
                        }
                    }
                }
                thread_results = Host::thread_joiner(thread_handlers);
                // Wait for writing Error handler
            }
        }
    }
}