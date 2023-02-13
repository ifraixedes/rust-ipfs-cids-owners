# Cli

This crate implements the command-line application.

The application is itself documented, use the `--help` flag to know what flags and parameters
accepts.

## Testing

Because the application functionality relies on interacting with third party services, some of the
tests require that these services are available on the local machine (i.e. `localhost`) in specific
ports.

Some of the tests also require to set the following specific environment variables:
- `IPFS_CIDS_OWNERS_CONTRACT_ADDRESS`: It has to contain the Ethereum address of the deployed
  CIDsOwners  smart contract.
- `IPFS_CIDS_OWNER_PRIV_KEY`: It has to contain a private key of one of the available Ethereum
  account addresses without the `0x` prefix.

The tests are skipped if  the third party services aren't available or the environment variables
aren't set.

The easiest thing is running the local services using the `Makefile` present in the root of this
repository. If it doesn't work in your machine, you can take a look to it to see what each recipe
executes and execute each command by yourself.

However, the `Makefile` doesn't export the environment variables because I didn't have time to find
how to take them from the local services, hence, you have to set them manually.

To find the values for them, execute from the root of the repository `docker compose logs ethereum`
to see the Ethereum local node logs and then, grab one of the available private keys and the address
where the smart contract was deployed.

You can also use them and the local services to execute the command-line application without having
to rely on any external service. NOTE that you have to pass the chain ID, which you find it in the
same logs commented above.
