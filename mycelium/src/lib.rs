#![deny(warnings)]

use serde::{Deserialize, Serialize};
use utils::FromHexStr;
mod utils;
use crate::metadata::Metadata;
use frame_metadata::RuntimeMetadataPrefixed;
use serde::de::DeserializeOwned;
use sp_core::Decode;
use sp_core::H256;
use sp_runtime::generic::SignedBlock;
use sp_runtime::traits::Block;

mod metadata;
mod storage;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("decoding from hex: {0}")]
    FromHexError(#[from] hex::FromHexError),
    #[error("error decoding json: {0}")]
    JsonError(#[from] serde_json::Error),
}

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
pub async fn fetch_runtime_metadata() -> Result<RuntimeMetadataPrefixed, Error> {
    let result = json_request("state_getMetadata", ()).await?;
    let result_str = result.result.as_str().expect("must be a str");
    let data = Vec::from_hex(result_str).expect("must decode hex");
    let rt_metadata =
        RuntimeMetadataPrefixed::decode(&mut data.as_slice()).expect("must not error");
    dbg!(&rt_metadata);
    Ok(rt_metadata)
}

pub async fn fetch_metadata() -> Result<Metadata, Error> {
    let rt_metadata = fetch_runtime_metadata().await?;
    let metadata = Metadata::try_from(rt_metadata).expect("must convert");
    Ok(metadata)
}

// curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
pub async fn fetch_rpc_methods() -> Result<Vec<String>, Error> {
    let result = json_request("rpc_methods", ()).await?;
    log::info!("result: {:#?}", result);
    let methods: Vec<String> =
        serde_json::from_value(result.result["methods"].clone()).expect("must deserialize");
    log::info!("methods: {:#?}", methods);
    Ok(methods)
}

/// return the block hash of block number `n`
pub async fn fetch_block_hash(n: usize) -> Result<Option<H256>, Error> {
    let result = json_request("chain_getBlockHash", vec![n]).await?;
    log::info!("result: {:?}", result);
    let hash = result
        .result
        .as_str()
        .map(|s| H256::from_hex(s))
        .transpose()?;
    Ok(hash)
}

pub async fn fetch_block<B>(n: usize) -> Result<Option<B>, Error>
where
    B: Block + DeserializeOwned,
{
    let signed_block = fetch_signed_block(n).await?;
    Ok(signed_block.map(|sb| sb.block))
}

pub async fn fetch_signed_block<B>(n: usize) -> Result<Option<SignedBlock<B>>, Error>
where
    B: Block + DeserializeOwned,
{
    let hash = fetch_block_hash(n).await?;
    if let Some(hash) = hash {
        let block = fetch_signed_block_with_hash(hash).await?;
        Ok(block)
    } else {
        Ok(None)
    }
}

pub async fn fetch_signed_block_with_hash<B>(hash: H256) -> Result<Option<SignedBlock<B>>, Error>
where
    B: Block + DeserializeOwned,
{
    let result = json_request("chain_getBlock", vec![hash]).await?;
    log::info!("result: {}", result.result);
    let block = serde_json::from_value(result.result.clone())?;
    Ok(block)
}

async fn json_request<P: Serialize>(method: &str, params: P) -> Result<JsonResult, Error> {
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
        //panic!();
    }

    #[tokio::test]
    async fn block_hashes() {
        let version = json_request("state_getRuntimeVersion", ()).await.unwrap();
        dbg!(&version);
        let result = fetch_block_hash(0).await.expect("must get genesis block");
        dbg!(&result);
        let block: Option<node_template_runtime::Block> =
            fetch_block(0).await.expect("must get block");
        dbg!(&block);
        panic!();
    }
}
