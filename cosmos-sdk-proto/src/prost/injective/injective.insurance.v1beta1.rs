// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// default_redemption_notice_period_duration defines the default minimum
    /// notice period duration that must pass after an underwriter sends a
    /// redemption request before the underwriter can claim his tokens
    #[prost(message, optional, tag = "1")]
    pub default_redemption_notice_period_duration:
        ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsuranceFund {
    /// deposit denomination for the given insurance fund
    #[prost(string, tag = "1")]
    pub deposit_denom: ::prost::alloc::string::String,
    /// insurance fund pool token denomination for the given insurance fund
    #[prost(string, tag = "2")]
    pub insurance_pool_token_denom: ::prost::alloc::string::String,
    /// redemption_notice_period_duration defines the minimum notice period
    /// duration that must pass after an underwriter sends a redemption request
    /// before the underwriter can claim his tokens
    #[prost(message, optional, tag = "3")]
    pub redemption_notice_period_duration:
        ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    /// balance of fund
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
    /// total share tokens minted
    #[prost(string, tag = "5")]
    pub total_share: ::prost::alloc::string::String,
    /// marketID of the derivative market
    #[prost(string, tag = "6")]
    pub market_id: ::prost::alloc::string::String,
    /// ticker of the derivative market
    #[prost(string, tag = "7")]
    pub market_ticker: ::prost::alloc::string::String,
    /// Oracle base currency of the derivative market OR the oracle symbol for the
    /// binary options market.
    #[prost(string, tag = "8")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency of the derivative market OR the oracle provider for
    /// the binary options market.
    #[prost(string, tag = "9")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type of the binary options or derivative market
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "10")]
    pub oracle_type: i32,
    /// Expiration time of the derivative market. Should be -1 for perpetual or -2
    /// for binary options markets.
    #[prost(int64, tag = "11")]
    pub expiry: i64,
}
impl ::prost::Name for InsuranceFund {
    const NAME: &'static str = "InsuranceFund";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedemptionSchedule {
    /// id of redemption schedule
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// marketId of insurance fund for the redemption
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// address of the redeemer
    #[prost(string, tag = "3")]
    pub redeemer: ::prost::alloc::string::String,
    /// the time after which the redemption can be claimed
    #[prost(message, optional, tag = "4")]
    pub claimable_redemption_time:
        ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// the insurance_pool_token amount to redeem
    #[prost(message, optional, tag = "5")]
    pub redemption_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for RedemptionSchedule {
    const NAME: &'static str = "RedemptionSchedule";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInsuranceFundUpdate {
    #[prost(message, optional, tag = "1")]
    pub fund: ::core::option::Option<InsuranceFund>,
}
impl ::prost::Name for EventInsuranceFundUpdate {
    const NAME: &'static str = "EventInsuranceFundUpdate";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRequestRedemption {
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<RedemptionSchedule>,
}
impl ::prost::Name for EventRequestRedemption {
    const NAME: &'static str = "EventRequestRedemption";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdrawRedemption {
    /// redemption schedule triggered withdraw
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<RedemptionSchedule>,
    /// redeem coin amount in base_currency
    #[prost(message, optional, tag = "2")]
    pub redeem_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventWithdrawRedemption {
    const NAME: &'static str = "EventWithdrawRedemption";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUnderwrite {
    /// address of the underwriter
    #[prost(string, tag = "1")]
    pub underwriter: ::prost::alloc::string::String,
    /// marketId of insurance fund for the redemption
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// deposit coin amount
    #[prost(message, optional, tag = "3")]
    pub deposit: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// share coin amount
    #[prost(message, optional, tag = "4")]
    pub shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventUnderwrite {
    const NAME: &'static str = "EventUnderwrite";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInsuranceWithdraw {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_ticker: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub withdrawal: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventInsuranceWithdraw {
    const NAME: &'static str = "EventInsuranceWithdraw";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// GenesisState defines the insurance module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to insurance.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// insurance_funds describes the insurance funds available for derivative
    /// markets
    #[prost(message, repeated, tag = "2")]
    pub insurance_funds: ::prost::alloc::vec::Vec<InsuranceFund>,
    /// redemption_schedule describes the redemption requests pending
    #[prost(message, repeated, tag = "3")]
    pub redemption_schedule: ::prost::alloc::vec::Vec<RedemptionSchedule>,
    /// next_share_denom_id describes the next share denom id to be used for newly
    /// creating insurance fund incremented by 1 per insurance fund creation
    #[prost(uint64, tag = "4")]
    pub next_share_denom_id: u64,
    /// next_redemption_schedule_id describes next redemption schedule id to be
    /// used for next schedule incremented by 1 per redemption request
    #[prost(uint64, tag = "5")]
    pub next_redemption_schedule_id: u64,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryInsuranceParamsRequest is the request type for the Query/InsuranceParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInsuranceParamsRequest {}
impl ::prost::Name for QueryInsuranceParamsRequest {
    const NAME: &'static str = "QueryInsuranceParamsRequest";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryInsuranceParamsRequest is the response type for the
/// Query/InsuranceParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInsuranceParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryInsuranceParamsResponse {
    const NAME: &'static str = "QueryInsuranceParamsResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryInsuranceFundRequest is the request type for the Query/InsuranceFunds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInsuranceFundRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryInsuranceFundRequest {
    const NAME: &'static str = "QueryInsuranceFundRequest";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryInsuranceFundResponse is the response type for the Query/InsuranceFund
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInsuranceFundResponse {
    #[prost(message, optional, tag = "1")]
    pub fund: ::core::option::Option<InsuranceFund>,
}
impl ::prost::Name for QueryInsuranceFundResponse {
    const NAME: &'static str = "QueryInsuranceFundResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryInsuranceFundsRequest is the request type for the Query/InsuranceFunds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInsuranceFundsRequest {}
impl ::prost::Name for QueryInsuranceFundsRequest {
    const NAME: &'static str = "QueryInsuranceFundsRequest";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryInsuranceFundsResponse is the response type for the Query/InsuranceFunds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInsuranceFundsResponse {
    #[prost(message, repeated, tag = "1")]
    pub funds: ::prost::alloc::vec::Vec<InsuranceFund>,
}
impl ::prost::Name for QueryInsuranceFundsResponse {
    const NAME: &'static str = "QueryInsuranceFundsResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryEstimatedRedemptionsRequest is the request type for the
/// Query/EstimatedRedemptions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimatedRedemptionsRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryEstimatedRedemptionsRequest {
    const NAME: &'static str = "QueryEstimatedRedemptionsRequest";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryEstimatedRedemptionsResponse is the response type for the
/// Query/EstimatedRedemptions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimatedRedemptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryEstimatedRedemptionsResponse {
    const NAME: &'static str = "QueryEstimatedRedemptionsResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryPendingRedemptionsRequest is the request type for the
/// Query/PendingRedemptions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingRedemptionsRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPendingRedemptionsRequest {
    const NAME: &'static str = "QueryPendingRedemptionsRequest";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryPendingRedemptionsResponse is the response type for the
/// Query/PendingRedemptions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingRedemptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryPendingRedemptionsResponse {
    const NAME: &'static str = "QueryPendingRedemptionsResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateRequest is the request type for the
/// Query/InsuranceModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {}
impl ::prost::Name for QueryModuleStateRequest {
    const NAME: &'static str = "QueryModuleStateRequest";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateResponse is the response type for the
/// Query/InsuranceModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
impl ::prost::Name for QueryModuleStateResponse {
    const NAME: &'static str = "QueryModuleStateResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateInsuranceFund a message to create an insurance fund for a derivative
/// market.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateInsuranceFund {
    /// Creator of the insurance fund.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Coin denom to use for the market quote denom
    #[prost(string, tag = "3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency of the derivative market OR the oracle symbol for the
    /// binary options market.
    #[prost(string, tag = "4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency of the derivative market OR the oracle provider for
    /// the binary options market.
    #[prost(string, tag = "5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type of the binary options or derivative market
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "6")]
    pub oracle_type: i32,
    /// Expiration time of the derivative market. Should be -1 for perpetual or -2
    /// for binary options markets.
    #[prost(int64, tag = "7")]
    pub expiry: i64,
    /// Initial deposit of the insurance fund
    #[prost(message, optional, tag = "8")]
    pub initial_deposit: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgCreateInsuranceFund {
    const NAME: &'static str = "MsgCreateInsuranceFund";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateInsuranceFundResponse {}
impl ::prost::Name for MsgCreateInsuranceFundResponse {
    const NAME: &'static str = "MsgCreateInsuranceFundResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// MsgUnderwrite defines a message for depositing coins to underwrite an
/// insurance fund
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnderwrite {
    /// Address of the underwriter.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// MarketID of the insurance fund.
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// Amount of quote_denom to underwrite the insurance fund.
    #[prost(message, optional, tag = "3")]
    pub deposit: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgUnderwrite {
    const NAME: &'static str = "MsgUnderwrite";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnderwriteResponse {}
impl ::prost::Name for MsgUnderwriteResponse {
    const NAME: &'static str = "MsgUnderwriteResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
/// MsgRequestRedemption defines a message for requesting a redemption of the
/// sender's insurance fund tokens
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestRedemption {
    /// Address of the underwriter requesting a redemption.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// MarketID of the insurance fund.
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// Insurance fund share token amount to be redeemed.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgRequestRedemption {
    const NAME: &'static str = "MsgRequestRedemption";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestRedemptionResponse {}
impl ::prost::Name for MsgRequestRedemptionResponse {
    const NAME: &'static str = "MsgRequestRedemptionResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the insurance parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.insurance.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.insurance.v1beta1.{}", Self::NAME)
    }
}
include!("injective.insurance.v1beta1.serde.rs");
include!("injective.insurance.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
