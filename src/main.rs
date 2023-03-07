use actix_web::{web, App, HttpResponse, HttpServer};
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

fn create_rpc_client(endpoint: String) -> RpcClient {
    RpcClient::new(endpoint)
}

#[derive(Debug, Serialize, Deserialize)]
struct EpochInfoRequest {
    network: Option<String>,
}

fn get_current_epoch(rpc_client: Arc<RpcClient>) -> u64 {
    // Get the current epoch
    let epoch_info = rpc_client.get_epoch_info().unwrap();
    epoch_info.epoch
}

/**
async fn get_epoch_info(network: web::Path<String>) -> HttpResponse {
    // Choose a Solana network to connect to based on the request path
    let endpoint = match network.as_str() {
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        _ => network.to_string(), // Custom network
    };

    println!({},endpoint)
    let rpc_client = Arc::new(create_rpc_client(endpoint));

    // Get the current epoch
    let current_epoch = get_current_epoch(rpc_client);

    HttpResponse::Ok().json(current_epoch)
}
*/

async fn get_epoch_info(info: web::Json<EpochInfoRequest>) -> HttpResponse {
    // Get the network name from the request
    let network = match &info.network {
        Some(network) => network,
        None => "mainnet", // Default network if not provided
    };

    // Choose a Solana network to connect to based on the request path
    let endpoint = match network {
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        _ => network.to_string(), // Custom network
    };

    println!("{}",endpoint);
    let rpc_client = Arc::new(create_rpc_client(endpoint));

    // Get the current epoch
    let current_epoch = get_current_epoch(rpc_client);

    println!("{}",current_epoch);

    HttpResponse::Ok().json(current_epoch)
}

/**
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/{network}/epoch").route(web::get().to(get_epoch_info)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/epoch")
                    .route(web::post().to(get_epoch_info))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
