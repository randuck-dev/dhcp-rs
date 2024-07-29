mod dhcp;

fn main() {
    let server = dhcp::server::DhcpServer::new();

    server.start();
}
