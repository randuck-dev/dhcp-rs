use log::error;
use simple_logger::SimpleLogger;

mod dhcp;

fn main() {
    SimpleLogger::new().init().unwrap();
    let server = dhcp::server::DhcpServer::new();

    if let Err(err) = server.run() {
        error!("Unexpected error occurred: {}", err);
        // Additional error handling code here
    }
}
