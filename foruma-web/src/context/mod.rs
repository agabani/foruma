mod create_account;
mod create_password;
mod get_account;
mod log_in;
mod log_out;
mod terminate_account;

pub struct Context {
    postgres: sqlx::Pool<sqlx::Postgres>,
}

impl Context {
    pub fn new(postgres: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { postgres }
    }
}
