#![allow(clippy::needless_lifetimes)]

use super::ChainApi;
use crate::{BlockDetail, Header};
use async_graphql::{Context, Object};

pub struct Block(BlockDetail);

/// A chain block
#[Object]
impl Block {
    /// The number of the block.
    async fn number(&self) -> &str {
        &self.0.number
    }

    async fn header(&self) -> &Header {
        &self.0.header
    }

    /*

    /// The parent hash of the block
    async fn parent_hash(&self) -> &str {
        &self.0.parent_hash
    }

    async fn state_root(&self) -> &str {
        &self.0.state_root
    }

    async fn extrinsics_root(&self) -> &str {
        &self.0.extrinsics_root
    }
    */
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn block<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "the block number")] number: u32,
    ) -> Option<Block> {
        ctx.data_unchecked::<ChainApi>().block(number).map(Block)
    }
}
