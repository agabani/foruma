use actix_web::{
    cookie::{Cookie, CookieBuilder, CookieJar, Key, SameSite},
    dev::{forward_ready, MessageBody, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{COOKIE, SET_COOKIE},
        HeaderValue,
    },
    Error, HttpMessage, HttpResponse,
};
use std::{
    future::{ready, Future, Ready},
    marker::PhantomData,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

pub struct SessionId(Rc<Inner>);

struct Inner {
    key: Key,
}

impl SessionId {
    pub fn new(key: Key) -> Self {
        Self(Rc::new(Inner { key }))
    }
}

impl<S, B> Transform<S, ServiceRequest> for SessionId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionIdMiddleware {
            service,
            inner: self.0.clone(),
        }))
    }
}

pub struct SessionIdMiddleware<S> {
    inner: Rc<Inner>,
    service: S,
}

impl<S, B> Service<ServiceRequest> for SessionIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = SessionIdFuture<S, B>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let cookie_header: Option<&HeaderValue> = req.headers().get(COOKIE);

        let cookie = cookie_header
            .and_then(|cookie| cookie.to_str().ok())
            .and_then(|cookie_value| Cookie::parse(cookie_value.to_string()).ok())
            .and_then(|cookie| CookieJar::new().private(&self.inner.key).decrypt(cookie));

        if let Some(cookie) = cookie {
            req.extensions_mut()
                .insert(crate::domain::SessionId::new(cookie.value()));
        }

        let inner = self.inner.clone();
        let fut = self.service.call(req);

        SessionIdFuture {
            fut,
            inner,
            _body: PhantomData,
        }
    }
}

#[pin_project::pin_project]
pub struct SessionIdFuture<S: Service<ServiceRequest>, B> {
    #[pin]
    fut: S::Future,
    inner: Rc<Inner>,
    _body: PhantomData<B>,
}

impl<S, B> Future for SessionIdFuture<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = <S::Future as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let mut res: ServiceResponse<B> = match this.fut.poll(cx)? {
            Poll::Ready(t) => t,
            Poll::Pending => return Poll::Pending,
        };

        if let Some(session_id) = get_session_id(res.response()) {
            let key = "session";

            let cookie = CookieBuilder::new(key, session_id.value().to_string())
                .http_only(true)
                .path("/")
                .same_site(SameSite::None)
                .secure(true)
                .finish();

            let mut jar = CookieJar::new();
            jar.private_mut(&this.inner.key).add(cookie);
            let cookie = jar.get(key).cloned().unwrap();

            let mut value = HeaderValue::from_str(&cookie.to_string()).unwrap();
            value.set_sensitive(true);

            res.headers_mut().insert(SET_COOKIE, value);
        }

        Poll::Ready(Ok(res))
    }
}

fn get_session_id<B>(
    response: &HttpResponse<B>,
) -> Option<crate::domain::SessionId> {
    let extensions = response.extensions();
    let session_id = extensions.get::<crate::domain::SessionId>()?;
    Some(crate::domain::SessionId::new(session_id.value()))
}
