#![allow(unused)]
/*
   Copyright 2019 Supercomputing Systems AG

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/

//! Primitives for substrate extrinsics.

extern crate alloc;

use crate::types::extrinsic_params::BaseExtrinsicParamsBuilder;
use crate::types::extrinsic_params::{
    BaseExtrinsicParams, ExtrinsicParams, GenericExtra, PlainTipExtrinsicParamsBuilder,
    SignedPayload,
};
use crate::Metadata;
use codec::{Decode, Encode, Error, Input};
use sp_core::crypto::Pair;
use sp_core::H256;
use sp_runtime::traits::IdentifyAccount;
pub use sp_runtime::{AccountId32 as AccountId, MultiAddress};
use sp_runtime::{MultiSignature, MultiSigner};
use sp_std::fmt;
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub type AccountIndex = u64;

pub type GenericAddress = sp_runtime::MultiAddress<AccountId, ()>;

pub type CallIndex = [u8; 2];

/// Mirrors the currently used Extrinsic format (V4) from substrate. Has less traits and methods though.
/// The SingedExtra used does not need to implement SingedExtension here.
#[derive(Clone, Eq, PartialEq)]
pub struct UncheckedExtrinsicV4<Call> {
    pub signature: Option<(GenericAddress, MultiSignature, GenericExtra)>,
    pub function: Call,
}

impl<Call> UncheckedExtrinsicV4<Call>
where
    Call: Encode,
{
    pub fn new_signed(
        function: Call,
        signed: GenericAddress,
        signature: MultiSignature,
        extra: GenericExtra,
    ) -> Self {
        UncheckedExtrinsicV4 {
            signature: Some((signed, signature, extra)),
            function,
        }
    }

    pub fn hex_encode(&self) -> alloc::string::String {
        let mut hex_str = hex::encode(self.encode());
        hex_str.insert_str(0, "0x");
        hex_str
    }
}

impl<Call> fmt::Debug for UncheckedExtrinsicV4<Call>
where
    Call: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UncheckedExtrinsic({:?}, {:?})",
            self.signature.as_ref().map(|x| (&x.0, &x.2)),
            self.function
        )
    }
}

const V4: u8 = 4;

impl<Call> Encode for UncheckedExtrinsicV4<Call>
where
    Call: Encode,
{
    fn encode(&self) -> Vec<u8> {
        encode_with_vec_prefix::<Self, _>(|v| {
            match self.signature.as_ref() {
                Some(s) => {
                    println!("--->>> It is using a signature..");
                    v.push(V4 | 0b1000_0000);
                    s.encode_to(v);
                }
                None => {
                    println!("___ OK _no signature is used____");
                    v.push(V4 & 0b0111_1111);
                }
            }
            self.function.encode_to(v);
        })
    }
}

impl<Call> Decode for UncheckedExtrinsicV4<Call>
where
    Call: Decode + Encode,
{
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        // This is a little more complicated than usual since the binary format must be compatible
        // with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
        // will be a prefix of vector length (we don't need
        // to use this).
        let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

        let version = input.read_byte()?;

        let is_signed = version & 0b1000_0000 != 0;
        let version = version & 0b0111_1111;
        if version != V4 {
            return Err("Invalid transaction version".into());
        }

        Ok(UncheckedExtrinsicV4 {
            signature: if is_signed {
                Some(Decode::decode(input)?)
            } else {
                None
            },
            function: Decode::decode(input)?,
        })
    }
}

/// Same function as in primitives::generic. Needed to be copied as it is private there.
fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F) -> Vec<u8> {
    let size = sp_std::mem::size_of::<T>();
    let reserve = match size {
        0..=0b0011_1111 => 1,
        0b0100_0000..=0b0011_1111_1111_1111 => 2,
        _ => 4,
    };
    let mut v = Vec::with_capacity(reserve + size);
    v.resize(reserve, 0);
    encoder(&mut v);

    // need to prefix with the total length to ensure it's binary compatible with
    // Vec<u8>.
    let mut length: Vec<()> = Vec::new();
    length.resize(v.len() - reserve, ());
    length.using_encoded(|s| {
        v.splice(0..reserve, s.iter().cloned());
    });

    v
}

fn compose_extrinsics<P, Params, Tip>()
where
    P: Pair,
    Params: ExtrinsicParams<OtherParams = BaseExtrinsicParamsBuilder<Tip>>,
    MultiSigner: From<P::Public>,
    MultiSignature: From<P::Signature>,
    u128: From<Tip>,
    Tip: Encode + Default,
{
    let runtime_version: RuntimeVersion = todo!();
    let genesis_hash: H256 = todo!();
    let metadata: Metadata = todo!();
    let signer: Option<P> = todo!();
    let nonce: u32 = todo!();
    let extrinsic_params: Option<Params::OtherParams> = todo!();

    let pallet = metadata.pallet("TemplateModule").unwrap();
    let call_index = pallet.calls.get("do_something").unwrap();
    let call = ([pallet.index, *call_index as u8], (2));

    if let Some(signer) = signer {
        let other_params = extrinsic_params.unwrap_or_default();
        let params: BaseExtrinsicParams<Tip> = BaseExtrinsicParams::new(nonce, other_params);
        let extra = GenericExtra::from(params);
        let raw_payload = SignedPayload::from_raw(
            call,
            extra,
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
        let xt: UncheckedExtrinsicV4<_> = UncheckedExtrinsicV4::new_signed(
            call,
            GenericAddress::from(multi_signer.into_account()),
            multi_signature,
            extra,
        );
        println!("a signed xt: {:#?}", xt);
    } else {
        let xt: UncheckedExtrinsicV4<_> = UncheckedExtrinsicV4 {
            signature: None,
            function: call,
        };
        println!("xt: {:#?}", xt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::extrinsic_params::PlainTip;
    use crate::types::extrinsic_params::PlainTipExtrinsicParams;
    use crate::types::extrinsic_params::{
        BaseExtrinsicParams, ExtrinsicParams, PlainTipExtrinsicParamsBuilder,
    };
    use sp_core::Pair;
    use sp_core::H256 as Hash;
    use sp_runtime::generic::Era;
    use sp_runtime::testing::sr25519;
    use sp_runtime::MultiSignature;

    #[test]
    fn try_compose_extrinsics() {
        compose_extrinsics::<sp_core::sr25519::Pair, PlainTipExtrinsicParams, PlainTip>();
    }

    #[test]
    fn encode_decode_roundtrip_works() {
        let msg = &b"test-message"[..];
        let (pair, _) = sr25519::Pair::generate();
        let signature = pair.sign(&msg);
        let multi_sig = MultiSignature::from(signature);
        let account: AccountId = pair.public().into();
        let tx_params =
            PlainTipExtrinsicParamsBuilder::new().era(Era::mortal(8, 0), Hash::from([0u8; 32]));

        let default_extra = BaseExtrinsicParams::new(0, tx_params);
        let xt = UncheckedExtrinsicV4::new_signed(
            vec![1, 1, 1],
            account.into(),
            multi_sig,
            GenericExtra::from(default_extra),
        );
        let xt_enc = xt.encode();
        assert_eq!(xt, Decode::decode(&mut xt_enc.as_slice()).unwrap())
    }
}
