use crate::error::NtpError;

use std::time::Duration;

use super::NtpClient;

pub type ServerAddress = [u8; 4];
pub type ServerPort = u16;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub server: ServerAddress,
    pub port: ServerPort,
    pub timeout: Duration,
}

impl ClientConfig {
    fn new() -> Self {
        ClientConfig {
            server: [0, 0, 0, 0],
            port: 123,
            timeout: Duration::from_secs(5),
        }
    }
}

pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder {
            config: ClientConfig::new(),
        }
    }

    pub fn server(mut self, server: ServerAddress) -> Self {
        self.config.server = server;
        self
    }

    pub fn port(mut self, port: ServerPort) -> Self {
        self.config.port = port;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<NtpClient, NtpError> {
        NtpClient::new(self.config)
    }
}
