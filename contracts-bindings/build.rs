use std::env;
use std::path::PathBuf;

use ethers::contract::Abigen;

fn main() {
    // Cargo defines this env var, but it's better to ensure that it's defined for having a nice
    // error in case that it isn't defined for some unknown reasons.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR env var not defined"));

    // Directory containing smart contract project.
    let smart_contract_dir = PathBuf::from("contracts");

    // Generate the Rust bindings for the smart contracts.
    Abigen::new(
        "CIDsOwners",
        &smart_contract_dir.join("CIDsOwners.json").to_string_lossy(),
    )
    .expect("create builder ABI JSON contract file")
    .generate()
    .expect("generate bindings for the CIDsOwners smart contract")
    .write_to_file(out_dir.join("cids_owners.rs"))
    .expect("write smart CIDsOwners contract bindings file (cids_owners.rs)");
}
