#![deny(warnings)]
use mycelium::{
    types::extrinsic_params::{
        PlainTip,
        PlainTipExtrinsicParams,
    },
    Api,
    Metadata,
};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let signer: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let api = Api::new("http://localhost:9933");
    let metadata: Metadata =
        api.fetch_metadata().await?.expect("cant get a metadata");
    let pallet = metadata.pallet("TemplateModule")?;
    let call_index = pallet
        .calls
        .get("do_something")
        .expect("function name does not exist");
    let call = ([pallet.index, *call_index], (200u32));

    let xt = api.compose_extrinsics::<
        sp_core::sr25519::Pair,
        PlainTipExtrinsicParams,
        PlainTip,
        ([u8; 2], u32),
    >(Some(signer), call, None)
    .await?;

    let encoded = xt.hex_encode();
    println!("encoded: {}", encoded);
    let result = api.author_submit_extrinsic(&encoded).await?;
    println!("result: {:?}", result);
    Ok(())
}
