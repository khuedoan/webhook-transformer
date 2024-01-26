use axum::{routing::post, Json, Router};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "localhost")]
    host: String,

    #[arg(long, default_value = "8080")]
    port: String,

    #[arg(long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(args.config)).await.unwrap();
}

fn app(config: String) -> Router {
    let jsonnet_config = fs::read_to_string(config).expect("Failed to read config file");

    Router::new().route(
        "/",
        post(|payload: Json<serde_json::Value>| async move { transform(jsonnet_config, payload) }),
    )
}

fn transform(jsonnet_config: String, payload: Json<serde_json::Value>) -> Json<serde_json::Value> {
    payload
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tokio::fs;
    use tower::util::ServiceExt;

    async fn run_example(name: &str) {
        let app = crate::app(format!("examples/{name}/input.json"));

        let input = fs::read_to_string(format!("examples/{name}/input.json"))
            .await
            .unwrap();

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

        let output = fs::read_to_string(format!("examples/{name}/output.json"))
            .await
            .unwrap();
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
