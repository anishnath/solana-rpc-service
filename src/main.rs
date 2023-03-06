use std::env;
use solana_client::rpc_client::RpcClient;

fn get_current_epoch(rpc_client: &RpcClient) -> u64 {
    // Get the current epoch
    let epoch_info = rpc_client.get_epoch_info().unwrap();
    epoch_info.epoch
}

fn main() {
    // Get the Solana network from the command-line argument
    let args: Vec<String> = env::args().collect();
    let network = args.get(1).unwrap_or(&"mainnet".to_string());

    // Choose a Solana network to connect to based on the command-line argument
    let endpoint = match network.as_str() {
        "devnet" => "https://api.devnet.solana.com".to_string(),
        "testnet" => "https://api.testnet.solana.com".to_string(),
        "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
        _ => network.clone(), // Custom network
    };
    let rpc_client = RpcClient::new(endpoint);

    let current_epoch = get_current_epoch(&rpc_client);
    println!("Current epoch on {} is: {}", network, current_epoch);
}
