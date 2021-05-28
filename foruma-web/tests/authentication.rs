use reqwest::{Method, Response};
use std::collections::HashMap;

mod test_server;

fn assert_preflight_access_control_allow_headers(methods: &[Method], response: &Response) {
    let methods = methods
        .iter()
        .map(|method| method.as_str())
        .collect::<Vec<_>>()
        .join(",");

    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Headers")
            .expect("Access-Control-Allow-Headers"),
        "content-type"
    );
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Methods")
            .expect("Access-Control-Allow-Methods"),
        &methods
    );
}

fn assert_access_control_allow_headers(response: &Response) {
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Credentials")
            .expect("Access-Control-Allow-Credentials"),
        "true"
    );
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Origin")
            .expect("Access-Control-Allow-Origin"),
        "http://localhost:8080"
    );
}

#[actix_rt::test]
async fn should_be_able_to_authenticate() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let client = reqwest::ClientBuilder::new().build().unwrap();

    // sign up
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/v1/authentication/signup", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::POST], &response);
    assert_access_control_allow_headers(&response);

    let mut map = HashMap::new();
    map.insert("username", "test-username");
    map.insert("password", "test-password");
    let response = client
        .post(&format!(
            "{}/api/v1/authentication/signup",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let cookie_header = response.headers().get("Set-Cookie").expect("Set-Cookie");
    let cookie = actix_web::cookie::Cookie::parse(cookie_header.to_str().unwrap()).unwrap();

    // who am i
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/v1/authentication/whoami", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::GET], &response);
    assert_access_control_allow_headers(&response);

    let response = client
        .request(
            Method::GET,
            &format!("{}/api/v1/authentication/whoami", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<HashMap<String, String>>()
        .await
        .expect("Failed to parse body.");
    assert_eq!(result.get("username").unwrap(), "test-username");

    // logout
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/v1/authentication/logout", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::POST], &response);
    assert_access_control_allow_headers(&response);

    let response = client
        .request(
            Method::POST,
            &format!("{}/api/v1/authentication/logout", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);

    // log in
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/v1/authentication/login", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::POST], &response);
    assert_access_control_allow_headers(&response);

    let mut map = HashMap::new();
    map.insert("username", "test-username");
    map.insert("password", "test-password");
    let response = client
        .post(&format!(
            "{}/api/v1/authentication/login",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let cookie_header = response.headers().get("Set-Cookie").expect("Set-Cookie");
    let cookie = actix_web::cookie::Cookie::parse(cookie_header.to_str().unwrap()).unwrap();

    // who am i
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/v1/authentication/whoami", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::GET], &response);
    assert_access_control_allow_headers(&response);

    let response = client
        .request(
            Method::GET,
            &format!("{}/api/v1/authentication/whoami", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<HashMap<String, String>>()
        .await
        .expect("Failed to parse body.");
    assert_eq!(result.get("username").unwrap(), "test-username");

    // logout
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/v1/authentication/logout", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::POST], &response);
    assert_access_control_allow_headers(&response);

    let response = client
        .request(
            Method::POST,
            &format!("{}/api/v1/authentication/logout", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
}

#[actix_rt::test]
async fn should_make_unauthenticated_requests() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let client = reqwest::Client::new();

    // login
    let mut map = HashMap::new();
    map.insert("username", "admin");
    map.insert("password", "password");
    let response = client
        .post(&format!(
            "{}/api/v1/authentication/login",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);

    // logout
    let response = client
        .post(&format!(
            "{}/api/v1/authentication/logout",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);

    // whoami
    let response = client
        .get(&format!(
            "{}/api/v1/authentication/whoami",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);
}

#[actix_rt::test]
async fn should_not_be_able_to_sign_up_using_existing_account() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let client = reqwest::ClientBuilder::new().build().unwrap();

    // sign up
    let mut map = HashMap::new();
    map.insert("username", "test-username");
    map.insert("password", "test-password");
    let response = client
        .post(&format!(
            "{}/api/v1/authentication/signup",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    assert!(response.headers().get("Set-Cookie").is_some());

    // sign up - fails
    let mut map = HashMap::new();
    map.insert("username", "test-username");
    map.insert("password", "test-password");
    let response = client
        .post(&format!(
            "{}/api/v1/authentication/signup",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);
    assert!(response.headers().get("Set-Cookie").is_none());
}
