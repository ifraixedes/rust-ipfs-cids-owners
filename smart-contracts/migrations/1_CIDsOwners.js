// Help Truffle find `CIDsOwners.sol` in the `/contracts` directory
const CIDsOwners = artifacts.require("CIDsOwners");

module.exports = function(deployer) {
  // Command Truffle to deploy the Smart Contract
  deployer.deploy(CIDsOwners);
};
