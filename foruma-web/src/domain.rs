// Types
#[derive(Debug)]
pub struct Account {
    account_id: AccountId,
    username: Username,
}

#[derive(Clone, Debug)]
pub struct AccountId(String);

pub struct AccountSession {
    session_id: SessionId,
    user_agent: Option<UserAgent>,
}

pub struct Password(String);

pub struct PasswordId(String);

#[derive(Clone, Debug)]
pub struct SessionId(String);

#[derive(Clone, Debug)]
pub struct UserAgent(String);

#[derive(Clone, Debug)]
pub struct Username(String);

// Functions
#[async_trait::async_trait]
pub trait CreateAccount {
    async fn create_account(&self, username: &Username) -> Result<Account, CreateAccountError>;
}

#[async_trait::async_trait]
pub trait ChangePassword {
    async fn change_password(
        &self,
        account: &Account,
        current_password: &Password,
        new_password: &Password,
    ) -> Result<(), ChangePasswordError>;
}

#[async_trait::async_trait]
pub trait CreatePassword {
    async fn create_password(&self, account: &Account, password: &Password);
}

#[async_trait::async_trait]
pub trait GetAccount {
    async fn get_account(&self, session_id: &SessionId) -> Option<Account>;
}

#[async_trait::async_trait]
pub trait GetAccountSessions {
    async fn get_account_sessions(
        &self,
        account_id: &AccountId,
    ) -> Result<Vec<AccountSession>, GetAccountSessionsError>;
}

#[async_trait::async_trait]
pub trait Login {
    async fn login(
        &self,
        username: &Username,
        password: &Password,
        user_agent: &Option<UserAgent>,
    ) -> Result<SessionId, LoginError>;
}

#[async_trait::async_trait]
pub trait Logout {
    async fn logout(&self, session_id: &SessionId) -> Result<(), LogoutError>;
}

#[async_trait::async_trait]
pub trait TerminateAccount {
    async fn terminate_account(&self, account: &Account) -> Result<(), TerminateAccountError>;
}

// Errors
pub enum ChangePasswordError {
    AccountDoesNotExist,
    AccountHasNoPassword,
    IncorrectPassword,
}

pub enum CreateAccountError {
    AccountAlreadyExists,
}

pub enum GetAccountSessionsError {
    AccountDoesNotExist,
}

pub enum LoginError {
    AccountDoesNotExist,
    AccountHasNoPassword,
    IncorrectPassword,
}

pub enum LogoutError {
    SessionDoesNotExist,
}

pub enum TerminateAccountError {
    AccountDoesNotExist,
}

// Implementations
impl Account {
    pub fn new(account_id: &AccountId, username: &Username) -> Self {
        Self {
            account_id: account_id.clone(),
            username: username.clone(),
        }
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }
}

impl AccountId {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AccountSession {
    pub fn new(session_id: &SessionId, user_agent: &Option<UserAgent>) -> Self {
        Self {
            session_id: session_id.clone(),
            user_agent: user_agent.clone(),
        }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn user_agent(&self) -> &Option<UserAgent> {
        &self.user_agent
    }
}

impl Password {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl PasswordId {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl SessionId {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl UserAgent {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Username {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
