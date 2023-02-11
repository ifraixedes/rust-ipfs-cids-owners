use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;

use ethers::{
    contract::EthEvent,
    core::types::{Address, H160},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use ipfs_api_backend_hyper::{request, IpfsApi, IpfsClient, TryFromUri};

use ipfs_cids_owners_contracts::cids_owners;

#[derive(Clone, Debug, EthEvent)]
struct ValueChanged {
    old_author: Address,
    new_author: Address,
    old_value: String,
    new_value: String,
}

#[tokio::main]
async fn main() {
    /*
    use http::uri::Scheme;

    let client = IpfsClient::from_host_and_port(
        Scheme::try_from("http").expect("http scheme"),
        "localhost",
        5001,
    )
    .unwrap();

    let data = Cursor::new("Hello World!");
    let add_opts = request::Add {
        to_files: Some("/my-hello-world.txt"),
        ..Default::default()
    };

    let cid = match client.add_with_options(data, add_opts).await {
        Ok(res) => {
            println!("{}", res.hash);
            res.hash
        }
        Err(e) => {
            eprintln!("error adding file: {}", e);
            return;
        }
    };
    */

    let contract_address = "4a4bc7883b8551827d3b03e1713192f799b3610f"
        .parse::<Address>()
        .expect("contract address");

    // Corresponding wallet: 0xe6A6c96b89389080CdBe5A9716a65026178938A3
    let wallet: LocalWallet = "7b5bd14f46bea1709b61011abb33d4bb6446f456eab4d0304e98425500cd3061"
        .parse()
        .expect("parse wallet address");

    let wallet = wallet.with_chain_id(1337 as u64);
    println!("wallet address: {}", wallet.address());

    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

    let client = SignerMiddleware::new(provider, wallet.clone());

    // connect to the network
    let client = Arc::new(client);
    let contract = cids_owners::CIDsOwners::new(contract_address, client.clone());

    let receipt = contract
        .register(String::from("my-file"))
        .send()
        .await
        .expect("sending")
        .await
        .expect("transaction receipt");

    println!("receipt: {:?}", receipt);

    let address = wallet.address();
    let registered_cids = contract.get_owned_ci_ds(address).call().await;

    println!("{:?}", registered_cids.expect("cids"));
}
