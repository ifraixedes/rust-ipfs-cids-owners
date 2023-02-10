// SPDX-License-Identifier: MIT

pragma solidity >=0.8.18 <0.9.0;

/**
 * @title IPFSUploadedFiles
 * @dev Register CIDs of files uploaded to IPFS.
 */
contract IPFSUploadedFiles {
    mapping(address owner => string[] cids) uploadedFiles;

    /**
     * @dev Register a CID owned by sender
     * @param cid content identifier for the upload file to register
     */
    function register(string calldata cid) public {
        uploadedFiles[msg.sender].push(cid);
    }

    /**
      * @dev Retrieves the list of CIDs belonging to owner.
      * @param owner of the returned list of CIDs.
      * @return cids List of CIDs.
      */
    function getUploadedCIDs(address owner) public view returns (string[] memory cids) {
        return uploadedFiles[owner];
    }
}
