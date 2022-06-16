#![deny(warnings)]
#![allow(clippy::needless_lifetimes)]
use async_graphql::{
    EmptyMutation,
    EmptySubscription,
    Object,
    Schema,
    SimpleObject,
};
use mycelium::{
    Api,
    BaseApi,
};
use node_template_runtime::Block;

pub type SubstrateApiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
pub struct Header {
    parent_hash: String,
    state_root: String,
    extrinsics_root: String,
}

#[derive(SimpleObject)]
pub struct BlockDetail {
    number: String,
    header: Header,
}


pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn block(
        &self,
        #[graphql(desc = "url of substrate node endpoint")]
        url: String,
        #[graphql(desc = "the block number")]
        number: u32,
    ) -> Result<Option<BlockDetail>, mycelium::Error> {
        let block = BaseApi::new(&url)
            .fetch_block::<Block>(number)
            .await?;
        match block {
            Some(block) => {
                Ok(Some(BlockDetail {
                    number: block.header.number.to_string(),
                    header: Header {
                        parent_hash: block.header.parent_hash.to_string(),
                        state_root: block.header.state_root.to_string(),
                        extrinsics_root: block
                            .header
                            .extrinsics_root
                            .to_string(),
                    },
                }))
            }
            None => Ok(None),
        }
    }

    async fn metadata(
        &self,
        #[graphql(desc = "url of substrate node endpoint")]
        url: String,
    ) -> Result<Option<mycelium::Metadata>, mycelium::Error> {
        let api = Api::new(&url).await?;
        Ok(Some(api.metadata().clone()))
    }

    async fn rpc_methods(
        &self,
        #[graphql(desc = "url of substrate node endpoint")]
        url: String,
    ) -> Result<Option<Vec<String>>, mycelium::Error> {
        BaseApi::new(&url)
            .fetch_rpc_methods()
            .await
    }

    async fn runtime_version(
        &self,
        #[graphql(desc = "url of substrate node endpoint")]
        url: String,
    ) -> Result<Option<serde_json::Value>, mycelium::Error> {
        let api = Api::new(&url).await?;
        let version = api.runtime_version();
        Ok(Some(serde_json::to_value(version)?))
    }

    async fn genesis_hash(&self,
        #[graphql(desc = "url of substrate node endpoint")]
        url: String,
        ) -> Result<Option<String>, mycelium::Error> {
        let hash = BaseApi::new(&url)
       .fetch_genesis_hash().await?;
        Ok(hash.map(|h|h.to_string()))
    }

    async fn block_hash(&self,
        #[graphql(desc = "url of substrate node endpoint")]
        url: String,
        #[graphql(desc = "the block number")]
        number: u32,
        ) -> Result<Option<String>, mycelium::Error> {
        let hash = BaseApi::new(&url)
       .fetch_block_hash(number).await?;
        Ok(hash.map(|h|h.to_string()))
    }

}
