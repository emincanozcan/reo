use clap::Parser;
mod node_config;
mod storage;
use axum::{
    body,
    extract::{Path, State},
    http::StatusCode,
    routing, Router,
};
use node_config::{Instance, NodeConfig};
use std::{sync::Arc, time::Duration};
use storage::{HashMapStorage, SledStorage, Storage};
use tokio::sync::RwLock;
use tokio::time::interval;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let data: SharedState = Arc::new(RwLock::new(AppState {
        storage: match args.storage.as_str() {
            "sled" => Box::new(SledStorage::new(
                args.db_name
                    .expect("DB Name required for sled")
                    .as_str(),
            )),
            "memory" => Box::new(HashMapStorage::new()),
            _ => panic!("Invalid storage type"),
        },
        node_config: NodeConfig::init(args.id),
        reqwest_client: reqwest::Client::new(),
    }));
    let app = Router::new()
        .route("/", routing::get(home))
        .route("/cache/:key", routing::get(get_key))
        .route("/cache/:key/:ttl", routing::put(set_key))
        .with_state(Arc::clone(&data));

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            println!("Invalidating old records");
            {
                data.read().await.storage.invalidate_old_records();
            }
        }
    });

    axum::Server::bind(&format!("127.0.0.1:{}", args.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
type SharedState = Arc<RwLock<AppState>>;
struct AppState {
    storage: Box<dyn Storage + Send + Sync>,
    node_config: NodeConfig,
    reqwest_client: reqwest::Client,
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub port: usize,

    // sled or memory
    #[arg(short, long)]
    pub storage: String,

    // required when storage is "sled"
    #[arg(short, long)]
    pub db_name: Option<String>,

    #[arg(short, long)]
    pub id: usize,
}

async fn home() -> &'static str {
    "REO, Distributed KV Store written in Rust!"
}

async fn get_key(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<String, (StatusCode, String)> {
    let state_read = &state.read().await;
    let config = &state_read.node_config;
    let storage = &state_read.storage;
    let redirect_to = check_redirect(&key, config);
    return match redirect_to {
        Some(instance) => {
            let client = &state_read.reqwest_client;
            let response = client
                .get(instance.address.clone() + "/cache/" + &key)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            Ok(String::from(response))
        }
        None => match storage.get_record(&key) {
            Some(value) => Ok(String::from_utf8(value).unwrap()),
            None => Err((StatusCode::NOT_FOUND, "Not found!".to_string())),
        },
    };
}

async fn set_key(
    Path((key, ttl)): Path<(String, u64)>,
    State(state): State<SharedState>,
    bytes: body::Bytes,
) -> Result<String, (StatusCode, String)> {
    let state_read = &state.read().await;
    let config = &state_read.node_config;
    let storage = &state_read.storage;
    let content = String::from_utf8(bytes.to_vec()).unwrap();

    let redirect_to = check_redirect(&key, config);
    match redirect_to {
        Some(instance) => {
            let client = &state_read.reqwest_client;
            let _ = client
                .put(instance.address.clone() + "/cache/" + &key + "/" + &ttl.to_string())
                .header("Content-Type", "application/json")
                .body(content)
                .send()
                .await
                .unwrap();
        }
        None => storage.add_record(&key, &content, ttl),
    }
    Ok("OK".to_string())
}

fn check_redirect<'a>(key: &'a String, config: &'a NodeConfig) -> Option<&'a Instance> {
    let key_as_num = key.chars().map(|c| c as usize).sum::<usize>();
    let count = config.nodes.len();
    let reminder = key_as_num % count;
    let instance = &config.nodes[reminder];
    if instance.id != config.current_node.id {
        return Some(&instance);
    }
    None
}
