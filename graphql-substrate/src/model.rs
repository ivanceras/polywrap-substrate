#![allow(clippy::needless_lifetimes)]

use super::ChainApi;
use crate::BlockDetail;
use async_graphql::{Context, Object};

pub struct Block(BlockDetail);

/// A humanoid creature in the Star Wars universe.
#[Object]
impl Block {
    /// The id of the block.
    async fn id(&self) -> &String {
        &self.0.id
    }

    /// The name of the block.
    async fn name(&self) -> &String {
        &self.0.name
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
}
