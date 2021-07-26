mod graphql;
mod test_server;

#[actix_rt::test]
async fn should_be_able_to_authenticate() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let mut client = graphql::GraphQLClient::new(&test_server.address);

    // signup
    let response = client.signup(&"test-username", &"test-password").await;
    assert!(response.errors.is_none());
    assert!(response.data.unwrap().get("signup").unwrap());

    // check if logged in
    let response = client.current_account().await;
    assert!(response.errors.is_none());
    assert_eq!(
        response
            .data
            .unwrap()
            .get("currentAccount")
            .unwrap()
            .as_ref()
            .unwrap()
            .get("username")
            .unwrap(),
        "test-username"
    );

    // logout
    let response = client.logout_current_account().await;
    assert!(response.errors.is_none());
    assert!(response.data.unwrap().get("logoutCurrentAccount").unwrap());

    // check if logged out
    let response = client.current_account().await;
    assert!(response.errors.is_none());
    assert!(response
        .data
        .unwrap()
        .get("currentAccount")
        .unwrap()
        .is_none());

    // login
    let response = client.login(&"test-username", &"test-password").await;
    assert!(response.errors.is_none());
    assert!(response.data.unwrap().get("login").unwrap());

    // check if logged in
    let response = client.current_account().await;
    assert!(response.errors.is_none());
    assert_eq!(
        response
            .data
            .unwrap()
            .get("currentAccount")
            .unwrap()
            .as_ref()
            .unwrap()
            .get("username")
            .unwrap(),
        "test-username"
    );

    // logout
    let response = client.logout_current_account().await;
    assert!(response.errors.is_none());
    assert!(response.data.unwrap().get("logoutCurrentAccount").unwrap());

    // check if logged out
    let response = client.current_account().await;
    assert!(response.errors.is_none());
    assert!(response
        .data
        .unwrap()
        .get("currentAccount")
        .unwrap()
        .is_none());
}

#[actix_rt::test]
async fn should_make_unauthenticated_requests() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let mut client = graphql::GraphQLClient::new(&test_server.address);

    // login
    let response = client.login("admin", "password").await;
    assert!(response.data.is_none());
    assert_eq!(
        response
            .errors
            .as_ref()
            .unwrap()
            .first()
            .as_ref()
            .unwrap()
            .message,
        "bad_request"
    );

    // logout
    let response = client.logout_current_account().await;
    assert!(response.data.is_none());
    assert_eq!(
        response
            .errors
            .as_ref()
            .unwrap()
            .first()
            .as_ref()
            .unwrap()
            .message,
        "unauthenticated"
    );

    // current account
    let response = client.current_account().await;
    assert!(response.errors.is_none());
    assert!(response
        .data
        .unwrap()
        .get("currentAccount")
        .unwrap()
        .is_none());
}

#[actix_rt::test]
async fn should_not_be_able_to_sign_up_using_existing_account() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let mut client = graphql::GraphQLClient::new(&test_server.address);

    // signup
    let response = client.signup(&"test-username", &"test-password").await;
    assert!(response.errors.is_none());
    assert!(response.data.unwrap().get("signup").unwrap());

    // signup fails
    let response = client.signup(&"test-username", &"test-password").await;
    assert!(response.data.is_none());
    assert_eq!(
        response
            .errors
            .as_ref()
            .unwrap()
            .first()
            .as_ref()
            .unwrap()
            .message,
        "bad_request"
    );
}
