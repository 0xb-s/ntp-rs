pub mod handler;

use std::net::SocketAddr;

use crate::error::NtpError;

use crate::server::handler::handle_request;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct NtpServer {
    addr: SocketAddr,
}

impl NtpServer {
    pub fn new(addr: SocketAddr) -> Self {
        NtpServer { addr }
    }

    pub async fn run(&self) -> Result<(), NtpError> {
        let socket = UdpSocket::bind(&self.addr).await?;
        log::info!("NTP Server listening on {}", self.addr);

        loop {
            let mut buf = [0u8; 48];
            let (len, src) = socket.recv_from(&mut buf).await?;
            log::info!("Received {} bytes from {}", len, src);

            let response = match handle_request(&buf) {
                Ok(pkt) => pkt.to_bytes().to_vec(),
                Err(e) => {
                    log::error!("Failed to handle request: {}", e);
                    Vec::new()
                }
            };

            if !response.is_empty() {
                socket.send_to(&response, &src).await?;
                log::info!("Sent response to {}", src);
            }
        }
    }
}
