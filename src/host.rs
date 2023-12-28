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

    // fn thread_spawn()
    // {

    // }

    // fn

    pub fn scan(&self)
    {
        let port_limit: (u16, u16);
        let mut handler: thread::JoinHandle<()>;
        let mut thread_handlers: Vec<thread::JoinHandle<()>> = Vec::new();
        let mut thread_results: Vec<Result<(), Box<(dyn Any + Send + 'static)>>> = Vec::new();

        match self.scan_mode.get_portlist()
        {
            Some(_portlist) => {
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
                    handler = thread::spawn(move ||{
                        println!("{i}");
                    });
                }

                for thread_status in thread_handlers
                {
                    let res:Result<(), Box<(dyn Any + Send + 'static)>> = thread_status.join();
                    thread_results.push(res);
                }
                
                for res in thread_results
                {
                    match res
                    {
                        Ok(_) => {
                            // println!("Ok");
                        }
                        Err(_) => {
                            panic!("Err");
                        }
                    }
                }
            }
        }
    }
}