#![deny(warnings)]
#![allow(clippy::needless_lifetimes)]
use async_graphql::Context;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use mycelium::Api;
use node_template_runtime::Block;

pub type ChainApiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

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

pub struct ChainApi;

impl ChainApi {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ChainApi
    }

    pub async fn block(&self, number: u32) -> Result<Option<BlockDetail>, mycelium::Error> {
        let block = Api::new("http://localhost:9933")
            .fetch_block::<Block>(number)
            .await?;
        match block {
            Some(block) => Ok(Some(BlockDetail {
                number: block.header.number.to_string(),
                header: Header {
                    parent_hash: block.header.parent_hash.to_string(),
                    state_root: block.header.state_root.to_string(),
                    extrinsics_root: block.header.extrinsics_root.to_string(),
                },
            })),
            None => Ok(None),
        }
    }

    pub async fn metadata(&self) -> Result<Option<mycelium::Metadata>, mycelium::Error> {
        Api::new("http://localhost:9933").fetch_metadata().await
    }

    pub async fn rpc_methods(&self) -> Result<Vec<String>, mycelium::Error> {
        Api::new("http://localhost:9933").fetch_rpc_methods().await
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn block<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "the block number")] number: u32,
    ) -> Result<Option<BlockDetail>, mycelium::Error> {
        ctx.data_unchecked::<ChainApi>().block(number).await
    }

    async fn metadata<'a>(
        &self,
        ctx: &Context<'a>,
    ) -> Result<Option<mycelium::Metadata>, mycelium::Error> {
        ctx.data_unchecked::<ChainApi>().metadata().await
    }

    async fn rpc_methods<'a>(&self, ctx: &Context<'a>) -> Result<Vec<String>, mycelium::Error> {
        ctx.data_unchecked::<ChainApi>().rpc_methods().await
    }
}
