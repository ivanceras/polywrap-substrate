#![allow(warnings)]
use crate::api::Api;
use crate::utils::FromHexStr;
use crate::Error;
use codec::Decode;
use sp_core::storage::StorageKey;

impl Api {
    // curl -H "Content-Type: application/json" -d '{"id":"1","jsonrpc":"2.0","method":"state_getStorage","params":["0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9de1e86a9a8c739864cf3cc5ec2bea59fd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",null]}' http://localhost:9933/
    pub async fn fetch_storage_value<'a, V>(
        &self,
        module: &str,
        storage_name: &str,
    ) -> Result<Option<V>, Error>
    where
        V: Decode,
    {
        //TODO: store the metadata at the first fetch
        let metadata = self.fetch_metadata().await?;
        let storage_key = metadata.storage_value_key(module, storage_name)?;
        println!("storage_key: 0x{}", hex::encode(&storage_key));
        self.fetch_storage_by_key_hash(storage_key).await
    }

    pub async fn fetch_storage_by_key_hash<V>(
        &self,
        storage_key: StorageKey,
    ) -> Result<Option<V>, Error>
    where
        V: Decode,
    {
        match self.fetch_opaque_storage_by_key_hash(storage_key).await? {
            Some(storage) => Ok(Some(Decode::decode(&mut storage.as_slice())?)),
            None => Ok(None),
        }
    }

    async fn fetch_opaque_storage_by_key_hash(
        &self,
        storage_key: StorageKey,
    ) -> Result<Option<Vec<u8>>, Error> {
        let result = self.json_request("state_getStorage", [storage_key]).await?;
        println!("result: {:#?}", result);
        if result.result.is_null() {
            Ok(None)
        } else {
            let result_str = result.result.as_str().expect("must be a str");
            let data = Vec::from_hex(result_str)?;
            println!("data: {:?}", data);
            Ok(Some(data))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn show_total_balance_total_issuance() {
        let result: Result<Option<u128>, Error> = Api::new("http://localhost:9933")
            .fetch_storage_value("Balances", "TotalIssuance")
            .await;
        println!("result: {:?}", result);
        assert!(result.is_ok());
        let result = result.ok().flatten().unwrap();
        // only succeed when the substrate node is fresh or unmodified
        assert_eq!(result, 4611686018427387904);
    }

    #[tokio::test]
    async fn show_template_module() {
        let result: Result<Option<u32>, Error> = Api::new("http://localhost:9933")
            .fetch_storage_value("TemplateModule", "Something")
            .await;
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
