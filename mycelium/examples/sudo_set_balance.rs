#![allow(warnings)]
use mycelium::{
    types::{
        extrinsic_params::{
            PlainTip,
            PlainTipExtrinsicParams,
        },
    },
    Api,
    Metadata,
};
use sp_keyring::AccountKeyring;
use mycelium::types::extrinsics::GenericAddress;
use codec::Compact;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let sudoer: sp_core::sr25519::Pair =
        AccountKeyring::Alice.pair();


    let to = AccountKeyring::Bob.to_account_id();

    let api = Api::new("http://localhost:9933");
    let metadata: Metadata =
        api.fetch_metadata().await?.expect("cant get a metadata");

    let balance_pallet = metadata.pallet("Balances")?;
    let set_balance_call_index = balance_pallet
        .calls
        .get("set_balance")
        .expect("function name does not exist");

    //u128::MAX = 340_282_366_920_938_463_463_374_607_431_768_211_455u128
    // 1Yunit = 1_000_000_000_000_000_000_000_000_000_000_000_000_u128
    // 1Munit = 1_000_000_000_000_000_000_u128
    let balance_call = ([balance_pallet.index, *set_balance_call_index], GenericAddress::Id(to),
        Compact(42_000_000_000_000_000_000_u128),
        Compact(42_000_000_000_000_000_000_u128),
        );


    let sudo_pallet = metadata.pallet("Sudo")?;
    let sudo_call_index = sudo_pallet
        .calls
        .get("sudo")
        .expect("function name does not exist");
    let sudo_call = ([sudo_pallet.index, *sudo_call_index as u8], balance_call);

    let xt = api.compose_extrinsics::<
        sp_core::sr25519::Pair,
        PlainTipExtrinsicParams,
        PlainTip,
        ([u8;2],([u8; 2], GenericAddress, Compact<u128>, Compact<u128>)),
    >(Some(sudoer), sudo_call, None)
    .await?;

    let encoded = xt.hex_encode();
    println!("encoded: {}", encoded);
    let result = api.author_submit_extrinsic(&encoded).await?;
    println!("result: {:?}", result);
    Ok(())
}

