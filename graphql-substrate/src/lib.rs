mod model;

use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use model::QueryRoot;
use slab::Slab;

use node_template_runtime::{Block, Header};
use sp_core::sr25519;
use sp_runtime::generic::SignedBlock as SignedBlockG;
use std::sync::mpsc::channel;
use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, PlainTipExtrinsicParams};

pub type StarWarsSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct BlockDetail {
    id: String,
    name: String,
}

pub struct ChainApi {}

impl ChainApi {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ChainApi {}
    }

    pub fn block(&self, number: u32) -> Option<BlockDetail> {
        let client = WsRpcClient::new("ws://127.0.0.1:9944");
        let api = Api::<sr25519::Pair, _, PlainTipExtrinsicParams>::new(client).unwrap();

        let head = api.get_finalized_head().unwrap().unwrap();
        println!("head: {:#?}", head);
        let block = api
            .get_block_by_num::<Block>(Some(number))
            .unwrap()
            .unwrap();

        println!("Genesis block: \n {:#?} \n", block);
        Some(BlockDetail {
            id: block.header.parent_hash.to_string(),
            name: block.header.number.to_string(),
        })
    }
}
