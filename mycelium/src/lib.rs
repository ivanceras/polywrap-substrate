use serde::{Deserialize, Serialize};
use utils::FromHexStr;
mod utils;
use frame_metadata::RuntimeMetadataPrefixed;
use sp_core::Decode;

#[derive(Serialize, Deserialize)]
struct JsonReq {
    id: usize,
    jsonrpc: String,
    method: String,
    params: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonResult {
    id: usize,
    jsonrpc: String,
    result: serde_json::Value,
}

// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata"}' http://localhost:9933/
async fn fetch_metadata() -> Result<(), reqwest::Error> {
    let param = JsonReq {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "state_getMetadata".to_string(),
        params: vec![],
    };
    let result: JsonResult = reqwest::Client::new()
        .post("http://localhost:9933")
        .json(&param)
        .send()
        .await?
        .json()
        .await?;
    let result_str = result.result.as_str().expect("must be a str");
    let meta = Vec::from_hex(result_str).expect("must decode hex");
    let metadata = RuntimeMetadataPrefixed::decode(&mut meta.as_slice()).expect("must not error");
    dbg!(&metadata);
    Ok(())
}

// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
async fn fetch_rpc_methods() -> Result<(), reqwest::Error> {
    let param = JsonReq {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "rpc_methods".to_string(),
        params: vec![],
    };
    let result: JsonResult = reqwest::Client::new()
        .post("http://localhost:9933")
        .json(&param)
        .send()
        .await?
        .json()
        .await?;
    dbg!(&result);

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test1() {
        println!("fetching metada...");
        fetch_metadata().await.expect("must not error");
    }

    #[tokio::test]
    async fn test2() {
        println!("fetching rpc methods...");
        fetch_rpc_methods().await.expect("must not error");
    }
}
