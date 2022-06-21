use crate::{
    Error,
    Metadata,
};
pub use base_api::BaseApi;
use codec::Decode;
use delegate::delegate;
use serde::de::DeserializeOwned;
use sp_core::H256;
use sp_runtime::traits::Header;
use sp_version::RuntimeVersion;

mod base_api;
mod extrinsic_api;
mod storage_api;
mod balance_api;

/// A more complex Api which requires prefetching some fields such as Metadata, genesis_hash and
/// runtime version
pub struct Api {
    base_api: BaseApi,
    metadata: Metadata,
    genesis_hash: H256,
    runtime_version: RuntimeVersion,
}

impl Api {
    // delegte function calls to BaseApi
    delegate! {
        to self.base_api {

            #[call(fetch_finalized_head)]
            pub async fn chain_get_finalized_head(&self) -> Result<Option<H256>, Error>;

            #[call(fetch_header)]
            pub async fn chain_get_header<H>(&self, hash: H256) -> Result<Option<H>,Error>
                where H:Header + DeserializeOwned;

            #[call(submit_extrinsic)]
            pub async fn author_submit_extrinsic(
                &self,
                hex_extrinsic: &str,
            ) -> Result<Option<serde_json::Value>, Error>;

        }
    }

    pub async fn new(url: &str) -> Result<Self, Error> {
        let base_api = BaseApi::new(url);
        let metadata = match base_api.fetch_metadata().await? {
            Some(metadata) => metadata,
            None => return Err(Error::NoMetadata),
        };
        let genesis_hash = match base_api.fetch_genesis_hash().await? {
            Some(genesis_hash) => genesis_hash,
            None => return Err(Error::NoGenesisHash),
        };
        let runtime_version = match base_api.fetch_runtime_version().await? {
            Some(runtime_version) => runtime_version,
            None => return Err(Error::NoRuntimeVersion),
        };

        Ok(Self {
            base_api,
            metadata,
            genesis_hash,
            runtime_version,
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn runtime_version(&self) -> &RuntimeVersion {
        &self.runtime_version
    }

    pub fn genesis_hash(&self) -> H256 {
        self.genesis_hash
    }
}
