use std::collections::HashMap;
use std::{fmt::format, net::UdpSocket};

use std::time::Instant;

use crate::dhcp::lease::{Lease, MACAddress};
use crate::dhcp::lib::parse_dhcp_packet;

pub struct DhcpServer {
    port: u16,

    lease_cache: HashMap<MACAddress, Lease>,
}

const DHCP_SERVER_PORT: u16 = 50010;
const DHCP_CLIENT_PORT: u8 = 68;

impl DhcpServer {
    pub fn new() -> Self {
        DhcpServer {
            port: DHCP_SERVER_PORT,
            lease_cache: HashMap::new(),
        }
    }

    pub fn start(&self) {
        let address = format!("127.0.0.1:{}", DHCP_SERVER_PORT);
        println!("DHCP Server started listening on address: {}", address);

        let socket = match UdpSocket::bind(address) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to bind to port {}: {}", DHCP_SERVER_PORT, e);
                return;
            }
        };

        loop {
            let mut buf = [0; 1024];
            match socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    println!("Received {} bytes from {}", size, src);
                    let start = Instant::now();

                    match parse_dhcp_packet(&buf) {
                        Ok(packet) => match packet.get_client_identifier() {
                            Ok(message_type) => {
                                println!("CI: {:?}", message_type);
                            }
                            Err(e) => {
                                eprintln!("Failed: {}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to parse packet: {:?}", e);
                        }
                    }
                    let duration = start.elapsed();
                    println!("Execution time: {:?}", duration);
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    break;
                }
            }
        }
    }
}
