use network_interface::{
    Addr::{
        V4,
        V6
    },
    Netmask,
    NetworkInterface,
};
use std::net::Ipv4Addr;
use ipnet::Ipv4Net;

fn combine_ip_and_netmask(ip: Ipv4Addr, netmask_res: Netmask<Ipv4Addr>) -> Option<Ipv4Net>
{
    let netmark: Ipv4Addr;

    match netmask_res {
        Some(res) => { netmark = res },
        None => { return None; }
    }

    match Ipv4Net::with_netmask(ip, netmark) {
        Ok(ipaddr) => Some(ipaddr),
        Err(_) => None
    }
}

pub fn lexer(clean_interfaces: Vec<NetworkInterface>) -> Vec<Ipv4Net>
{
    let mut ip_list: Vec<Ipv4Net> = Vec::new();

    for interface in clean_interfaces
    {
        match interface.addr[0] {
            V4(v4if) => {
                match combine_ip_and_netmask(v4if.ip.clone(), v4if.netmask.clone()) {
                    
                }
            }
            V6(_) => {}
        }
    }
    return ip_list;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::Ipv4Addr;
    use std::str::FromStr;

    fn netmasker(nm: &str) -> u8 {
        let mask = Ipv4Addr::from_str(nm);
        return netmask_to_cidr(mask.unwrap());
    }

    fn ip_n_netmask(ip: &str, nm: &str) -> String {
        let ipv4 = Ipv4Addr::from_str(ip).unwrap();
        let netmask = Ipv4Addr::from_str(nm).unwrap();
        combine_ip_and_netmask(ipv4, netmask)
    }

    #[test]
    fn netmask_to_cidr1() {
        assert_eq!(netmasker("255.255.255.0"), 24)
    }

    #[test]
    fn netmask_to_cidr2() {
        assert_eq!(netmasker("255.0.0.0"), 8)
    }

    #[test]
    fn netmask_to_cidr3() {
        assert_eq!(netmasker("255.255.240.0"), 20)
    }

    #[test]
    fn netmask_to_cidr4() {
        assert_eq!(netmasker("255.255.255.192"), 26)
    }

    #[test]
    fn netmask_to_cidr5() {
        assert_eq!(netmasker("255.255.255.128"), 25)
    }

    #[test]
    fn netmask_to_cidr6() {
        assert_eq!(netmasker("255.255.255.224"), 27)
    }

    #[test]
    fn combine_ip_and_netmask1() {
        assert_eq!(ip_n_netmask("127.0.0.1", "255.255.255.0"), "127.0.0.1/24")
    }

    #[test]
    fn combine_ip_and_netmask2() {
        assert_eq!(ip_n_netmask("127.0.0.1", "255.0.0.0"), "127.0.0.1/8")
    }

    #[test]
    fn combine_ip_and_netmask3() {
        assert_eq!(ip_n_netmask("127.0.0.1", "255.255.254.0"), "127.0.0.1/23")
    }

    #[test]
    fn combine_ip_and_netmask4() {
        assert_eq!(ip_n_netmask("127.0.0.1", "255.255.255.192"), "127.0.0.1/26")
    }

    #[test]
    fn combine_ip_and_netmask5() {
        assert_eq!(ip_n_netmask("127.0.0.1", "255.255.255.128"), "127.0.0.1/25")
    }

    #[test]
    fn combine_ip_and_netmask6() {
        assert_eq!(ip_n_netmask("127.0.0.1", "255.255.255.224"), "127.0.0.1/27")
    }
}