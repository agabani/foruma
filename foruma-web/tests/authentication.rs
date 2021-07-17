use reqwest::{Method, Response};
use std::collections::HashMap;

mod graphql;
mod test_server;

fn assert_preflight_access_control_allow_headers(methods: &[Method], response: &Response) {
    assert_eq!(
        response
            .headers()
            .get("Access-Control-Allow-Headers")
            .expect("Access-Control-Allow-Headers"),
        "content-type"
    );

    for method in methods {
        assert!(response
            .headers()
            .get("Access-Control-Allow-Methods")
            .expect("Access-Control-Allow-Methods")
            .to_str()
            .unwrap()
            .contains(method.as_str()))
    }
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
        .header("Access-Control-Request-Headers", "content-type")
        .header("Access-Control-Request-Method", "POST")
        .header("Origin", "http://localhost:8080")
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_preflight_access_control_allow_headers(&[Method::GET, Method::POST], &response);
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
    let mut map = HashMap::new();
    map.insert(
        "query",
        r#"
{
    currentAccount {
        id
        username
    }
}
"#,
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<HashMap<String, HashMap<String, HashMap<String, String>>>>()
        .await
        .expect("Failed to parse body.");
    assert_eq!(
        result
            .get("data")
            .unwrap()
            .get("currentAccount")
            .unwrap()
            .get("username")
            .unwrap(),
        "test-username"
    );

    // logout
    let mut map = HashMap::new();
    map.insert(
        "query",
        r#"
mutation {
  logoutCurrentAccount
}
"#,
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<graphql::Response<HashMap<String, bool>>>()
        .await
        .expect("Failed to parse body.");
    assert!(result
        .data
        .as_ref()
        .unwrap()
        .get("logoutCurrentAccount")
        .unwrap());
    assert!(result.errors.is_none());

    // log in
    let mut map = HashMap::new();
    map.insert(
        "query",
        format!(
            r#"
mutation {{
  login(input: {{ username: "{}", password: "{}" }})
}}
"#,
            "test-username", "test-password"
        ),
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);

    let cookie_header = &response
        .headers()
        .get("Set-Cookie")
        .expect("Set-Cookie")
        .clone();
    let cookie = actix_web::cookie::Cookie::parse(cookie_header.to_str().unwrap()).unwrap();

    let result = &response
        .json::<graphql::Response<HashMap<String, bool>>>()
        .await
        .expect("Failed to parse body.");
    assert!(result.errors.is_none());
    assert!(result.data.as_ref().unwrap().get("login").unwrap());

    // who am i
    let mut map = HashMap::new();
    map.insert(
        "query",
        r#"
{
    currentAccount {
        id
        username
    }
}
"#,
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<HashMap<String, HashMap<String, HashMap<String, String>>>>()
        .await
        .expect("Failed to parse body.");
    assert_eq!(
        result
            .get("data")
            .unwrap()
            .get("currentAccount")
            .unwrap()
            .get("username")
            .unwrap(),
        "test-username"
    );

    // logout
    let mut map = HashMap::new();
    map.insert(
        "query",
        r#"
mutation {
  logoutCurrentAccount
}
"#,
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<graphql::Response<HashMap<String, bool>>>()
        .await
        .expect("Failed to parse body.");
    assert!(result
        .data
        .as_ref()
        .unwrap()
        .get("logoutCurrentAccount")
        .unwrap());
    assert!(result.errors.is_none());
}

#[actix_rt::test]
async fn should_make_unauthenticated_requests() {
    let test_server = test_server::TestServer::spawn(&[]).await;
    let client = reqwest::Client::new();

    // login
    let mut map = HashMap::new();
    map.insert(
        "query",
        format!(
            r#"
mutation {{
  login(input: {{ username: "{}", password: "{}" }})
}}
"#,
            "admin", "password"
        ),
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<graphql::Response<HashMap<String, bool>>>()
        .await
        .expect("Failed to parse body.");
    assert!(result.data.is_none());
    assert_eq!(
        result
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
    let mut map = HashMap::new();
    map.insert(
        "query",
        r#"
mutation {
  logoutCurrentAccount
}
"#,
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<graphql::Response<HashMap<String, bool>>>()
        .await
        .expect("Failed to parse body.");
    assert!(result.data.is_none());
    assert_eq!(
        result
            .errors
            .as_ref()
            .unwrap()
            .first()
            .as_ref()
            .unwrap()
            .message,
        "unauthenticated"
    );

    // whoami
    let mut map = HashMap::new();
    map.insert(
        "query",
        r#"
{
    currentAccount {
        id
        username
    }
}
"#,
    );
    let response = client
        .request(
            Method::POST,
            &format!("{}/api/graphql/", test_server.address),
        )
        .header("Origin", "http://localhost:8080")
        .json(&map)
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status().as_u16(), 200);
    assert_access_control_allow_headers(&response);
    let result = response
        .json::<HashMap<String, HashMap<String, Option<HashMap<String, String>>>>>()
        .await
        .expect("Failed to parse body.");
    assert!(result
        .get("data")
        .as_ref()
        .unwrap()
        .get("currentAccount")
        .unwrap()
        .is_none());
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
