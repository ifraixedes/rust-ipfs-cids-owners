use crate::{
    cli,
    error::{BoxError, Error},
    ethereum, ipfs,
};

use ethers::{abi::AbiEncode, core::types::Address, signers::LocalWallet};
use ipfs_api_backend_hyper::{IpfsClient, TryFromUri};

/// Upload a file specified by the command-line to IPFS and register it's CID to the CIDsOwners
/// smart contract.
pub async fn upload_and_register(args: cli::App) -> Result<UploadRegisterSummary, Error> {
    let ipfs_cli = IpfsClient::from_host_and_port(
        args.ipfs_endpoint.scheme,
        &args.ipfs_endpoint.host,
        args.ipfs_endpoint.port,
    )
    // TODO: verify if IpfsClient checks the connectivity and if it may return error because of
    // other reasons, in that case this error could vary between `Error::InvalidArguments` and
    // `Error::Internal`.
    .map_err(|_| {
        Error::invalid_arguments(
            "ipfs-endpoint",
            "endpoint doesn't correspond to an IPFS service",
        )
    })?;

    let contract_addr = args.ether_contract_address
            .parse::<Address>()
            .map_err(|err|
                     Error::internal(
                         "BUG cli module should validate that the passed Etherem address is of a valid format",
                         BoxError::from(err),
                    ),
            )?;

    let owner_wallet = args.ether_owner_priv_key
            .parse::<LocalWallet>()
            .map_err(|err|
                     Error::internal(
                         "BUG cli module should validate that the passed Etherem private key is of a valid format",
                         BoxError::from(err),
                    ),
            )?;

    let cids_owners = ethereum::CIDsOwners::new(
        contract_addr,
        &args.ether_endpoint.to_string(),
        Some(args.ehter_chain_id),
    )?;

    let client = ipfs::Client::with_client(&ipfs_cli);

    let cid = client
        .uploload_file(&args.filepath, args.remote_path.as_deref())
        .await?;

    let receipt = cids_owners.register_cid_owner(&cid, owner_wallet).await?;

    Ok(UploadRegisterSummary {
        cid,
        ether_tx_hash: receipt.transaction_hash.encode_hex(),
    })
}

pub struct UploadRegisterSummary {
    pub cid: String,
    pub ether_tx_hash: String,
}

impl std::fmt::Display for UploadRegisterSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CID: '{}', Etherem transaction hash: '{}'",
            self.cid, self.ether_tx_hash
        )
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for the `upload_and_register` function.
}
