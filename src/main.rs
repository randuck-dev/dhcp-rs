use dhcp::RawPacket;

mod dhcp;
mod server;

fn main() {
    let server = server::DhcpServer::new();

    server.start();
}
