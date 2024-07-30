use std::net::Ipv4Addr;

pub type MACAddress = [u8; 6];

#[derive(Debug, Clone)]
pub struct Lease {
    ip: Ipv4Addr,
    mac: String,
    expires: u64,
}

impl Lease {
    pub fn new(mac: String, ip: Ipv4Addr, expires: u64) -> Self {
        Lease { mac, ip, expires }
    }
}
