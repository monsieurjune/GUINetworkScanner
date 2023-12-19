mod scanmode;
mod scan;

use std::net::IpAddr;
use std::str::FromStr;
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
        match IpAddr::from_str(ipv4_str)
        {
            Ok(ipv4_res) =>
            {
                return Ok
                (
                    Host
                    {
                        ipv4: ipv4_res,
                        scan_mode: ScanMode::new(mode)
                    }
                )
            }
            Err(_) =>
            {
                return Err(());
            }
        }
    }

    pub fn scan(&self)
    {

    }
}