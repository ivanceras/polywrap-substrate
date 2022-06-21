//! This example get the values from the storage items from their respective pallets
#![deny(warnings)]
use mycelium::Api;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let api = Api::new("http://localhost:9933").await?;
    let something: Result<Option<u32>, _> =
        api.fetch_storage_value("TemplateModule", "Something").await;
    println!("something: {:?}", something);

    let total_issuance: Result<Option<u128>, _> =
        api.fetch_storage_value("Balances", "TotalIssuance").await;
    println!("total issuance: {:?}", total_issuance);
    Ok(())
}
