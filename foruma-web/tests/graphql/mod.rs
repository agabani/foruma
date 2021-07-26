use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<Error>>,
}

#[derive(serde::Deserialize)]
pub struct Error {
    pub message: String,
    pub locations: Vec<Location>,
    pub path: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

pub struct GraphQLClient {
    client: reqwest::Client,
    cookie_header: Option<String>,
    url: String,
}

impl GraphQLClient {
    pub fn new(address: &str) -> Self {
        let mut header_map = reqwest::header::HeaderMap::new();
        header_map.insert(
            reqwest::header::ORIGIN,
            "http://localhost:8080".parse().unwrap(),
        );
        header_map.insert(
            reqwest::header::USER_AGENT,
            "integration-test".parse().unwrap(),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(header_map)
            .build()
            .unwrap();

        let url = format!("{}/api/graphql/", address);

        Self {
            client,
            cookie_header: None,
            url,
        }
    }

    pub async fn current_account(
        &mut self,
    ) -> Response<HashMap<String, Option<HashMap<String, String>>>> {
        let mut request = HashMap::new();
        request.insert(
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
        self.send(&request).await
    }

    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Response<HashMap<String, bool>> {
        let mut request = HashMap::new();
        request.insert(
            "query",
            format!(
                r#"
mutation {{
  login(input: {{ username: "{}", password: "{}" }})
}}
"#,
                username, password
            ),
        );
        self.send(&request).await
    }

    pub async fn logout_current_account(&mut self) -> Response<HashMap<String, bool>> {
        let mut request = HashMap::new();
        request.insert(
            "query",
            r#"
mutation {
  logoutCurrentAccount
}
"#,
        );
        self.send(&request).await
    }

    pub async fn signup(
        &mut self,
        username: &str,
        password: &str,
    ) -> Response<HashMap<String, bool>> {
        let mut request = HashMap::new();
        request.insert(
            "query",
            format!(
                r#"
mutation {{
  signup(input: {{ username: "{}", password: "{}" }})
}}
"#,
                username, password
            ),
        );
        self.send(&request).await
    }

    async fn send<
        Request: serde::ser::Serialize + ?Sized,
        Response: serde::de::DeserializeOwned,
    >(
        &mut self,
        request: &Request,
    ) -> Response {
        // preflight request
        let http_response = self
            .client
            .request(reqwest::Method::OPTIONS, &self.url)
            .header("Access-Control-Request-Headers", "content-type")
            .header("Access-Control-Request-Method", "POST")
            .send()
            .await
            .expect("Failed to send request.");

        assert_eq!(http_response.status().as_u16(), 200);
        assert_eq!(
            http_response
                .headers()
                .get("Access-Control-Allow-Headers")
                .expect("Access-Control-Allow-Headers"),
            "content-type"
        );

        // graphql request
        let mut http_request = self
            .client
            .request(reqwest::Method::POST, &self.url)
            .json(request);

        if let Some(cookie_header) = &self.cookie_header {
            http_request = http_request.header(reqwest::header::COOKIE, cookie_header);
        }

        let http_response = http_request.send().await.expect("Failed to send request.");

        assert_eq!(http_response.status().as_u16(), 200);
        assert_eq!(
            http_response
                .headers()
                .get("Access-Control-Allow-Origin")
                .expect("Access-Control-Allow-Origin"),
            "http://localhost:8080"
        );

        if let Some(cookie_header) = http_response.headers().get(reqwest::header::SET_COOKIE) {
            self.cookie_header = Some(cookie_header.to_str().unwrap().to_string());
        }

        let response = http_response
            .json::<Response>()
            .await
            .expect("Failed to parse body");

        response
    }
}
