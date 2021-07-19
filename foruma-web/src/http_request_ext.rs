use crate::domain::{IpAddress, SessionId, UserAgent};
use actix_web::HttpRequest;

pub trait HttpRequestExt {
    fn client_ip(&self) -> Option<IpAddress>;
    fn session_id(&self) -> Option<SessionId>;
    fn user_agent(&self) -> Option<UserAgent>;
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

    fn user_agent(&self) -> Option<UserAgent> {
        self.headers()
            .get("User-Agent")
            .and_then(|value| value.to_str().ok())
            .map(|value| UserAgent::new(value.to_string()))
    }
}
