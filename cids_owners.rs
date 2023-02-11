pub use ci_ds_owners::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ci_ds_owners {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "CIDsOwners was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"cid\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"register\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOwnedCIDs\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"cids\",\"type\":\"string[]\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static CIDSOWNERS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct CIDsOwners<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CIDsOwners<M> {
        fn clone(&self) -> Self {
            CIDsOwners(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CIDsOwners<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CIDsOwners<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CIDsOwners))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CIDsOwners<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CIDSOWNERS_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `getOwnedCIDs` (0x5ba52c96) function"]
        pub fn get_owned_ci_ds(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<String>> {
            self.0
                .method_hash([91, 165, 44, 150], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `register` (0xf2c298be) function"]
        pub fn register(&self, cid: String) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 194, 152, 190], cid)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CIDsOwners<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getOwnedCIDs` function with signature `getOwnedCIDs(address)` and selector `[91, 165, 44, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOwnedCIDs", abi = "getOwnedCIDs(address)")]
    pub struct GetOwnedCIDsCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `register` function with signature `register(string)` and selector `[242, 194, 152, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "register", abi = "register(string)")]
    pub struct RegisterCall {
        pub cid: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CIDsOwnersCalls {
        GetOwnedCIDs(GetOwnedCIDsCall),
        Register(RegisterCall),
    }
    impl ethers::core::abi::AbiDecode for CIDsOwnersCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetOwnedCIDsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CIDsOwnersCalls::GetOwnedCIDs(decoded));
            }
            if let Ok(decoded) =
                <RegisterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CIDsOwnersCalls::Register(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CIDsOwnersCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CIDsOwnersCalls::GetOwnedCIDs(element) => element.encode(),
                CIDsOwnersCalls::Register(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CIDsOwnersCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CIDsOwnersCalls::GetOwnedCIDs(element) => element.fmt(f),
                CIDsOwnersCalls::Register(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetOwnedCIDsCall> for CIDsOwnersCalls {
        fn from(var: GetOwnedCIDsCall) -> Self {
            CIDsOwnersCalls::GetOwnedCIDs(var)
        }
    }
    impl ::std::convert::From<RegisterCall> for CIDsOwnersCalls {
        fn from(var: RegisterCall) -> Self {
            CIDsOwnersCalls::Register(var)
        }
    }
    #[doc = "Container type for all return fields from the `getOwnedCIDs` function with signature `getOwnedCIDs(address)` and selector `[91, 165, 44, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOwnedCIDsReturn {
        pub cids: ::std::vec::Vec<String>,
    }
}
