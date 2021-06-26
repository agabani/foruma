use crate::domain::SessionId;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::Error,
};
use tracing_actix_web::{
    root_span_macro::private::tracing::Span, DefaultRootSpanBuilder, RootSpanBuilder,
};

pub struct DomainRootSpanBuilder;

impl RootSpanBuilder for DomainRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        if let Some(session_id) = get_session_id(request) {
            let session_id = session_id.value();
            tracing_actix_web::root_span!(request, session_id)
        } else {
            DefaultRootSpanBuilder::on_request_start(request)
        }
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

fn get_session_id(request: &ServiceRequest) -> Option<SessionId> {
    use actix_web::HttpMessage;

    request.extensions().get::<SessionId>().cloned()
}
