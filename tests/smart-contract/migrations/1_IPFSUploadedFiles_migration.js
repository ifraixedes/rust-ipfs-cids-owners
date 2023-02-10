// Help Truffle find `IPFSUploadedFiles.sol` in the `/contracts` directory
const IPFSUploadedFiles = artifacts.require("IPFSUploadedFiles");

module.exports = function(deployer) {
  // Command Truffle to deploy the Smart Contract
  deployer.deploy(IPFSUploadedFiles);
};
