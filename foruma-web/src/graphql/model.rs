use crate::domain::{
    Account, AccountId, AccountSession, GetAccount, GetAccountSessions, SessionId,
    UpdateLastActive, UpdateLastActiveError,
};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder};

pub type GraphQlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn current_account<'a>(&self, ctx: &'a Context<'a>) -> Option<Account> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        let session = ctx.data_opt::<SessionId>()?;

        let account = context.get_account(session).await?;

        match context.update_last_active(session).await {
            Ok(_) => (),
            Err(UpdateLastActiveError::SessionDoesNotExist) => return None,
        }

        Some(account)
    }
}

#[Object]
impl Account {
    async fn id(&self) -> &str {
        self.get_account_id().value()
    }

    async fn username(&self) -> &str {
        self.get_username().value()
    }

    async fn authentication(&self) -> Authentication {
        Authentication(self.get_account_id().clone())
    }
}

struct Authentication(AccountId);

#[Object]
impl Authentication {
    async fn sessions<'a>(&self, ctx: &'a Context<'a>) -> Vec<AccountSession> {
        let context = ctx
            .data::<actix_web::web::Data<crate::context::Context>>()
            .expect("Database not in context");

        context
            .get_account_sessions(&self.0)
            .await
            .unwrap_or_else(|_| Vec::new())
    }
}

#[Object]
impl AccountSession {
    async fn browser(&self) -> Option<String> {
        let normalized = match self.get_user_agent() {
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

    async fn id(&self) -> &str {
        self.get_session_id().value()
    }

    async fn is_current_session<'a>(&self, ctx: &'a Context<'a>) -> bool {
        let session_id = ctx
            .data_opt::<SessionId>()
            .expect("SessionId not in context");

        self.get_session_id().value() == session_id.value()
    }

    async fn last_active_date(&self) -> String {
        self.get_last_active().to_8601_string()
    }

    async fn location<'a>(&self, ctx: &'a Context<'a>) -> Option<String> {
        let geoip = ctx
            .data::<actix_web::web::Data<crate::geoip::GeoIp>>()
            .expect("GeoIp not in context");

        let location = self.get_ip_address().as_ref().and_then(|ip_address| {
            if let Ok(lookup) = geoip.lookup(&ip_address.value().ip(), "en") {
                Some(lookup.to_human_readable())
            } else {
                None
            }
        });

        location
    }

    async fn operating_system(&self) -> Option<String> {
        let normalized = match self.get_user_agent() {
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
}
