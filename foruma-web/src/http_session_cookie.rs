use crate::domain::SessionId;
use actix_web::cookie::{Cookie, CookieBuilder, CookieJar, Key, SameSite};
use actix_web::http::HeaderValue;

pub struct HttpSessionCookie {
    key: Key,
}

impl HttpSessionCookie {
    pub fn new(key: Key) -> Self {
        Self { key }
    }

    pub fn decrypt_session_id(&self, header_value: &HeaderValue) -> Option<SessionId> {
        header_value
            .to_str()
            .ok()
            .and_then(|value| Cookie::parse(value.to_string()).ok())
            .and_then(|cookie| CookieJar::new().private(&self.key).decrypt(cookie))
            .map(|cookie| SessionId::new(cookie.value().to_string()))
    }

    pub fn encrypt_session_id(&self, session_id: &SessionId) -> HeaderValue {
        let key = "session";

        let cookie = CookieBuilder::new(key, session_id.value().to_string())
            .http_only(true)
            .path("/")
            .same_site(SameSite::None)
            .secure(true)
            .finish();

        let mut jar = CookieJar::new();
        jar.private_mut(&self.key).add(cookie);
        let cookie = jar.get(key).cloned().unwrap();

        let mut value = HeaderValue::from_str(&cookie.to_string()).unwrap();
        value.set_sensitive(true);
        value
    }
}
