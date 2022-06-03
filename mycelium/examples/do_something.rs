use codec::{Decode, Encode};
use mycelium::types::account_info::AccountInfo;
use mycelium::types::extrinsic_params::BaseExtrinsicParams;
use mycelium::types::extrinsic_params::BaseExtrinsicParamsBuilder;
use mycelium::types::extrinsic_params::ExtrinsicParams;
use mycelium::types::extrinsic_params::GenericExtra;
use mycelium::types::extrinsic_params::PlainTip;
use mycelium::types::extrinsic_params::PlainTipExtrinsicParams;
use mycelium::types::extrinsic_params::SignedPayload;
use mycelium::types::extrinsics::GenericAddress;
use mycelium::types::extrinsics::UncheckedExtrinsicV4;
use mycelium::{Api, Metadata};
use node_template_runtime::AccountId;
use sp_core::storage::StorageKey;
use sp_core::Pair;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use sp_runtime::generic::Era;
use sp_runtime::testing::sr25519;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::MultiSignature;
use sp_runtime::MultiSigner;
use sp_version::RuntimeVersion;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    execute_extrinsics().await?;
    Ok(())
}

async fn execute_extrinsics() -> Result<(), mycelium::Error> {
    let signer: Option<sp_core::sr25519::Pair> = Some(AccountKeyring::Alice.pair());
    //let signer: Option<sp_core::sr25519::Pair> = None;
    let xt =
        compose_extrinsics::<sp_core::sr25519::Pair, PlainTipExtrinsicParams, PlainTip>(signer)
            .await?;
    let encoded = xt.hex_encode();
    println!("encoded: {}", encoded);
    Ok(())
}

pub async fn compose_extrinsics<P, Params, Tip>(
    signer: Option<P>,
) -> Result<UncheckedExtrinsicV4<([u8; 2], u32)>, mycelium::Error>
where
    P: Pair,
    Params: ExtrinsicParams<OtherParams = BaseExtrinsicParamsBuilder<Tip>>,
    MultiSigner: From<P::Public>,
    MultiSignature: From<P::Signature>,
    u128: From<Tip>,
    Tip: Encode + Default,
{
    println!("composing extrinisics..");
    let api = Api::new("http://localhost:9933");
    let runtime_version: RuntimeVersion = api
        .fetch_runtime_version()
        .await?
        .expect("cant get a runtime version");
    let genesis_hash: H256 = api
        .fetch_genesis_hash()
        .await?
        .expect("cant get a genesis hash");
    let metadata: Metadata = api.fetch_metadata().await?.expect("cant get a metadata");

    let extrinsic_params: Option<Params::OtherParams> = None;

    let pallet = metadata.pallet("TemplateModule")?;
    let call_index = pallet
        .calls
        .get("do_something")
        .expect("function name does not exist");
    let call = ([pallet.index, *call_index as u8], (200u32));

    println!("call: {:?}", call);

    let xt: UncheckedExtrinsicV4<([u8; 2], u32)> = if let Some(signer) = signer.as_ref() {
        let multi_signer = MultiSigner::from(signer.public());
        let account_id: AccountId = multi_signer.into_account();
        let storage_key: StorageKey = metadata
            .storage_map_key::<AccountId>("System", "Account", account_id)
            .unwrap();
        let account_info: AccountInfo = api.fetch_storage_by_key_hash(storage_key).await?.unwrap();
        let nonce: u32 = account_info.nonce;
        println!("nonce: {}", nonce);

        println!("got a signer..");
        let other_params = extrinsic_params.unwrap_or_default();
        let params: BaseExtrinsicParams<Tip> = BaseExtrinsicParams::new(nonce, other_params);
        let extra = GenericExtra::from(params);
        println!("extra: {:?}", extra);
        let raw_payload = SignedPayload::from_raw(
            call,
            extra.clone(),
            (
                runtime_version.spec_version,
                runtime_version.transaction_version,
                genesis_hash,
                genesis_hash,
                (),
                (),
                (),
            ),
        );
        let signature: P::Signature = raw_payload.using_encoded(|payload| signer.sign(payload));
        let multi_signer: MultiSigner = signer.public().into();
        let multi_signature: MultiSignature = signature.into();
        UncheckedExtrinsicV4::new_signed(
            call,
            GenericAddress::from(multi_signer.into_account()),
            multi_signature,
            extra,
        )
    } else {
        UncheckedExtrinsicV4 {
            signature: None,
            function: call,
        }
    };

    println!("xt: {:#?}", xt);
    let encoded = xt.hex_encode();
    let result = api.author_submit_and_watch_extrinsic(&encoded).await?;
    println!("result: {:?}", result);
    Ok(xt)
}
