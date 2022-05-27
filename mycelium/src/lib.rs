use serde::{Deserialize, Serialize};
use utils::FromHexStr;
mod utils;
use frame_metadata::RuntimeMetadataPrefixed;
use sp_core::Decode;

#[derive(Serialize, Deserialize)]
pub struct JsonReq {
    id: usize,
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResult {
    id: usize,
    jsonrpc: String,
    result: serde_json::Value,
}

// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata"}' http://localhost:9933/
pub async fn fetch_metadata() -> Result<(), reqwest::Error> {
    let param: Vec<usize> = vec![];
    let result = json_request("state_getMetadata", param).await?;
    let result_str = result.result.as_str().expect("must be a str");
    let meta = Vec::from_hex(result_str).expect("must decode hex");
    let metadata = RuntimeMetadataPrefixed::decode(&mut meta.as_slice()).expect("must not error");
    dbg!(&metadata);
    Ok(())
}

// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
pub async fn fetch_rpc_methods() -> Result<JsonResult, reqwest::Error> {
    let param: Vec<usize> = vec![];
    json_request("rpc_methods", param).await
}

pub async fn fetch_block_hash(n: usize) -> Result<JsonResult, reqwest::Error> {
    json_request("chain_getBlockHash", vec![n]).await
}

pub async fn fetch_block_with_hash(hash: &str) -> Result<JsonResult, reqwest::Error> {
    json_request("chain_getBlock", vec![hash]).await
}

async fn json_request<P: Serialize>(method: &str, params: P) -> Result<JsonResult, reqwest::Error> {
    let param = JsonReq {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: method.to_string(),
        params: serde_json::to_value(params).expect("can not convert to json value"),
    };
    let result: JsonResult = reqwest::Client::new()
        .post("http://localhost:9933")
        .json(&param)
        .send()
        .await?
        .json()
        .await?;
    dbg!(&result);

    Ok(result)
}

#[cfg(test)]
mod tests {
    #![cfg(not(target_arch = "wasm32"))]
    use super::*;

    #[tokio::test]
    async fn test1() {
        println!("fetching metada...");
        let result = fetch_metadata().await.expect("must not error");
        dbg!(&result);
        panic!();
    }

    #[tokio::test]
    async fn test2() {
        println!("fetching rpc methods...");
        let result = fetch_rpc_methods().await.expect("must not error");
        dbg!(&result);
        panic!();
    }

    #[tokio::test]
    async fn block_hashes() {
        let result = fetch_block_hash(0).await.expect("must get genesis block");
        dbg!(&result);
        panic!();
    }

    #[tokio::test]
    async fn blocks() {
        let result = fetch_block_with_hash(
            "0x9aa34b52a90619e71d70b51bd22b958b5558c85b337623a9e65295448a2e3c6d",
        )
        .await
        .expect("must get block");
        dbg!(&result);
        panic!();
    }
}
