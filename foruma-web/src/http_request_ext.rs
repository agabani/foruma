use crate::domain::{IpAddress, SessionId};
use actix_web::HttpRequest;

pub trait HttpRequestExt {
    fn client_ip(&self) -> Option<IpAddress>;
    fn session_id(&self) -> Option<SessionId>;
}

impl HttpRequestExt for HttpRequest {
    fn client_ip(&self) -> Option<IpAddress> {
        IpAddress::parse(self.connection_info().realip_remote_addr()?).ok()
    }

    fn session_id(&self) -> Option<SessionId> {
        let extensions = self.extensions();
        let session_id = extensions.get::<SessionId>()?;
        Some(session_id.clone())
    }
}
