use crate::error::{BoxError, Error, ExternalSystem};

use std::sync::Arc;

use ethers::{
    core::types::{Address, TransactionReceipt},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use ipfs_cids_owners_contracts::cids_owners;

pub struct CIDsOwners {
    contract_address: Address,
    provider: Provider<Http>,
    chain_id: u64,
}

impl CIDsOwners {
    /// Create a new instance.
    /// When `chain_id` is `None`, 1 is used (i.e. Ethereum mainnet).
    pub fn new(
        contract_address: Address,
        endpoint: &str,
        chain_id: Option<u64>,
    ) -> Result<Self, Error> {
        let chain_id = if let Some(id) = chain_id { id } else { 1 };

        let provider = Provider::<Http>::try_from(endpoint)
            .map_err(|_| Error::invalid_arguments("endpoint", "malformed HTTP address"))?;

        // TODO: this constructor should validate `endppoint` and `contract_address` to report the
        // caller any problem with them rather than creating the instance and then finding the
        // issue when calling some of the methods of the instance later on.

        Ok(Self {
            contract_address,
            provider,
            chain_id,
        })
    }

    // Register `cid` to the `owner` wallet.
    pub async fn register_cid_owner(
        &self,
        cid: &str,
        owner: LocalWallet,
    ) -> Result<TransactionReceipt, Error> {
        let owner = owner.with_chain_id(self.chain_id);

        let client = SignerMiddleware::new(self.provider.clone(), owner);
        let client = Arc::new(client);
        let contract = cids_owners::CIDsOwners::new(self.contract_address, client);

        let receipt = contract
            .register(String::from(cid))
            .send()
            .await
            .map_err(|err| Error::external(BoxError::from(err), ExternalSystem::Ethereum))?
            .await
            .map_err(|err| Error::external(BoxError::from(err), ExternalSystem::Ethereum))?;

        Ok(receipt.expect("always expecting a transaction receipt from the register method of the CIDsOwners contracdt"))
    }

    // Get the registered CIDs from `owner`.
    // Currently isn't exposed publicly because only tests uses it and it isn't part of the
    // requirements.
    #[allow(dead_code)]
    pub(crate) async fn my_registered_cids(
        &self,
        owner: LocalWallet,
    ) -> Result<std::vec::Vec<String>, Error> {
        let owner = owner.with_chain_id(self.chain_id);
        let owner_address = owner.address();

        let client = SignerMiddleware::new(self.provider.clone(), owner);
        let client = Arc::new(client);
        let contract = cids_owners::CIDsOwners::new(self.contract_address, client);

        let cids = contract
            .get_owned_ci_ds(owner_address)
            .call()
            .await
            .map_err(|err| Error::external(BoxError::from(err), ExternalSystem::Ethereum))?;

        Ok(cids)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::env;

    #[test_with::http(localhost:8545)]
    #[test_with::env(IPFS_CIDS_OWNERS_CONTRACT_ADDRESS, IPFS_CIDS_OWNER_PRIV_KEY)]
    #[tokio::test]
    async fn test_cids_owners_register() {
        let contract_addr = env::var("IPFS_CIDS_OWNERS_CONTRACT_ADDRESS").expect(
            "BUG this test should be ignored without the 'IPFS_CIDS_OWNERS_CONTRACT_ADDRESS' env var",
        );
        let contract_addr = contract_addr
            .parse::<Address>()
            .expect("a valid contract address");

        let owner_priv_key = env::var("IPFS_CIDS_OWNER_PRIV_KEY").expect(
            "BUG this test should be ignored without the 'IPFS_CIDS_OWNER_PRIV_KEY' env var",
        );
        let owner_wallet = owner_priv_key
            .parse::<LocalWallet>()
            .expect("a valid private key. NOTE set it without the '0x' prefix)");

        let cids_owners =
            CIDsOwners::new(contract_addr, "http://localhost:8545", Some(1337 as u64))
                .expect("instance CIDsOwners successfully");

        cids_owners
            .register_cid_owner("fake CID", owner_wallet.clone())
            .await
            .expect("register a CID successfully");

        let registered_cids = cids_owners
            .my_registered_cids(owner_wallet)
            .await
            .expect("get my registered CIDs successful");

        // Note we check that at least the CID is once because if we run the test several times the
        // CID will be more than once and checking that the vector only contains one element and
        // the CID matches would make the test fragile.
        assert!(
            registered_cids.iter().any(|cid| cid == "fake CID"),
            "has the registered CID"
        );
    }
}
