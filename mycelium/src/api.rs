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
use sp_version::RuntimeVersion;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

mod storage_api;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonReq {
    id: usize,
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub async fn fetch_runtime_metadata(&self) -> Result<Option<RuntimeMetadataPrefixed>, Error> {
        let value = self.json_request_value("state_getMetadata", ()).await?;
        match value {
            Some(value) => {
                let value_str = value
                    .as_str()
                    .expect("Expecting a string value on the result");
                let data = Vec::from_hex(value_str)?;
                let rt_metadata = RuntimeMetadataPrefixed::decode(&mut data.as_slice())?;
                Ok(Some(rt_metadata))
            }
            None => Ok(None),
        }
    }

    /// Get the metadata of the substrate chain
    pub async fn fetch_metadata(&self) -> Result<Option<Metadata>, Error> {
        let rt_metadata = self.fetch_runtime_metadata().await?;
        match rt_metadata {
            Some(rt_metadata) => {
                let metadata = Metadata::try_from(rt_metadata)?;
                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    // curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
    pub async fn fetch_rpc_methods(&self) -> Result<Option<Vec<String>>, Error> {
        let value = self.json_request_value("rpc_methods", ()).await?;
        match value {
            Some(value) => {
                let methods: Vec<String> = serde_json::from_value(value["methods"].clone())?;
                Ok(Some(methods))
            }
            None => Ok(None),
        }
    }

    /// return the block hash of block number `n`
    pub async fn fetch_block_hash(&self, n: u32) -> Result<Option<H256>, Error> {
        let value = self
            .json_request_value("chain_getBlockHash", vec![n])
            .await?;

        match value {
            Some(value) => {
                let hash = value.as_str().map(|s| H256::from_hex(s)).transpose()?;
                Ok(hash)
            }
            None => Ok(None),
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

    pub async fn fetch_genesis_hash(&self) -> Result<Option<H256>, Error> {
        self.fetch_block_hash(0).await
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
        let value = self
            .json_request_value("chain_getBlock", vec![hash])
            .await?;
        match value {
            Some(value) => Ok(serde_json::from_value(value)?),
            None => Ok(None),
        }
    }

    pub async fn fetch_runtime_version(&self) -> Result<Option<RuntimeVersion>, Error> {
        let version = self
            .json_request_value("state_getRuntimeVersion", ())
            .await?;
        println!("version: {:#?}", version);
        match version {
            Some(version) => {
                let rt_version: RuntimeVersion = serde_json::from_value(version)?;
                println!("rt_version: {:#?}", rt_version);
                Ok(Some(rt_version))
            }
            None => Ok(None),
        }
    }

    pub async fn author_submit_and_watch_extrinsic(
        &self,
        hex_extrinsic: &str,
    ) -> Result<Option<serde_json::Value>, Error> {
        self.json_request_value("author_submitExtrinsic", vec![hex_extrinsic])
            .await
    }

    /// Make a rpc request and return the result.result if it has value
    async fn json_request_value<P: Serialize>(
        &self,
        method: &str,
        params: P,
    ) -> Result<Option<serde_json::Value>, Error> {
        let result = self.json_request(method, params).await?;
        if result.result.is_null() {
            Ok(None)
        } else {
            Ok(Some(result.result))
        }
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
        println!("param: {:#?}", param);
        let result_value: serde_json::Value = reqwest::Client::new()
            .post(&self.url)
            .json(&param)
            .send()
            .await?
            .json()
            .await?;
        println!("http result_value: {:#?}", result_value);
        let result: JsonResult = serde_json::from_value(result_value)?;
        println!("http result: {:#?}", result);

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
    async fn versions() {
        let version = Api::new("http://localhost:9933")
            .fetch_runtime_version()
            .await;
        dbg!(version);
        panic!();
    }

    #[tokio::test]
    async fn block_hashes() {
        let version = Api::new("http://localhost:9933")
            .json_request_value("state_getRuntimeVersion", ())
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
