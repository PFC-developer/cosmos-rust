// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractRegistrationRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub contract_registration_request: ::core::option::Option<ContractRegistrationRequest>,
}
impl ::prost::Name for ContractRegistrationRequestProposal {
    const NAME: &'static str = "ContractRegistrationRequestProposal";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchContractRegistrationRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub contract_registration_requests: ::prost::alloc::vec::Vec<ContractRegistrationRequest>,
}
impl ::prost::Name for BatchContractRegistrationRequestProposal {
    const NAME: &'static str = "BatchContractRegistrationRequestProposal";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchContractDeregistrationProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BatchContractDeregistrationProposal {
    const NAME: &'static str = "BatchContractDeregistrationProposal";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractRegistrationRequest {
    /// Unique Identifier for contract instance to be registered.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// Maximum gas to be used for the smart contract execution.
    #[prost(uint64, tag = "2")]
    pub gas_limit: u64,
    /// gas price to be used for the smart contract execution.
    #[prost(uint64, tag = "3")]
    pub gas_price: u64,
    #[prost(bool, tag = "4")]
    pub should_pin_contract: bool,
    /// if true contract owner can update it, if false only current code_id will be
    /// allowed to be executed
    #[prost(bool, tag = "5")]
    pub is_migration_allowed: bool,
    /// code_id of the contract being registered - will be verified upon every
    /// execution but only if is_migration_allowed is false
    #[prost(uint64, tag = "6")]
    pub code_id: u64,
    /// Optional address of admin account (that will be allowed to pause or update
    /// contract params)
    #[prost(string, tag = "7")]
    pub admin_address: ::prost::alloc::string::String,
    /// Optional address of the contract that grants fees. Must be set if
    /// funding_mode is other than SelfFunded
    #[prost(string, tag = "8")]
    pub granter_address: ::prost::alloc::string::String,
    /// Specifies how the contract will fund its execution
    #[prost(enumeration = "FundingMode", tag = "9")]
    pub funding_mode: i32,
}
impl ::prost::Name for ContractRegistrationRequest {
    const NAME: &'static str = "ContractRegistrationRequest";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchStoreCodeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub proposals:
        ::prost::alloc::vec::Vec<super::super::super::cosmwasm::wasm::v1::StoreCodeProposal>,
}
impl ::prost::Name for BatchStoreCodeProposal {
    const NAME: &'static str = "BatchStoreCodeProposal";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FundingMode {
    Unspecified = 0,
    SelfFunded = 1,
    GrantOnly = 2,
    Dual = 3,
}
impl FundingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FundingMode::Unspecified => "Unspecified",
            FundingMode::SelfFunded => "SelfFunded",
            FundingMode::GrantOnly => "GrantOnly",
            FundingMode::Dual => "Dual",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "SelfFunded" => Some(Self::SelfFunded),
            "GrantOnly" => Some(Self::GrantOnly),
            "Dual" => Some(Self::Dual),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Set the status to active to indicate that contracts can be executed in
    /// begin blocker.
    #[prost(bool, tag = "1")]
    pub is_execution_enabled: bool,
    /// Maximum aggregate total gas to be used for the contract executions in the
    /// BeginBlocker.
    #[prost(uint64, tag = "2")]
    pub max_begin_block_total_gas: u64,
    /// the maximum gas limit each individual contract can consume in the
    /// BeginBlocker.
    #[prost(uint64, tag = "3")]
    pub max_contract_gas_limit: u64,
    /// min_gas_price defines the minimum gas price the contracts must pay to be
    /// executed in the BeginBlocker.
    #[prost(uint64, tag = "4")]
    pub min_gas_price: u64,
    #[prost(message, optional, tag = "5")]
    pub register_contract_access:
        ::core::option::Option<super::super::super::cosmwasm::wasm::v1::AccessConfig>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredContract {
    /// limit of gas per BB execution
    #[prost(uint64, tag = "1")]
    pub gas_limit: u64,
    /// gas price that contract is willing to pay for execution in BeginBlocker
    #[prost(uint64, tag = "2")]
    pub gas_price: u64,
    /// is contract currently active
    #[prost(bool, tag = "3")]
    pub is_executable: bool,
    /// code_id that is allowed to be executed (to prevent malicious updates) - if
    /// nil/0 any code_id can be executed
    #[prost(uint64, tag = "4")]
    pub code_id: u64,
    /// optional - admin addr that is allowed to update contract data
    #[prost(string, tag = "5")]
    pub admin_address: ::prost::alloc::string::String,
    /// Optional: address of the contract granting fee
    /// Must be set if fund_mode is GrantOnly
    #[prost(string, tag = "6")]
    pub granter_address: ::prost::alloc::string::String,
    /// funding mode
    #[prost(enumeration = "FundingMode", tag = "7")]
    pub fund_mode: i32,
}
impl ::prost::Name for RegisteredContract {
    const NAME: &'static str = "RegisteredContract";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventContractExecution {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub response: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub other_error: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub execution_error: ::prost::alloc::string::String,
}
impl ::prost::Name for EventContractExecution {
    const NAME: &'static str = "EventContractExecution";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventContractRegistered {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub gas_price: u64,
    #[prost(bool, tag = "4")]
    pub should_pin_contract: bool,
    #[prost(bool, tag = "5")]
    pub is_migration_allowed: bool,
    #[prost(uint64, tag = "6")]
    pub code_id: u64,
    #[prost(string, tag = "7")]
    pub admin_address: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub granter_address: ::prost::alloc::string::String,
    #[prost(enumeration = "FundingMode", tag = "9")]
    pub funding_mode: i32,
}
impl ::prost::Name for EventContractRegistered {
    const NAME: &'static str = "EventContractRegistered";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventContractDeregistered {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
impl ::prost::Name for EventContractDeregistered {
    const NAME: &'static str = "EventContractDeregistered";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredContractWithAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub registered_contract: ::core::option::Option<RegisteredContract>,
}
impl ::prost::Name for RegisteredContractWithAddress {
    const NAME: &'static str = "RegisteredContractWithAddress";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// GenesisState defines the wasmx module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to wasmx.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// registered_contracts is an array containing the genesis registered
    /// contracts
    #[prost(message, repeated, tag = "2")]
    pub registered_contracts: ::prost::alloc::vec::Vec<RegisteredContractWithAddress>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// QueryWasmxParamsRequest is the request type for the Query/WasmxParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWasmxParamsRequest {}
impl ::prost::Name for QueryWasmxParamsRequest {
    const NAME: &'static str = "QueryWasmxParamsRequest";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// QueryWasmxParamsRequest is the response type for the Query/WasmxParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWasmxParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryWasmxParamsResponse {
    const NAME: &'static str = "QueryWasmxParamsResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// QueryModuleStateRequest is the request type for the Query/WasmxModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {}
impl ::prost::Name for QueryModuleStateRequest {
    const NAME: &'static str = "QueryModuleStateRequest";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// QueryModuleStateResponse is the response type for the Query/WasmxModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
impl ::prost::Name for QueryModuleStateResponse {
    const NAME: &'static str = "QueryModuleStateResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// Contract registration info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractRegistrationInfoRequest {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryContractRegistrationInfoRequest {
    const NAME: &'static str = "QueryContractRegistrationInfoRequest";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractRegistrationInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub contract: ::core::option::Option<RegisteredContract>,
}
impl ::prost::Name for QueryContractRegistrationInfoResponse {
    const NAME: &'static str = "QueryContractRegistrationInfoResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// MsgExecuteContractCompat submits the given message data to a smart contract,
/// compatible with EIP712
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContractCompat {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
    /// Funds coins that are transferred to the contract on execution
    #[prost(string, tag = "4")]
    pub funds: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgExecuteContractCompat {
    const NAME: &'static str = "MsgExecuteContractCompat";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
/// MsgExecuteContractCompatResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContractCompatResponse {
    /// Data contains bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MsgExecuteContractCompatResponse {
    const NAME: &'static str = "MsgExecuteContractCompatResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Unique Identifier for contract instance to be registered.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    /// Maximum gas to be used for the smart contract execution.
    #[prost(uint64, tag = "3")]
    pub gas_limit: u64,
    /// gas price to be used for the smart contract execution.
    #[prost(uint64, tag = "4")]
    pub gas_price: u64,
    /// optional - admin account that will be allowed to perform any changes
    #[prost(string, tag = "5")]
    pub admin_address: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateContract {
    const NAME: &'static str = "MsgUpdateContract";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateContractResponse {}
impl ::prost::Name for MsgUpdateContractResponse {
    const NAME: &'static str = "MsgUpdateContractResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Unique Identifier for contract instance to be activated.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgActivateContract {
    const NAME: &'static str = "MsgActivateContract";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateContractResponse {}
impl ::prost::Name for MsgActivateContractResponse {
    const NAME: &'static str = "MsgActivateContractResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeactivateContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Unique Identifier for contract instance to be deactivated.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgDeactivateContract {
    const NAME: &'static str = "MsgDeactivateContract";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeactivateContractResponse {}
impl ::prost::Name for MsgDeactivateContractResponse {
    const NAME: &'static str = "MsgDeactivateContractResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the wasmx parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub contract_registration_request: ::core::option::Option<ContractRegistrationRequest>,
}
impl ::prost::Name for MsgRegisterContract {
    const NAME: &'static str = "MsgRegisterContract";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterContractResponse {}
impl ::prost::Name for MsgRegisterContractResponse {
    const NAME: &'static str = "MsgRegisterContractResponse";
    const PACKAGE: &'static str = "injective.wasmx.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.wasmx.v1.{}", Self::NAME)
    }
}
include!("injective.wasmx.v1.serde.rs");
include!("injective.wasmx.v1.tonic.rs");
// @@protoc_insertion_point(module)
