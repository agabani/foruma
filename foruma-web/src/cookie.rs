use crate::domain::SessionId;
use actix_web::{HttpRequest, HttpResponseBuilder};

const SESSION: &str = "session";

pub trait SessionCookieHttpRequest {
    fn decrypt_session_cookie<'c>(&self, key: &cookie::Key) -> Option<cookie::Cookie<'c>>;
}

impl SessionCookieHttpRequest for HttpRequest {
    fn decrypt_session_cookie<'c>(&self, key: &cookie::Key) -> Option<cookie::Cookie<'c>> {
        let cookie = self.cookie(SESSION)?;
        let mut jar = cookie::CookieJar::new();
        jar.private_mut(&key).decrypt(cookie)
    }
}

pub trait SessionCookie {
    fn new(session_id: &SessionId) -> Self;
    fn session_id(&self) -> SessionId;
}

impl<'c> SessionCookie for cookie::Cookie<'c> {
    fn new(session_id: &SessionId) -> cookie::Cookie<'c> {
        cookie::CookieBuilder::new(SESSION, session_id.value().to_string())
            .http_only(true)
            .path("/")
            .same_site(cookie::SameSite::None)
            .secure(true)
            .finish()
    }

    fn session_id(&self) -> SessionId {
        SessionId::new(self.value())
    }
}

pub trait SessionCookieHttpResponseBuilder {
    fn encrypt_session_cookie(
        &mut self,
        key: &cookie::Key,
        cookie: cookie::Cookie<'static>,
    ) -> &mut Self;

    fn delete_session_cookie(&mut self, cookie: &mut cookie::Cookie) -> &mut Self;
}

impl SessionCookieHttpResponseBuilder for HttpResponseBuilder {
    fn encrypt_session_cookie(
        &mut self,
        key: &cookie::Key,
        cookie: cookie::Cookie<'static>,
    ) -> &mut Self {
        let mut jar = cookie::CookieJar::new();
        jar.private_mut(&key).add(cookie);
        let cookie = jar.get(SESSION).cloned().unwrap();
        self.cookie(cookie)
    }

    fn delete_session_cookie(&mut self, cookie: &mut cookie::Cookie) -> &mut Self {
        cookie.set_http_only(true);
        cookie.set_secure(true);
        self.del_cookie(cookie)
    }
}
