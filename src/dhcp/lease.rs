use std::net::Ipv4Addr;

pub type MACAddress = [u8; 6];

pub struct Lease {
    ip: Ipv4Addr,
    mac: MACAddress,
    expires: u64,

    host: Option<String>,
}
