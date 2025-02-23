use crate::Result;
use std::{
    io,
    net::{IpAddr, TcpStream, ToSocketAddrs},
    time::{Duration, SystemTime},
};

quick_error! {
    #[derive(Debug)]
    pub enum UtilError {
        AddrResolveError
    }
}

#[allow(dead_code)]
fn tcp_ping(addr: &str) -> Result<u16> {
    let addr = addr.to_socket_addrs()?.next();
    if addr.is_none() {
        return Err(Box::new(UtilError::AddrResolveError));
    }
    let timeout = Duration::from_secs(3);
    let start_time = SystemTime::now();
    let stream = TcpStream::connect_timeout(&addr.unwrap(), timeout)?;
    stream.peer_addr()?;
    let d = SystemTime::now().duration_since(start_time)?;
    Ok(d.as_millis() as u16)
}

fn get_ips() -> Vec<IpAddr> {
    let ifs = pnet_datalink::interfaces();
    let mut ips = Vec::with_capacity(ifs.len());
    for i in ifs {
        if i.is_up() && i.is_multicast() {
            for ip in i.ips {
                if ip.is_ipv4() {
                    ips.push(ip.ip())
                }
            }
        }
    }
    ips
}

pub fn get_ip_by_if_name(if_name: &str) -> Option<String> {
    let ifs = pnet_datalink::interfaces();
    for i in ifs {
        if i.is_up() && i.name.contains(if_name) && !i.ips.is_empty() {
            for ip in i.ips {
                if ip.is_ipv4() {
                    return Some(ip.ip().to_string());
                }
            }
        }
    }
    None
}

pub fn select_ip() -> Option<String> {
    let ips = get_ips();
    if ips.is_empty() {
        return None;
    }
    if ips.len() == 1 {
        return Some(ips[0].to_string());
    }

    println!("Please select your IP:");
    for (n, ip) in ips.iter().enumerate() {
        println!("    {}. {}", n + 1, ip);
    }

    for t in 1..=3 {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let trimmed = input_text.trim();
        if let Ok(i) = trimmed.parse::<usize>() {
            if i > 0 && i <= ips.len() {
                let ip = ips[i - 1].to_string();
                println!("you choose {}", ip);
                return Some(ip);
            }
        }
        println!("not a valid index number, {}/3", t);
    }
    println!("invalid input for 3 times");
    None
}

#[test]
fn test_get_ips() {
    select_ip();
}

#[test]
fn test_get_ip_by_name() {
    println!("{:?}", get_ip_by_if_name("wlp3s0"));
}

#[test]
fn test_tcp_ping() {
    let p = tcp_ping("baidu.com:80");
    println!("{:?}", p);
}
