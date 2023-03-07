use std::env;
use solana_client::rpc_client::RpcClient;

fn create_rpc_client(network: &str) -> RpcClient {
    // Choose a Solana network to connect to based on the command-line argument
    let endpoint = match network {
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        _ => network.to_string(), // Custom network
    };
    RpcClient::new(endpoint)
}

fn get_current_epoch(network: &str) -> u64 {
    // Create a RPC client for the specified network
    let rpc_client = create_rpc_client(network);

    // Get the current epoch
    let epoch_info = rpc_client.get_epoch_info().unwrap();
    epoch_info.epoch
}

fn main() {
    // Get the Solana network from the command-line argument
    let args: Vec<String> = env::args().collect();
    let network = args.get(1).unwrap_or(&"mainnet".to_string()).to_string();

    // Get the current epoch for the specified network
    let current_epoch = get_current_epoch(&network);
    println!("Current epoch on {} is: {}", network, current_epoch);
}