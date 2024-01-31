use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use clap::Parser;
use serde_json::Value;
use std::{collections::HashMap, env, fs, sync::Arc};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "localhost")]
    host: String,

    #[arg(long, default_value = "8080")]
    port: String,

    #[arg(long)]
    upstream_host: String,

    #[arg(long)]
    config: String,

    #[arg(long)]
    env: Vec<String>,
}

struct AppState {
    jsonnet_config: String,
    upstream_host: String,
    env: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .expect("failed to bind listener");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    let jsonnet_config = fs::read_to_string(args.config).expect("failed to read config file");
    // TODO for each env var, reach it and put into App state
    let shared_state = Arc::new(AppState {
        jsonnet_config,
        env: args
            .env
            .iter()
            .map(|env| {
                (
                    env.clone(),
                    env::var(env).expect("failed to read environment variable"),
                )
            })
            .collect(),
        upstream_host: args.upstream_host,
    });

    let app = Router::new()
        .route("/", post(transform_handler))
        .route("/healthz", get(healthcheck_handler))
        .with_state(shared_state);

    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck_handler() -> &'static str {
    "ok"
}

async fn transform_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let transformed_payload =
        webhook_transformer::transform(state.jsonnet_config.clone(), state.env.clone(), payload);
    let client = reqwest::Client::new();
    let _res = client
        .post(&state.upstream_host)
        .json(&transformed_payload)
        .send()
        .await
        .expect("failed to forward request to upstream");

    transformed_payload.into()
}
