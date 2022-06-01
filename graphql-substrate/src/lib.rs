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

    pub async fn block(&self, number: u32) -> Option<BlockDetail> {
        let block = Api::new("http://localhost:9933")
            .fetch_block::<Block>(number)
            .await
            .ok()
            .flatten();
        block.map(|block| BlockDetail {
            number: block.header.number.to_string(),
            header: Header {
                parent_hash: block.header.parent_hash.to_string(),
                state_root: block.header.state_root.to_string(),
                extrinsics_root: block.header.extrinsics_root.to_string(),
            },
        })
    }

    pub async fn metadata(&self) -> Option<mycelium::Metadata> {
        let metadata = Api::new("http://localhost:9933")
            .fetch_metadata()
            .await
            .expect("must not error");
        dbg!(&metadata);
        Some(metadata)
    }

    pub async fn rpc_methods(&self) -> Option<Vec<String>> {
        Api::new("http://localhost:9933")
            .fetch_rpc_methods()
            .await
            .ok()
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn block<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "the block number")] number: u32,
    ) -> Option<BlockDetail> {
        ctx.data_unchecked::<ChainApi>().block(number).await
    }

    async fn metadata<'a>(&self, ctx: &Context<'a>) -> Option<mycelium::Metadata> {
        ctx.data_unchecked::<ChainApi>().metadata().await
    }

    async fn rpc_methods<'a>(&self, ctx: &Context<'a>) -> Option<Vec<String>> {
        ctx.data_unchecked::<ChainApi>().rpc_methods().await
    }
}
