use crate::client::builder::ClientConfig;
use crate::error::NtpError;
use crate::packet::NtpPacket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;
use tokio::time;

pub struct NtpClient {
    config: ClientConfig,
}

impl NtpClient {
    pub fn new(config: ClientConfig) -> Result<Self, NtpError> {
        Ok(NtpClient { config })
    }

    pub async fn request_time(&self) -> Result<NtpPacket, NtpError> {
        let server_ip = IpAddr::V4(Ipv4Addr::from(self.config.server));
        let socket_addr = SocketAddr::new(server_ip, self.config.port);

        let socket = UdpSocket::bind("0.0.0.0:0").await?;

        let packet = NtpPacket::new();
        let data = packet.to_bytes();

        socket.send_to(&data, &socket_addr).await?;

        let mut buf = [0u8; 48];
        let recv_future = socket.recv_from(&mut buf);
        let (len, _) = time::timeout(self.config.timeout, recv_future)
            .await
            .map_err(|_| NtpError::Timeout)??;

        if len < 48 {
            return Err(NtpError::InvalidResponse);
        }

        let response = NtpPacket::from_bytes(&buf)?;
        Ok(response)
    }
}
