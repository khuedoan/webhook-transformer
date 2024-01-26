use axum::{
    routing::post,
    Json, Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route(
            "/",
            post(|payload: Json<serde_json::Value>| async move {
                payload
            }),
        )
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::util::ServiceExt;
    use tokio::fs;

    async fn run_example(name: &str) {
        let app = crate::app();

        let input = fs::read_to_string(format!("examples/{}/input.json", name)).await.unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(input))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let output = fs::read_to_string(format!("examples/{}/output.json", name)).await.unwrap();
        let expected_response: Value = serde_json::from_str(&output).unwrap();

        assert_eq!(body, expected_response);
    }

    #[tokio::test]
    async fn noop() {
        run_example("noop").await;
    }

    #[tokio::test]
    async fn basic() {
        run_example("basic").await;
    }
}
