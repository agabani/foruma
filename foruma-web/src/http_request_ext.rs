use crate::domain::SessionId;
use actix_web::HttpRequest;

pub trait HttpRequestExt {
    fn session_id(&self) -> Option<SessionId>;
}

impl HttpRequestExt for HttpRequest {
    fn session_id(&self) -> Option<SessionId> {
        let extensions = self.extensions();
        let session_id = extensions.get::<SessionId>()?;
        Some(session_id.clone())
    }
}
