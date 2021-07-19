use crate::http_session_cookie::HttpSessionCookie;
use actix_web::{
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
    http_session_cookie: HttpSessionCookie,
}

impl SessionId {
    pub fn new(http_session_cookie: HttpSessionCookie) -> Self {
        Self(Rc::new(Inner {
            http_session_cookie,
        }))
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

#[allow(clippy::module_name_repetitions)]
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

        let session_id = cookie_header.and_then(|header_value| {
            self.inner
                .http_session_cookie
                .decrypt_session_id(header_value)
        });

        if let Some(session_id) = session_id {
            req.extensions_mut().insert(session_id);
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
            let http_session_cookie: &HttpSessionCookie = &this.inner.http_session_cookie;

            let header_value = http_session_cookie.encrypt_session_id(&session_id);

            res.headers_mut().insert(SET_COOKIE, header_value);
        }

        Poll::Ready(Ok(res))
    }
}

fn get_session_id<B>(response: &HttpResponse<B>) -> Option<crate::domain::SessionId> {
    let extensions = response.extensions();
    Some(extensions.get::<crate::domain::SessionId>()?.clone())
}
