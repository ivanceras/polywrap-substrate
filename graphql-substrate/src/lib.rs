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
        block.map(|block| BlockDetail {
            number: block.header.number.to_string(),
            header: Header {
                parent_hash: block.header.parent_hash.to_string(),
                state_root: block.header.state_root.to_string(),
                extrinsics_root: block.header.extrinsics_root.to_string(),
            },
        })
    }

    //TODO: display the metadata object
    pub async fn metadata(&self) -> Option<String> {
        let metadata: mycelium::Metadata =
            mycelium::fetch_metadata().await.expect("must not error");
        dbg!(&metadata);
        let json = serde_json::to_string(&metadata);
        dbg!(&json);
        json.ok()
    }

    pub async fn rpc_methods(&self) -> Option<Vec<String>> {
        mycelium::fetch_rpc_methods().await.ok()
    }
}
