#[cfg(feature = "axum")]
#[tokio::test]
async fn test_axum_response() {
    use axum::{Router, http::StatusCode, routing::get};
    use axum_test::{TestServer, TestServerConfig};
    use hypersynthetic::prelude::*;

    // Create a simple handler that returns your HTML
    async fn handler() -> HtmlFragment {
        html!(<body><h1>"Hello, world!"</h1></body>)
    }

    // Build the router
    let app = Router::new().route("/", get(handler));

    // Create test server
    let server = TestServer::new_with_config(app, TestServerConfig::default()).unwrap();

    // Make the request
    let response = server.get("/").await;

    // Assert status
    assert_eq!(response.status_code(), StatusCode::OK);

    // Assert content type
    assert!(
        response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html"),
    );

    // Assert body
    let body = response.text();
    assert_eq!(body, "<body><h1>Hello, world!</h1></body>");
}
