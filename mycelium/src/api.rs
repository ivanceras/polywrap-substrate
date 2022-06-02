#![allow(warnings)]

use crate::error::Error;
use crate::types::metadata::Metadata;
use crate::utils::FromHexStr;
use frame_metadata::RuntimeMetadataPrefixed;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sp_core::Decode;
use sp_core::H256;
use sp_runtime::generic::SignedBlock;
use sp_runtime::traits::Block;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

mod storage_api;

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

/// Expose the substrate api decoding the results
pub struct Api {
    /// the url of the substrate node we are running the rpc call from
    url: String,
}

impl Api {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    /// Get the runtime metadata of a substrate node.
    /// This is equivalent to running the following command
    ///
    /// `curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getMetadata"}' http://localhost:9933/`
    ///
    /// Which makes an rpc call of a substrate node running locally.
    pub async fn fetch_runtime_metadata(&self) -> Result<RuntimeMetadataPrefixed, Error> {
        let result = self.json_request("state_getMetadata", ()).await?;
        let result_str = result.result.as_str().expect("must be a str");
        let data = Vec::from_hex(result_str)?;
        let rt_metadata = RuntimeMetadataPrefixed::decode(&mut data.as_slice())?;
        Ok(rt_metadata)
    }

    /// Get the metadata of the substrate chain
    pub async fn fetch_metadata(&self) -> Result<Metadata, Error> {
        let rt_metadata = self.fetch_runtime_metadata().await?;
        let metadata = Metadata::try_from(rt_metadata)?;
        Ok(metadata)
    }

    // curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
    pub async fn fetch_rpc_methods(&self) -> Result<Vec<String>, Error> {
        let result = self.json_request("rpc_methods", ()).await?;
        let methods: Vec<String> =
            serde_json::from_value(result.result["methods"].clone()).expect("must deserialize");
        Ok(methods)
    }

    /// return the block hash of block number `n`
    pub async fn fetch_block_hash(&self, n: u32) -> Result<Option<H256>, Error> {
        let result = self.json_request("chain_getBlockHash", vec![n]).await?;
        if result.result.is_null() {
            Ok(None)
        } else {
            let hash = result
                .result
                .as_str()
                .map(|s| H256::from_hex(s))
                .transpose()?;
            Ok(hash)
        }
    }

    /// Fetch a substrate block by number `n`
    pub async fn fetch_block<B>(&self, n: u32) -> Result<Option<B>, Error>
    where
        B: Block + DeserializeOwned,
    {
        let signed_block = self.fetch_signed_block(n).await?;
        Ok(signed_block.map(|sb| sb.block))
    }

    /// Fetch a substrate signed block by number `n`
    pub async fn fetch_signed_block<B>(&self, n: u32) -> Result<Option<SignedBlock<B>>, Error>
    where
        B: Block + DeserializeOwned,
    {
        let hash = self.fetch_block_hash(n).await?;
        if let Some(hash) = hash {
            let block = self.fetch_signed_block_by_hash(hash).await?;
            Ok(block)
        } else {
            Ok(None)
        }
    }

    /// Fetch a substrate block by its hash `hash`
    pub async fn fetch_signed_block_by_hash<B>(
        &self,
        hash: H256,
    ) -> Result<Option<SignedBlock<B>>, Error>
    where
        B: Block + DeserializeOwned,
    {
        let result = self.json_request("chain_getBlock", vec![hash]).await?;
        let block = serde_json::from_value(result.result.clone())?;
        Ok(block)
    }

    /// Do the actual rpc call into the substrate node using `reqwest` crate.
    /// Note: reqwest crate can run in a tokio runtime or in webassembly runtime, which is why
    /// we are able to compile this whole library into wasm.
    ///
    /// TODO: replace this with polywrap's `client.query` or `client.invoke`
    async fn json_request<P: Serialize>(
        &self,
        method: &str,
        params: P,
    ) -> Result<JsonResult, Error> {
        let param = JsonReq {
            id: 1,
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: serde_json::to_value(params)?,
        };
        let result: JsonResult = reqwest::Client::new()
            .post(&self.url)
            .json(&param)
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #![cfg(not(target_arch = "wasm32"))]
    use super::*;

    #[tokio::test]
    async fn test1() {
        println!("fetching metada...");
        let result = Api::new("http://localhost:9933").fetch_metadata().await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test2() {
        println!("fetching rpc methods...");
        let result = Api::new("http://localhost:9933").fetch_rpc_methods().await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn block_hashes() {
        let version = Api::new("http://localhost:9933")
            .json_request("state_getRuntimeVersion", ())
            .await;
        dbg!(&version);
        assert!(version.is_ok());

        let result = Api::new("http://localhost:9933").fetch_block_hash(0).await;
        dbg!(&result);
        assert!(result.is_ok());

        let block: Result<Option<node_template_runtime::Block>, _> =
            Api::new("http://localhost:9933").fetch_block(0).await;
        dbg!(&block);
        assert!(block.is_ok());
        assert!(block.unwrap().is_some());
    }
}
