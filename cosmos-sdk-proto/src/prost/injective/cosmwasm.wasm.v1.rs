// @generated
/// AccessTypeParam
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTypeParam {
    #[prost(enumeration = "AccessType", tag = "1")]
    pub value: i32,
}
impl ::prost::Name for AccessTypeParam {
    const NAME: &'static str = "AccessTypeParam";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// AccessConfig access control type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfig {
    #[prost(enumeration = "AccessType", tag = "1")]
    pub permission: i32,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for AccessConfig {
    const NAME: &'static str = "AccessConfig";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Params defines the set of wasm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub code_upload_access: ::core::option::Option<AccessConfig>,
    #[prost(enumeration = "AccessType", tag = "2")]
    pub instantiate_default_permission: i32,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// CodeInfo is data for the uploaded contract WASM code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfo {
    /// CodeHash is the unique identifier created by wasmvm
    #[prost(bytes = "vec", tag = "1")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    /// Creator address who initially stored the code
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// InstantiateConfig access control to apply on contract creation, optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_config: ::core::option::Option<AccessConfig>,
}
impl ::prost::Name for CodeInfo {
    const NAME: &'static str = "CodeInfo";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// ContractInfo stores a WASM contract instance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfo {
    /// CodeID is the reference to the stored Wasm code
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// Creator address who initially instantiated the contract
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a contract instance.
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    /// Created Tx position when the contract was instantiated.
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(string, tag = "6")]
    pub ibc_port_id: ::prost::alloc::string::String,
    /// Extension is an extension point to store custom metadata within the
    /// persistence model.
    #[prost(message, optional, tag = "7")]
    pub extension: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for ContractInfo {
    const NAME: &'static str = "ContractInfo";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// ContractCodeHistoryEntry metadata to a contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCodeHistoryEntry {
    #[prost(enumeration = "ContractCodeHistoryOperationType", tag = "1")]
    pub operation: i32,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "2")]
    pub code_id: u64,
    /// Updated Tx position when the operation was executed.
    #[prost(message, optional, tag = "3")]
    pub updated: ::core::option::Option<AbsoluteTxPosition>,
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ContractCodeHistoryEntry {
    const NAME: &'static str = "ContractCodeHistoryEntry";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// AbsoluteTxPosition is a unique transaction position that allows for global
/// ordering of transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbsoluteTxPosition {
    /// BlockHeight is the block the contract was created at
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    /// TxIndex is a monotonic counter within the block (actual transaction index,
    /// or gas consumed)
    #[prost(uint64, tag = "2")]
    pub tx_index: u64,
}
impl ::prost::Name for AbsoluteTxPosition {
    const NAME: &'static str = "AbsoluteTxPosition";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Model is a struct that holds a KV pair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// hex-encode key to read it better (this is often ascii)
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// base64-encode raw value
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Model {
    const NAME: &'static str = "Model";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// AccessType permission types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessType {
    /// AccessTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// AccessTypeNobody forbidden
    Nobody = 1,
    /// AccessTypeEverybody unrestricted
    Everybody = 3,
    /// AccessTypeAnyOfAddresses allow any of the addresses
    AnyOfAddresses = 4,
}
impl AccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessType::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
            AccessType::Nobody => "ACCESS_TYPE_NOBODY",
            AccessType::Everybody => "ACCESS_TYPE_EVERYBODY",
            AccessType::AnyOfAddresses => "ACCESS_TYPE_ANY_OF_ADDRESSES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCESS_TYPE_NOBODY" => Some(Self::Nobody),
            "ACCESS_TYPE_EVERYBODY" => Some(Self::Everybody),
            "ACCESS_TYPE_ANY_OF_ADDRESSES" => Some(Self::AnyOfAddresses),
            _ => None,
        }
    }
}
/// ContractCodeHistoryOperationType actions that caused a code change
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractCodeHistoryOperationType {
    /// ContractCodeHistoryOperationTypeUnspecified placeholder for empty value
    Unspecified = 0,
    /// ContractCodeHistoryOperationTypeInit on chain contract instantiation
    Init = 1,
    /// ContractCodeHistoryOperationTypeMigrate code migration
    Migrate = 2,
    /// ContractCodeHistoryOperationTypeGenesis based on genesis data
    Genesis = 3,
}
impl ContractCodeHistoryOperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractCodeHistoryOperationType::Unspecified => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED"
            }
            ContractCodeHistoryOperationType::Init => "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT",
            ContractCodeHistoryOperationType::Migrate => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
            }
            ContractCodeHistoryOperationType::Genesis => {
                "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT" => Some(Self::Init),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE" => Some(Self::Migrate),
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_GENESIS" => Some(Self::Genesis),
            _ => None,
        }
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit StoreCodeProposal. To submit WASM code to the system,
/// a simple MsgStoreCode can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreCodeProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "4")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "7")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// UnpinCode code on upload, optional
    #[prost(bool, tag = "8")]
    pub unpin_code: bool,
    /// Source is the URL where the code is hosted
    #[prost(string, tag = "9")]
    pub source: ::prost::alloc::string::String,
    /// Builder is the docker image used to build the code deterministically, used
    /// for smart contract verification
    #[prost(string, tag = "10")]
    pub builder: ::prost::alloc::string::String,
    /// CodeHash is the SHA256 sum of the code outputted by builder, used for smart
    /// contract verification
    #[prost(bytes = "vec", tag = "11")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for StoreCodeProposal {
    const NAME: &'static str = "StoreCodeProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit InstantiateContractProposal. To instantiate a contract,
