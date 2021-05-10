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
async fn should_make_authenticated_requests() {
    // Arrange
    let test_server = test_server::TestServer::spawn(&[]);
    let client = reqwest::ClientBuilder::new().build().unwrap();

    // Act - login - OPTION
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/authentication/login", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - login - OPTION
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::POST], &response);
    assert_access_control_allow_headers(&response);

    // Arrange - login - POST
    let mut map = HashMap::new();
    map.insert("username", "test-username");
    map.insert("password", "test-password");

    // Act - login - POST
    let response = client
        .post(&format!("{}/api/authentication/login", test_server.address))
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");

    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);

    // Assert - login - POST
    let cookie_header = response.headers().get("Set-Cookie").expect("Set-Cookie");

    // Arrange - whoami - Option
    let cookie = cookie::Cookie::parse(cookie_header.to_str().unwrap()).unwrap();

    // Act - whoami - OPTION
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/authentication/whoami", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - whoami - GET
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::GET], &response);
    assert_access_control_allow_headers(&response);

    // Act - whoami - GET
    let response = client
        .request(
            Method::GET,
            &format!("{}/api/authentication/whoami", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - whoami - GET
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);

    let result = response
        .json::<HashMap<String, String>>()
        .await
        .expect("Failed to parse body.");
    assert_eq!(result.get("username").unwrap(), "test-username");

    // Act - logout - OPTION
    let response = client
        .request(
            Method::OPTIONS,
            &format!("{}/api/authentication/logout", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - logout - OPTION
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::POST], &response);
    assert_access_control_allow_headers(&response);

    // Act - logout - POST
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/authentication/logout", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - logout - POST
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);

    let cookie_header = response.headers().get("Set-Cookie").expect("Set-Cookie");
    let cookie = cookie::Cookie::parse(cookie_header.to_str().unwrap()).unwrap();

    assert!(cookie.max_age().unwrap().is_zero());
}

#[actix_rt::test]
async fn should_make_unauthenticated_requests() {
    let test_server = test_server::TestServer::spawn(&[]);
    let client = reqwest::Client::new();

    // Arrange - login - POST
    let mut map = HashMap::new();
    map.insert("username", "admin");
    map.insert("password", "password");

    // Act - login - POST
    let response = client
        .post(&format!("{}/api/authentication/login", test_server.address))
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - login - POST
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);

    // Act - whoami - GET
    let response = client
        .get(&format!(
            "{}/api/authentication/whoami",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - whoami - GET
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);

    // Act - logout - POST
    let response = client
        .post(&format!(
            "{}/api/authentication/logout",
            test_server.address
        ))
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");

    // Assert - logout - POST
    assert_eq!(response.status().as_u16(), 401);
    assert_access_control_allow_headers(&response);
}
