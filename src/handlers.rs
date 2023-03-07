use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::rpc;
//use crate::rpc::RpcClient;
use crate::rpc::rpc_client::RpcClient;


#[derive(Debug, Serialize, Deserialize)]
pub struct EpochInfoRequest {
    pub network: Option<String>,
}

pub async fn get_epoch_info(info: web::Json<EpochInfoRequest>) -> HttpResponse {
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
    let rpc_client = Arc::new(rpc::create_rpc_client(endpoint));
    // Get the current epoch
    let current_epoch = rpc::get_current_epoch(rpc_client).await;

    println!("{}",current_epoch);

    HttpResponse::Ok().json(current_epoch)
}