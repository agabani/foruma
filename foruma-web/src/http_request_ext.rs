use crate::domain::{IpAddress, SessionId};
use actix_web::HttpRequest;
use std::net::SocketAddr;

pub trait HttpRequestExt {
    fn client_ip(&self) -> Option<IpAddress>;
    fn session_id(&self) -> Option<SessionId>;
}

impl HttpRequestExt for HttpRequest {
    fn client_ip(&self) -> Option<IpAddress> {
        let connection_info = self.connection_info();
        let ip_port = connection_info.realip_remote_addr()?;
        let socket_addr: SocketAddr = ip_port.parse().ok()?;
        let ipnetwork = ipnetwork::IpNetwork::new(
            socket_addr.ip(),
            if socket_addr.is_ipv4() { 32 } else { 128 },
        )
        .ok()?;

        Some(IpAddress::new(&ipnetwork))
    }

    fn session_id(&self) -> Option<SessionId> {
        let extensions = self.extensions();
        let session_id = extensions.get::<SessionId>()?;
        Some(session_id.clone())
    }
}
