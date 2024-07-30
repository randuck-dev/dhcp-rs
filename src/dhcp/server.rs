use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::{fmt::format, net::UdpSocket};

use std::time::Instant;

use anyhow::{anyhow, Result};
use log::info;

use crate::dhcp;
use crate::dhcp::lease::{Lease, MACAddress};
use crate::dhcp::lib::parse_dhcp_packet;

use super::packet::Packet;

pub struct DhcpServer {
    port: u16,

    lease_cache: HashMap<String, Lease>,
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

    pub fn run(&mut self) -> Result<()> {
        let address = format!("127.0.0.1:{}", DHCP_SERVER_PORT);
        let socket = UdpSocket::bind(&address)?;

        info!("DHCP Server started listening on address: {}", &address);

        loop {
            let mut buf = [0; 1024];
            let (size, src) = socket.recv_from(&mut buf)?;
            info!("Received {} bytes from {}", size, src);
            let start = Instant::now();

            let packet = parse_dhcp_packet(&buf)?;

            match packet.get_message_type()? {
                dhcp::messagetype::MessageType::DHCPREQUEST => self.send_dhcp_ack(packet)?,
                dhcp::messagetype::MessageType::DHCPDECLINE => self.handle_decline(packet)?,
                dhcp::messagetype::MessageType::DHCPDISCOVER => self.handle_discover(packet)?,
                dhcp::messagetype::MessageType::DHCPINFORM => self.handle_inform(packet)?,
                dhcp::messagetype::MessageType::DHCPRELEASE => self.handle_release(packet)?,

                dhcp::messagetype::MessageType::DHCPOFFER => info!("DHCPOFFER: Nothing to do here"),
                dhcp::messagetype::MessageType::DHCPNAK => info!("DHCPNACK: Nothing to do here"),
                dhcp::messagetype::MessageType::DHCPACK => info!("DHCPACK: Nothing to do here"),
            }

            let duration = start.elapsed();
            info!("Execution time: {:?}", duration);
        }
    }

    fn send_dhcp_ack(&mut self, packet: Packet) -> Result<()> {
        let mac_address = packet.get_client_identifier()?;
        info!("ClientIdentifier: {}", mac_address);

        let ipaddr = Ipv4Addr::new(192, 168, 1, 69);
        let expiration_s = 60 * 60 * 24;
        let lease = dhcp::lease::Lease::new(mac_address.clone(), ipaddr, expiration_s);

        match self.lease_cache.get(&mac_address) {
            Some(l) => {
                info!("Already holding lease")
            }
            None => {
                info!("Not holding lease, creating new and inserting");
                self.lease_cache.insert(mac_address, lease.clone());
            }
        }
        info!("Sending to client");

        Ok(())
    }

    fn handle_decline(&self, packet: Packet) -> Result<()> {
        todo!()
    }

    fn handle_discover(&self, packet: Packet) -> Result<()> {
        info!("handle_discover: Received request from client to ask for a DHCP server. Lucky us");

        let mac_address = packet.get_client_identifier()?;
        info!("address: {}", &mac_address);
        Ok(())
    }

    fn handle_inform(&self, packet: Packet) -> Result<()> {
        todo!()
    }

    fn handle_release(&self, packet: Packet) -> Result<()> {
        todo!()
    }
}
