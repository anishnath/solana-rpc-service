use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use serde_json::{Result, Value};

fn main() -> Result<()> {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let start_slot = 117894315;
    let end_slot = 117894318;
    let blocks: Vec<Value> = rpc_client
        .get_blocks(start_slot, Some(end_slot))
        .unwrap_or_else(|err| panic!("Failed to get blocks: {:?}", err));

    for block in blocks {
        let blockhash = block["blockhash"].as_str().expect("Block hash not found");
        println!("{}", blockhash);
    }

    Ok(())
}
