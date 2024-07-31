use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::net::UdpSocket;

use std::time::Instant;

use anyhow::Result;
use log::{debug, info};

use crate::dhcp;
use crate::dhcp::lease::Lease;
use crate::dhcp::lib::{parse_dhcp_packet, RawPacket};

use super::packet::Packet;

pub struct DhcpServer {
    port: u16,

    lease_cache: HashMap<String, Lease>,

    socket: Option<UdpSocket>,
}

const DHCP_SERVER_PORT: u16 = 50010;
const DHCP_CLIENT_PORT: u16 = 67;

impl DhcpServer {
    pub fn new() -> Self {
        DhcpServer {
            port: DHCP_SERVER_PORT,
            lease_cache: HashMap::new(),

            socket: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let address = format!("127.0.0.1:{}", DHCP_SERVER_PORT);
        let s = UdpSocket::bind(&address)?;

        self.socket = Some(s);

        info!("DHCP Server started listening on address: {}", &address);

        loop {
            let mut buf = [0; 1024];
            let (size, src) = self.socket.as_ref().unwrap().recv_from(&mut buf)?;
            debug!("Received {} bytes from {}", size, src);
            let start = Instant::now();

            let packet = parse_dhcp_packet(&buf)?;

            let msg_type = packet.get_message_type()?;
            match msg_type {
                dhcp::messagetype::MessageType::DHCPREQUEST => self.handle_dhcprequest(packet)?,
                dhcp::messagetype::MessageType::DHCPDECLINE => self.handle_decline(packet)?,
                dhcp::messagetype::MessageType::DHCPDISCOVER => self.handle_discover(packet)?,
                dhcp::messagetype::MessageType::DHCPINFORM => self.handle_inform(packet)?,
                dhcp::messagetype::MessageType::DHCPRELEASE => self.handle_release(packet)?,

                dhcp::messagetype::MessageType::DHCPOFFER => info!("DHCPOFFER: Nothing to do here"),
                dhcp::messagetype::MessageType::DHCPNAK => info!("DHCPNACK: Nothing to do here"),
                dhcp::messagetype::MessageType::DHCPACK => info!("DHCPACK: Nothing to do here"),
            }

            let duration = start.elapsed();
            debug!("Execution time: {:?}", duration);
        }
    }

    fn handle_dhcprequest(&mut self, packet: Packet) -> Result<()> {
        info!("send_dhcp_ack: DHCPREQUEST -> DHPACK");
        let mac_address = packet.get_client_identifier()?;

        let ipaddr = Ipv4Addr::new(192, 168, 1, 69);
        let expiration_s = 60 * 60 * 24;
        let lease = dhcp::lease::Lease::new(mac_address.clone(), ipaddr, expiration_s);

        match self.lease_cache.get(&mac_address) {
            Some(l) => {
                debug!("Already holding lease")
            }
            None => {
                debug!("Not holding lease, creating new and inserting");
                self.lease_cache.insert(mac_address, lease.clone());
            }
        }
        let mut rp = RawPacket::default(dhcp::messagetype::MessageType::DHCPACK);
        rp.set_xid(packet.xid);
        rp.set_client_ip_address(ipaddr);

        self.send(rp)?;

        Ok(())
    }

    fn handle_decline(&self, packet: Packet) -> Result<()> {
        todo!()
    }

    fn handle_discover(&self, packet: Packet) -> Result<()> {
        info!("handle_discover: DHCPDISCOVER -> DHCPOFFER");

        let mac_address = packet.get_client_identifier()?;
        info!("address: {}", &mac_address);

        let mut rp = RawPacket::default(dhcp::messagetype::MessageType::DHCPOFFER);
        rp.set_xid(packet.xid);

        self.send(rp)?;

        Ok(())
    }

    fn handle_inform(&self, packet: Packet) -> Result<()> {
        todo!()
    }

    fn handle_release(&self, packet: Packet) -> Result<()> {
        todo!()
    }

    fn send(&self, raw: RawPacket) -> Result<()> {
        let addr = format!("127.0.0.1:{}", DHCP_CLIENT_PORT);

        let s = self.socket.as_ref().unwrap();
        s.set_broadcast(true)?;
        s.send_to(&raw.buf, addr)?;

        Ok(())
    }
}
