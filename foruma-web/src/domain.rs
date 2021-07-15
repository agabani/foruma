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
    ip_address: Option<IpAddress>,
    user_agent: Option<UserAgent>,
    last_active: LastActive,
}

#[derive(Clone, Debug)]
pub struct IpAddress(ipnetwork::IpNetwork);

#[derive(Clone)]
pub struct LastActive(time::OffsetDateTime);

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
        ip_address: &Option<IpAddress>,
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

#[async_trait::async_trait]
pub trait UpdateLastActive {
    async fn update_last_active(&self, session_id: &SessionId)
        -> Result<(), UpdateLastActiveError>;
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

pub enum UpdateLastActiveError {
    SessionDoesNotExist,
}

// Implementations
impl Account {
    pub fn new(account_id: &AccountId, username: &Username) -> Self {
        Self {
            account_id: account_id.clone(),
            username: username.clone(),
        }
    }

    pub fn get_account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn get_username(&self) -> &Username {
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
    pub fn new(
        session_id: &SessionId,
        ip_address: &Option<IpAddress>,
        user_agent: &Option<UserAgent>,
        last_active: &LastActive,
    ) -> Self {
        Self {
            session_id: session_id.clone(),
            ip_address: ip_address.clone(),
            user_agent: user_agent.clone(),
            last_active: last_active.clone(),
        }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn ip_address(&self) -> &Option<IpAddress> {
        &self.ip_address
    }

    pub fn user_agent(&self) -> &Option<UserAgent> {
        &self.user_agent
    }

    pub fn last_active(&self) -> &LastActive {
        &self.last_active
    }
}

impl IpAddress {
    #[allow(dead_code)]
    pub fn new(ip_address: &ipnetwork::IpNetwork) -> Self {
        Self(*ip_address)
    }

    pub fn parse(value: &str) -> Result<Self, ()> {
        Ok(Self(if let Ok(ip_network) = value.parse() {
            ip_network
        } else {
            let socket_address: std::net::SocketAddr = match value.parse() {
                Ok(socket_address) => socket_address,
                Err(_) => return Err(()),
            };
            ipnetwork::IpNetwork::from(socket_address.ip())
        }))
    }

    pub fn value(&self) -> &ipnetwork::IpNetwork {
        &self.0
    }
}

impl LastActive {
    pub fn new(value: &time::OffsetDateTime) -> Self {
        Self(*value)
    }

    pub fn value(&self) -> &time::OffsetDateTime {
        &self.0
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

#[cfg(test)]
mod tests {
    use super::*;

    mod ip_address {
        use super::*;

        #[test]
        fn ipv4() {
            let address = IpAddress::parse("127.0.0.1").unwrap();
            assert_eq!(
                address.value(),
                &ipnetwork::IpNetwork::new(
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                    32
                )
                .unwrap()
            )
        }

        #[test]
        fn ipv4_port() {
            let address = IpAddress::parse("127.0.0.1:8080").unwrap();
            assert_eq!(
                address.value(),
                &ipnetwork::IpNetwork::new(
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                    32
                )
                .unwrap()
            )
        }

        #[test]
        fn ipv6() {
            let address = IpAddress::parse("2001:0db8:85a3:0000:0000:8a2e:0370:7334").unwrap();
            assert_eq!(
                address.value(),
                &ipnetwork::IpNetwork::new(
                    std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                        0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334
                    )),
                    128
                )
                .unwrap()
            )
        }

        #[test]
        fn ipv6_port() {
            let address =
                IpAddress::parse("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:8080").unwrap();
            assert_eq!(
                address.value(),
                &ipnetwork::IpNetwork::new(
                    std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                        0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334
                    )),
                    128
                )
                .unwrap()
            )
        }
    }
}
