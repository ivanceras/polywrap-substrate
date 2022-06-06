use crate::{
    error::Error,
    types::{
        account_info::AccountInfo,
        extrinsic_params::{
            BaseExtrinsicParams,
            BaseExtrinsicParamsBuilder,
            ExtrinsicParams,
            GenericExtra,
            SignedPayload,
        },
        extrinsics::{
            GenericAddress,
            UncheckedExtrinsicV4,
        },
    },
    Api,
    Metadata,
};
use codec::Encode;
use sp_core::{
    crypto::Pair,
    storage::StorageKey,
    H256,
};
use sp_runtime::{
    traits::IdentifyAccount,
    AccountId32,
    MultiSignature,
    MultiSigner,
};
use sp_version::RuntimeVersion;
use std::fmt;

impl Api {
    pub fn signer_account<P>(&self, signer: &P) -> AccountId32
    where
        P: Pair,
        MultiSigner: From<P::Public>,
    {
        let multi_signer = MultiSigner::from(signer.public());
        multi_signer.into_account()
    }

    pub async fn get_nonce<P>(&self, signer: &P) -> Result<u32, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
    {
        let signer_account = self.signer_account(signer);
        let account_info = self.get_account_info(signer_account).await?;
        match account_info {
            None => Ok(0),
            Some(account_info) => Ok(account_info.nonce),
        }
    }

    pub async fn get_account_info(
        &self,
        account_id: AccountId32,
    ) -> Result<Option<AccountInfo>, Error> {
        let metadata: Metadata =
            self.fetch_metadata().await?.expect("cant get a metadata");
        let storage_key: StorageKey = metadata
            .storage_map_key::<AccountId32>("System", "Account", account_id)?;
        self.fetch_storage_by_key_hash(storage_key).await
    }

    pub fn unsigned_extrinsic<Call>(
        &self,
        call: Call,
    ) -> UncheckedExtrinsicV4<Call>
    where
        Call: Encode,
    {
        UncheckedExtrinsicV4::new_unsigned(call)
    }

    pub async fn compose_extrinsics<P, Params, Tip, Call>(
        &self,
        signer: Option<P>,
        call: Call,
        head_hash: Option<H256>,
        extrinsic_params: Option<Params::OtherParams>,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: Pair,
        Params: ExtrinsicParams<OtherParams = BaseExtrinsicParamsBuilder<Tip>>,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        u128: From<Tip>,
        Tip: Encode + Default,
        Call: Encode + Clone + fmt::Debug,
    {
        match signer {
            None => Ok(self.unsigned_extrinsic(call)),
            Some(signer) => {
                let runtime_version: RuntimeVersion = self
                    .fetch_runtime_version()
                    .await?
                    .expect("cant get a runtime version");
                let genesis_hash: H256 = self
                    .fetch_genesis_hash()
                    .await?
                    .expect("cant get a genesis hash");

                let nonce = self.get_nonce(&signer).await?;

                let other_params = extrinsic_params.unwrap_or_default();
                let params: BaseExtrinsicParams<Tip> =
                    BaseExtrinsicParams::new(nonce, other_params);
                let extra = GenericExtra::from(params);
                println!("call: {:?}", call);
                let head_or_genesis_hash = match head_hash {
                    Some(hash) => hash,
                    None => genesis_hash,
                };
                let raw_payload = SignedPayload::from_raw(
                    call.clone(),
                    extra.clone(),
                    (
                        runtime_version.spec_version,
                        runtime_version.transaction_version,
                        genesis_hash,
                        head_or_genesis_hash,
                        (),
                        (),
                        (),
                    ),
                );
                let signature: P::Signature =
                    raw_payload.using_encoded(|payload| signer.sign(payload));
                let multi_signer = MultiSigner::from(signer.public());
                let multi_signature = MultiSignature::from(signature);
                Ok(UncheckedExtrinsicV4::new_signed(
                    call,
                    GenericAddress::from(multi_signer.into_account()),
                    multi_signature,
                    extra,
                ))
            }
        }
    }
}
