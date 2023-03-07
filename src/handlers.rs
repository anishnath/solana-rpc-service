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
    let endpoint = get_network_from_request(&info);

    let rpc_client = Arc::new(rpc::create_rpc_client(endpoint));
    // Get the current epoch
    let current_epoch = rpc::get_current_epoch(rpc_client).await;

    HttpResponse::Ok().json(current_epoch)
}

fn get_network_from_request(info: &EpochInfoRequest) -> String {
    match &info.network {
        Some(network) => match network.as_str() {
            "devnet" => "https://api.devnet.solana.com".to_string(),
            "testnet" => "https://api.testnet.solana.com".to_string(),
            "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
            _ => network.to_string(), // Custom network
        },
        None => "https://api.mainnet-beta.solana.com".to_string(), // Default network if not provided
    }
}