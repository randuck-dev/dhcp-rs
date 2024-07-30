use std::collections::HashMap;
use std::{fmt::format, net::UdpSocket};

use std::time::Instant;

use anyhow::{anyhow, Result};
use log::info;

use crate::dhcp::lease::{Lease, MACAddress};
use crate::dhcp::lib::parse_dhcp_packet;

pub struct DhcpServer {
    port: u16,

    lease_cache: HashMap<MACAddress, Lease>,
}

const DHCP_SERVER_PORT: u16 = 50010;
const DHCP_CLIENT_PORT: u16 = 68;

impl DhcpServer {
    pub fn new() -> Self {
        DhcpServer {
            port: DHCP_SERVER_PORT,
            lease_cache: HashMap::new(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let address = format!("127.0.0.1:{}", DHCP_SERVER_PORT);
        let socket = UdpSocket::bind(&address)?;

        info!("DHCP Server started listening on address: {}", &address);

        loop {
            let mut buf = [0; 1024];
            let (size, src) = socket.recv_from(&mut buf)?;
            info!("Received {} bytes from {}", size, src);
            let start = Instant::now();

            let packet = parse_dhcp_packet(&buf)?;

            let duration = start.elapsed();
            info!("Execution time: {:?}", duration);
        }
    }
}
