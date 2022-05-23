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

    async fn metadata<'a>(&self, ctx: &Context<'a>) -> Option<String> {
        ctx.data_unchecked::<ChainApi>().metadata()
    }
}
