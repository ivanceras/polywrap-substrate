use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = IpfsClient::default();
    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => {
            println!("{}", res.hash);
            Ok(())
        }
        Err(e) => anyhow::bail!("error adding file: {}", e),
    }
}
