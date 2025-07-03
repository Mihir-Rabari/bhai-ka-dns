use crate::errors::Result;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::tcp::TcpClientConnection;
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_proto::rr::{Record, RrKey};
use std::net::{IpAddr, SocketAddr};

pub struct DnsResolver {
    tcp_client: AsyncClient,
    udp_client: AsyncClient,
}

impl DnsResolver {
    pub async fn new(server_addr: SocketAddr) -> Result<Self> {
        let tcp_conn = TcpClientConnection::new(server_addr)?;
        let (tcp_client, tcp_bg) = AsyncClient::new(tcp_conn, Default::default()).await?;
        tokio::spawn(tcp_bg);

        let udp_conn = UdpClientConnection::new(server_addr)?;
        let (udp_client, udp_bg) = AsyncClient::new(udp_conn, Default::default()).await?;
        tokio::spawn(udp_bg);

        Ok(Self {
            tcp_client,
            udp_client,
        })
    }

    pub async fn resolve(&mut self, key: &RrKey) -> Result<Vec<Record>> {
        // Try UDP first, fall back to TCP if needed
        match self.udp_client.query(key.name().clone(), key.record_class(), key.record_type()).await {
            Ok(response) => Ok(response.answers().to_vec()),
            Err(_) => {
                let response = self.tcp_client.query(key.name().clone(), key.record_class(), key.record_type()).await?;
                Ok(response.answers().to_vec())
            }
        }
    }
}