mod change_password;
mod create_account;
mod create_password;
mod get_account;
mod get_account_sessions;
mod login;
mod logout;
mod terminate_account;

pub struct Context {
    postgres: sqlx::Pool<sqlx::Postgres>,
}

impl Context {
    pub fn new(postgres: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { postgres }
    }
}
