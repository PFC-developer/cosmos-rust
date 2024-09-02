// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetVoucher {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventSetVoucher {
    const NAME: &'static str = "EventSetVoucher";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// Params defines the parameters for the permissions module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub wasm_hook_query_max_gas: u64,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// Namespace defines a permissions namespace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    /// tokenfactory denom to which this namespace applies to
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// address of smart contract to apply code-based restrictions
    #[prost(string, tag = "2")]
    pub wasm_hook: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub mints_paused: bool,
    #[prost(bool, tag = "4")]
    pub sends_paused: bool,
    #[prost(bool, tag = "5")]
    pub burns_paused: bool,
    /// permissions for each role
    #[prost(message, repeated, tag = "6")]
    pub role_permissions: ::prost::alloc::vec::Vec<Role>,
    #[prost(message, repeated, tag = "7")]
    pub address_roles: ::prost::alloc::vec::Vec<AddressRoles>,
}
impl ::prost::Name for Namespace {
    const NAME: &'static str = "Namespace";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressRoles {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for AddressRoles {
    const NAME: &'static str = "AddressRoles";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// Role is only used for storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    #[prost(string, tag = "1")]
    pub role: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub permissions: u32,
}
impl ::prost::Name for Role {
    const NAME: &'static str = "Role";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// used in storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleIDs {
    #[prost(uint32, repeated, tag = "1")]
    pub role_ids: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for RoleIDs {
    const NAME: &'static str = "RoleIDs";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Voucher {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for Voucher {
    const NAME: &'static str = "Voucher";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressVoucher {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub voucher: ::core::option::Option<Voucher>,
}
impl ::prost::Name for AddressVoucher {
    const NAME: &'static str = "AddressVoucher";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// each Action enum value should be a power of two
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Unspecified = 0,
    Mint = 1,
    Receive = 2,
    Burn = 4,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unspecified => "UNSPECIFIED",
            Action::Mint => "MINT",
            Action::Receive => "RECEIVE",
            Action::Burn => "BURN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "MINT" => Some(Self::Mint),
            "RECEIVE" => Some(Self::Receive),
            "BURN" => Some(Self::Burn),
            _ => None,
        }
    }
}
/// GenesisState defines the permissions module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryAllNamespacesRequest is the request type for the Query/AllNamespaces RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllNamespacesRequest {}
impl ::prost::Name for QueryAllNamespacesRequest {
    const NAME: &'static str = "QueryAllNamespacesRequest";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryAllNamespacesResponse is the response type for the Query/AllNamespaces
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllNamespacesResponse {
    #[prost(message, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
}
impl ::prost::Name for QueryAllNamespacesResponse {
    const NAME: &'static str = "QueryAllNamespacesResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryNamespaceByDenomRequest is the request type for the
/// Query/NamespaceByDenom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespaceByDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub include_roles: bool,
}
impl ::prost::Name for QueryNamespaceByDenomRequest {
    const NAME: &'static str = "QueryNamespaceByDenomRequest";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryNamespaceByDenomResponse is the response type for the
/// Query/NamespaceByDenom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespaceByDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub namespace: ::core::option::Option<Namespace>,
}
impl ::prost::Name for QueryNamespaceByDenomResponse {
    const NAME: &'static str = "QueryNamespaceByDenomResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryAddressesByRoleRequest is the request type for the Query/AddressesByRole
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddressesByRoleRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAddressesByRoleRequest {
    const NAME: &'static str = "QueryAddressesByRoleRequest";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
/// QueryAddressesByRoleResponse is the response type for the
/// Query/AddressesByRole RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddressesByRoleResponse {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryAddressesByRoleResponse {
    const NAME: &'static str = "QueryAddressesByRoleResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddressRolesRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAddressRolesRequest {
    const NAME: &'static str = "QueryAddressRolesRequest";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddressRolesResponse {
    #[prost(string, repeated, tag = "1")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryAddressRolesResponse {
    const NAME: &'static str = "QueryAddressRolesResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVouchersForAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryVouchersForAddressRequest {
    const NAME: &'static str = "QueryVouchersForAddressRequest";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVouchersForAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub vouchers: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryVouchersForAddressResponse {
    const NAME: &'static str = "QueryVouchersForAddressResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the permissions parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNamespace {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub namespace: ::core::option::Option<Namespace>,
}
impl ::prost::Name for MsgCreateNamespace {
    const NAME: &'static str = "MsgCreateNamespace";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNamespaceResponse {}
impl ::prost::Name for MsgCreateNamespaceResponse {
    const NAME: &'static str = "MsgCreateNamespaceResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteNamespace {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgDeleteNamespace {
    const NAME: &'static str = "MsgDeleteNamespace";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteNamespaceResponse {}
impl ::prost::Name for MsgDeleteNamespaceResponse {
    const NAME: &'static str = "MsgDeleteNamespaceResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateNamespace {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
    /// address of smart contract to apply code-based restrictions
    #[prost(message, optional, tag = "3")]
    pub wasm_hook: ::core::option::Option<msg_update_namespace::MsgSetWasmHook>,
    #[prost(message, optional, tag = "4")]
    pub mints_paused: ::core::option::Option<msg_update_namespace::MsgSetMintsPaused>,
    #[prost(message, optional, tag = "5")]
    pub sends_paused: ::core::option::Option<msg_update_namespace::MsgSetSendsPaused>,
    #[prost(message, optional, tag = "6")]
    pub burns_paused: ::core::option::Option<msg_update_namespace::MsgSetBurnsPaused>,
}
/// Nested message and enum types in `MsgUpdateNamespace`.
pub mod msg_update_namespace {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MsgSetWasmHook {
        #[prost(string, tag = "1")]
        pub new_value: ::prost::alloc::string::String,
    }
    impl ::prost::Name for MsgSetWasmHook {
        const NAME: &'static str = "MsgSetWasmHook";
        const PACKAGE: &'static str = "injective.permissions.v1beta1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "injective.permissions.v1beta1.MsgUpdateNamespace.{}",
                Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MsgSetMintsPaused {
        #[prost(bool, tag = "1")]
        pub new_value: bool,
    }
    impl ::prost::Name for MsgSetMintsPaused {
        const NAME: &'static str = "MsgSetMintsPaused";
        const PACKAGE: &'static str = "injective.permissions.v1beta1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "injective.permissions.v1beta1.MsgUpdateNamespace.{}",
                Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MsgSetSendsPaused {
        #[prost(bool, tag = "1")]
        pub new_value: bool,
    }
    impl ::prost::Name for MsgSetSendsPaused {
        const NAME: &'static str = "MsgSetSendsPaused";
        const PACKAGE: &'static str = "injective.permissions.v1beta1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "injective.permissions.v1beta1.MsgUpdateNamespace.{}",
                Self::NAME
            )
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MsgSetBurnsPaused {
        #[prost(bool, tag = "1")]
        pub new_value: bool,
    }
    impl ::prost::Name for MsgSetBurnsPaused {
        const NAME: &'static str = "MsgSetBurnsPaused";
        const PACKAGE: &'static str = "injective.permissions.v1beta1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!(
                "injective.permissions.v1beta1.MsgUpdateNamespace.{}",
                Self::NAME
            )
        }
    }
}
impl ::prost::Name for MsgUpdateNamespace {
    const NAME: &'static str = "MsgUpdateNamespace";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateNamespaceResponse {}
impl ::prost::Name for MsgUpdateNamespaceResponse {
    const NAME: &'static str = "MsgUpdateNamespaceResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateNamespaceRoles {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
    /// new role definitions or updated permissions for existing roles
    #[prost(message, repeated, tag = "3")]
    pub role_permissions: ::prost::alloc::vec::Vec<Role>,
    /// new addresses to add or new roles for existing addresses to
    #[prost(message, repeated, tag = "4")]
    pub address_roles: ::prost::alloc::vec::Vec<AddressRoles>,
}
impl ::prost::Name for MsgUpdateNamespaceRoles {
    const NAME: &'static str = "MsgUpdateNamespaceRoles";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateNamespaceRolesResponse {}
impl ::prost::Name for MsgUpdateNamespaceRolesResponse {
    const NAME: &'static str = "MsgUpdateNamespaceRolesResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeNamespaceRoles {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag = "2")]
    pub namespace_denom: ::prost::alloc::string::String,
    /// {"address" => array of roles to revoke from this address}
    #[prost(message, repeated, tag = "3")]
    pub address_roles_to_revoke: ::prost::alloc::vec::Vec<AddressRoles>,
}
impl ::prost::Name for MsgRevokeNamespaceRoles {
    const NAME: &'static str = "MsgRevokeNamespaceRoles";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeNamespaceRolesResponse {}
impl ::prost::Name for MsgRevokeNamespaceRolesResponse {
    const NAME: &'static str = "MsgRevokeNamespaceRolesResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimVoucher {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgClaimVoucher {
    const NAME: &'static str = "MsgClaimVoucher";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimVoucherResponse {}
impl ::prost::Name for MsgClaimVoucherResponse {
    const NAME: &'static str = "MsgClaimVoucherResponse";
    const PACKAGE: &'static str = "injective.permissions.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.permissions.v1beta1.{}", Self::NAME)
    }
}
include!("injective.permissions.v1beta1.serde.rs");
include!("injective.permissions.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