/// a simple MsgInstantiateContract can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantiateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "4")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "5")]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "7")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "8")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for InstantiateContractProposal {
    const NAME: &'static str = "InstantiateContractProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit InstantiateContract2Proposal. To instantiate contract 2,
/// a simple MsgInstantiateContract2 can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstantiateContract2Proposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's enviroment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "4")]
    pub admin: ::prost::alloc::string::String,
    /// CodeID is the reference to the stored WASM code
    #[prost(uint64, tag = "5")]
    pub code_id: u64,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encode message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "7")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "8")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Salt is an arbitrary value provided by the sender. Size can be 1 to 64.
    #[prost(bytes = "vec", tag = "9")]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// FixMsg include the msg value into the hash for the predictable address.
    /// Default is false
    #[prost(bool, tag = "10")]
    pub fix_msg: bool,
}
impl ::prost::Name for InstantiateContract2Proposal {
    const NAME: &'static str = "InstantiateContract2Proposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit MigrateContractProposal. To migrate a contract,
/// a simple MsgMigrateContract can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    ///
    /// Note: skipping 3 as this was previously used for unneeded run_as
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// CodeID references the new WASM code
    #[prost(uint64, tag = "5")]
    pub code_id: u64,
    /// Msg json encoded message to be passed to the contract on migration
    #[prost(bytes = "vec", tag = "6")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MigrateContractProposal {
    const NAME: &'static str = "MigrateContractProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit SudoContractProposal. To call sudo on a contract,
/// a simple MsgSudoContract can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SudoContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as sudo
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SudoContractProposal {
    const NAME: &'static str = "SudoContractProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit ExecuteContractProposal. To call execute on a contract,
/// a simple MsgExecuteContract can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract as execute
    #[prost(bytes = "vec", tag = "5")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "6")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for ExecuteContractProposal {
    const NAME: &'static str = "ExecuteContractProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit UpdateAdminProposal. To set an admin for a contract,
/// a simple MsgUpdateAdmin can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAdminProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// NewAdmin address to be set
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "4")]
    pub contract: ::prost::alloc::string::String,
}
impl ::prost::Name for UpdateAdminProposal {
    const NAME: &'static str = "UpdateAdminProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit ClearAdminProposal. To clear the admin of a contract,
/// a simple MsgClearAdmin can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearAdminProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "3")]
    pub contract: ::prost::alloc::string::String,
}
impl ::prost::Name for ClearAdminProposal {
    const NAME: &'static str = "ClearAdminProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit PinCodesProposal. To pin a set of code ids in the wasmvm
/// cache, a simple MsgPinCodes can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinCodesProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// CodeIDs references the new WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for PinCodesProposal {
    const NAME: &'static str = "PinCodesProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit UnpinCodesProposal. To unpin a set of code ids in the wasmvm
/// cache, a simple MsgUnpinCodes can be invoked from the x/gov module via
/// a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpinCodesProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// CodeIDs references the WASM codes
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub code_ids: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for UnpinCodesProposal {
    const NAME: &'static str = "UnpinCodesProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// AccessConfigUpdate contains the code id and the access config to be
/// applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfigUpdate {
    /// CodeID is the reference to the stored WASM code to be updated
    #[prost(uint64, tag = "1")]
    pub code_id: u64,
    /// InstantiatePermission to apply to the set of code ids
    #[prost(message, optional, tag = "2")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
}
impl ::prost::Name for AccessConfigUpdate {
    const NAME: &'static str = "AccessConfigUpdate";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit UpdateInstantiateConfigProposal. To update instantiate config
/// to a set of code ids, a simple MsgUpdateInstantiateConfig can be invoked from
/// the x/gov module via a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstantiateConfigProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// AccessConfigUpdate contains the list of code ids and the access config
    /// to be applied.
    #[prost(message, repeated, tag = "3")]
    pub access_config_updates: ::prost::alloc::vec::Vec<AccessConfigUpdate>,
}
impl ::prost::Name for UpdateInstantiateConfigProposal {
    const NAME: &'static str = "UpdateInstantiateConfigProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
/// Deprecated: Do not use. Since wasmd v0.40, there is no longer a need for
/// an explicit StoreAndInstantiateContractProposal. To store and instantiate
/// the contract, a simple MsgStoreAndInstantiateContract can be invoked from
/// the x/gov module via a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreAndInstantiateContractProposal {
    /// Title is a short summary
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// RunAs is the address that is passed to the contract's environment as sender
    #[prost(string, tag = "3")]
    pub run_as: ::prost::alloc::string::String,
    /// WASMByteCode can be raw or gzip compressed
    #[prost(bytes = "vec", tag = "4")]
    pub wasm_byte_code: ::prost::alloc::vec::Vec<u8>,
    /// InstantiatePermission to apply on contract creation, optional
    #[prost(message, optional, tag = "5")]
    pub instantiate_permission: ::core::option::Option<AccessConfig>,
    /// UnpinCode code on upload, optional
    #[prost(bool, tag = "6")]
    pub unpin_code: bool,
    /// Admin is an optional address that can execute migrations
    #[prost(string, tag = "7")]
    pub admin: ::prost::alloc::string::String,
    /// Label is optional metadata to be stored with a constract instance.
    #[prost(string, tag = "8")]
    pub label: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract on instantiation
    #[prost(bytes = "vec", tag = "9")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on instantiation
    #[prost(message, repeated, tag = "10")]
    pub funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Source is the URL where the code is hosted
    #[prost(string, tag = "11")]
    pub source: ::prost::alloc::string::String,
    /// Builder is the docker image used to build the code deterministically, used
    /// for smart contract verification
    #[prost(string, tag = "12")]
    pub builder: ::prost::alloc::string::String,
    /// CodeHash is the SHA256 sum of the code outputted by builder, used for smart
    /// contract verification
    #[prost(bytes = "vec", tag = "13")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for StoreAndInstantiateContractProposal {
    const NAME: &'static str = "StoreAndInstantiateContractProposal";
    const PACKAGE: &'static str = "cosmwasm.wasm.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmwasm.wasm.v1.{}", Self::NAME)
    }
}
include!("cosmwasm.wasm.v1.serde.rs");
// @@protoc_insertion_point(module)
