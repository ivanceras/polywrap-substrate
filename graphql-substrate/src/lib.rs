#![deny(warnings)]
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
pub use model::QueryRoot;
use node_template_runtime::Block;

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

pub struct ChainApi {}

impl ChainApi {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ChainApi {}
    }

    pub async fn block(&self, number: u32) -> Option<BlockDetail> {
        let block = mycelium::fetch_block::<Block>(number).await.ok().flatten();

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

    pub async fn metadata(&self) -> Option<String> {
        let metadata = mycelium::fetch_runtime_metadata()
            .await
            .expect("must not error");
        dbg!(&metadata);
        serde_json::to_string(&metadata).ok()
    }
}
