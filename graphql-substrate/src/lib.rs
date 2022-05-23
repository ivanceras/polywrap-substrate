#![deny(warnings)]
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
pub use model::QueryRoot;
use node_template_runtime::Block;
use sp_core::sr25519;
use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, PlainTipExtrinsicParams};

mod model;

pub type ChainApiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct Header {
    parent_hash: String,
    state_root: String,
    extrinsics_root: String,
}

#[Object]
impl Header {
    async fn parent_hash(&self) -> &str {
        &self.parent_hash
    }

    async fn state_root(&self) -> &str {
        &self.state_root
    }

    async fn extrinsics_root(&self) -> &str {
        &self.extrinsics_root
    }
}

pub struct BlockDetail {
    number: String,
    header: Header,
}

pub struct ChainApi {
    api: Api<sr25519::Pair, WsRpcClient, PlainTipExtrinsicParams>,
}

impl ChainApi {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let client = WsRpcClient::new("ws://127.0.0.1:9944");
        let api = Api::<sr25519::Pair, _, PlainTipExtrinsicParams>::new(client).unwrap();
        ChainApi { api }
    }

    pub fn block(&self, number: u32) -> Option<BlockDetail> {
        let block = self
            .api
            .get_block_by_num::<Block>(Some(number))
            .ok()
            .flatten();

        println!("block: \n {:#?} \n", block);
        block.map(|block| BlockDetail {
            number: block.header.number.to_string(),
            header: Header {
                parent_hash: block.header.parent_hash.to_string(),
                state_root: block.header.state_root.to_string(),
                extrinsics_root: block.header.extrinsics_root.to_string(),
            },
        })
    }

    pub fn metadata(&self) -> Option<String> {
        let metadata = self.api.get_metadata().ok();
        dbg!(&metadata);
        metadata.map(|m| serde_json::to_string(&m).expect("must serialize"))
    }
}
