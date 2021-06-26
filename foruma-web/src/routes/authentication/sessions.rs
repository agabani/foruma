use crate::domain::{
    AccountSession, GetAccountSessions, GetAccountSessionsError, Logout, LogoutError, SessionId,
    UserAgent,
};
use crate::geoip::GeoIp;
use crate::{context::Context, domain::GetAccount, http_request_ext::HttpRequestExt};
use actix_web::{web, HttpRequest, HttpResponse};
use time::OffsetDateTime;

#[derive(serde::Serialize)]
struct Response {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "isCurrentSession")]
    is_current_session: bool,

    #[serde(rename = "browser")]
    browser: Option<String>,

    #[serde(rename = "operatingSystem")]
    operating_system: Option<String>,

    #[serde(rename = "lastActiveDate")]
    last_active_date: String,

    #[serde(rename = "location")]
    location: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Request {
    #[serde(rename = "id")]
    id: String,
}

impl Response {
    fn parse(
        geoip: &GeoIp,
        current_session_id: &SessionId,
        account_session: &AccountSession,
    ) -> Self {
        let location = account_session
            .ip_address()
            .as_ref()
            .and_then(|ip_address| {
                if let Ok(lookup) = geoip.lookup(&ip_address.value().ip(), "en") {
                    Some(lookup.to_human_readable())
                } else {
                    None
                }
            });

        Self {
            id: account_session.session_id().value().to_string(),
            is_current_session: current_session_id.value() == account_session.session_id().value(),
            browser: parse_browser(account_session.user_agent()),
            operating_system: parse_operating_system(account_session.user_agent()),
            last_active_date: OffsetDateTime::now_utc().format("%Y-%m-%dT%H:%M:%S.%NZ"),
            location,
        }
    }
}

fn parse_browser(user_agent: &Option<UserAgent>) -> Option<String> {
    let normalized = match user_agent {
        Some(user_agent) => user_agent.value().to_lowercase(),
        None => {
            return None;
        }
    };

    if normalized.contains("chrome") {
        Some("Chrome".to_string())
    } else if normalized.contains("edge") {
        Some("Edge".to_string())
    } else if normalized.contains("firefox") {
        Some("Firefox".to_string())
    } else if normalized.contains("safari") {
        Some("Safari".to_string())
    } else {
        None
    }
}

fn parse_operating_system(user_agent: &Option<UserAgent>) -> Option<String> {
    let normalized = match user_agent {
        Some(user_agent) => user_agent.value().to_lowercase(),
        None => {
            return None;
        }
    };
    if normalized.contains("windows") {
        Some("Windows".to_string())
    } else if normalized.contains("macos") {
        Some("MacOS".to_string())
    } else if normalized.contains("android") {
        Some("Android".to_string())
    } else if normalized.contains("linux") {
        Some("Linux".to_string())
    } else {
        None
    }
}

pub async fn get(
    http_request: HttpRequest,
    context: web::Data<Context>,
    geoip: web::Data<GeoIp>,
) -> Result<HttpResponse, actix_web::Error> {
    let geoip = geoip.get_ref();

    let session_id = match http_request.session_id() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let account = match context.get_account(&session_id).await {
        Some(account) => account,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let account_sessions = match context.get_account_sessions(&account.account_id()).await {
        Ok(account_sessions) => account_sessions,
        Err(GetAccountSessionsError::AccountDoesNotExist) => {
            return Ok(HttpResponse::NotFound().finish())
        }
    };

    return Ok(HttpResponse::Ok().json(
        account_sessions
            .iter()
            .map(|account_session| Response::parse(geoip, &session_id, &account_session))
            .collect::<Vec<_>>(),
    ));
}

pub async fn delete(
    http_request: HttpRequest,
    context: web::Data<Context>,
    request: web::Path<Request>,
) -> Result<HttpResponse, actix_web::Error> {
    /* TODO: Check if the session the user tried to delete belongs to the user.
     *       Return 404 if it does not belong to the user.
     */

    match http_request.session_id() {
        Some(session_id) => session_id,
        None => {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };

    let session_id = SessionId::new(&request.id);

    match context.logout(&session_id).await {
        Ok(u) => u,
        Err(LogoutError::SessionDoesNotExist) => {
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    Ok(HttpResponse::NoContent().finish())
}
