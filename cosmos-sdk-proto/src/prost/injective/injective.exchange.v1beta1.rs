// @generated
/// spot authz messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpotLimitOrderAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for CreateSpotLimitOrderAuthz {
    const NAME: &'static str = "CreateSpotLimitOrderAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpotMarketOrderAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for CreateSpotMarketOrderAuthz {
    const NAME: &'static str = "CreateSpotMarketOrderAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateSpotLimitOrdersAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BatchCreateSpotLimitOrdersAuthz {
    const NAME: &'static str = "BatchCreateSpotLimitOrdersAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSpotOrderAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for CancelSpotOrderAuthz {
    const NAME: &'static str = "CancelSpotOrderAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCancelSpotOrdersAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BatchCancelSpotOrdersAuthz {
    const NAME: &'static str = "BatchCancelSpotOrdersAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// derivative authz messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDerivativeLimitOrderAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for CreateDerivativeLimitOrderAuthz {
    const NAME: &'static str = "CreateDerivativeLimitOrderAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDerivativeMarketOrderAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for CreateDerivativeMarketOrderAuthz {
    const NAME: &'static str = "CreateDerivativeMarketOrderAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateDerivativeLimitOrdersAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BatchCreateDerivativeLimitOrdersAuthz {
    const NAME: &'static str = "BatchCreateDerivativeLimitOrdersAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDerivativeOrderAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for CancelDerivativeOrderAuthz {
    const NAME: &'static str = "CancelDerivativeOrderAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCancelDerivativeOrdersAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BatchCancelDerivativeOrdersAuthz {
    const NAME: &'static str = "BatchCancelDerivativeOrdersAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// common authz message used in both spot & derivative markets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateOrdersAuthz {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub spot_markets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub derivative_markets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BatchUpdateOrdersAuthz {
    const NAME: &'static str = "BatchUpdateOrdersAuthz";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// spot_market_instant_listing_fee defines the expedited fee in INJ required
    /// to create a spot market by bypassing governance
    #[prost(message, optional, tag = "1")]
    pub spot_market_instant_listing_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// derivative_market_instant_listing_fee defines the expedited fee in INJ
    /// required to create a derivative market by bypassing governance
    #[prost(message, optional, tag = "2")]
    pub derivative_market_instant_listing_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// default_spot_maker_fee defines the default exchange trade fee for makers on
    /// a spot market
    #[prost(string, tag = "3")]
    pub default_spot_maker_fee_rate: ::prost::alloc::string::String,
    /// default_spot_taker_fee_rate defines the default exchange trade fee rate for
    /// takers on a new spot market
    #[prost(string, tag = "4")]
    pub default_spot_taker_fee_rate: ::prost::alloc::string::String,
    /// default_derivative_maker_fee defines the default exchange trade fee for
    /// makers on a new derivative market
    #[prost(string, tag = "5")]
    pub default_derivative_maker_fee_rate: ::prost::alloc::string::String,
    /// default_derivative_taker_fee defines the default exchange trade fee for
    /// takers on a new derivative market
    #[prost(string, tag = "6")]
    pub default_derivative_taker_fee_rate: ::prost::alloc::string::String,
    /// default_initial_margin_ratio defines the default initial margin ratio on a
    /// new derivative market
    #[prost(string, tag = "7")]
    pub default_initial_margin_ratio: ::prost::alloc::string::String,
    /// default_maintenance_margin_ratio defines the default maintenance margin
    /// ratio on a new derivative market
    #[prost(string, tag = "8")]
    pub default_maintenance_margin_ratio: ::prost::alloc::string::String,
    /// default_funding_interval defines the default funding interval on a
    /// derivative market
    #[prost(int64, tag = "9")]
    pub default_funding_interval: i64,
    /// funding_multiple defines the timestamp multiple that the funding timestamp
    /// should be a multiple of
    #[prost(int64, tag = "10")]
    pub funding_multiple: i64,
    /// relayer_fee_share_rate defines the trade fee share percentage that goes to
    /// relayers
    #[prost(string, tag = "11")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// default_hourly_funding_rate_cap defines the default maximum absolute value
    /// of the hourly funding rate
    #[prost(string, tag = "12")]
    pub default_hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag = "13")]
    pub default_hourly_interest_rate: ::prost::alloc::string::String,
    /// max_derivative_order_side_count defines the maximum number of derivative
    /// active orders a subaccount can have for a given orderbook side
    #[prost(uint32, tag = "14")]
    pub max_derivative_order_side_count: u32,
    /// inj_reward_staked_requirement_threshold defines the threshold on INJ
    /// rewards after which one also needs staked INJ to receive more
    #[prost(string, tag = "15")]
    pub inj_reward_staked_requirement_threshold: ::prost::alloc::string::String,
    /// the trading_rewards_vesting_duration defines the vesting times for trading
    /// rewards
    #[prost(int64, tag = "16")]
    pub trading_rewards_vesting_duration: i64,
    /// liquidator_reward_share_rate defines the ratio of the split of the surplus
    /// collateral that goes to the liquidator
    #[prost(string, tag = "17")]
    pub liquidator_reward_share_rate: ::prost::alloc::string::String,
    /// binary_options_market_instant_listing_fee defines the expedited fee in INJ
    /// required to create a derivative market by bypassing governance
    #[prost(message, optional, tag = "18")]
    pub binary_options_market_instant_listing_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// atomic_market_order_access_level defines the required access permissions
    /// for executing atomic market orders
    #[prost(enumeration = "AtomicMarketOrderAccessLevel", tag = "19")]
    pub atomic_market_order_access_level: i32,
    /// spot_atomic_market_order_fee_multiplier defines the default multiplier for
    /// executing atomic market orders in spot markets
    #[prost(string, tag = "20")]
    pub spot_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// derivative_atomic_market_order_fee_multiplier defines the default
    /// multiplier for executing atomic market orders in derivative markets
    #[prost(string, tag = "21")]
    pub derivative_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// binary_options_atomic_market_order_fee_multiplier defines the default
    /// multiplier for executing atomic market orders in binary markets
    #[prost(string, tag = "22")]
    pub binary_options_atomic_market_order_fee_multiplier: ::prost::alloc::string::String,
    /// minimal_protocol_fee_rate defines the minimal protocol fee rate
    #[prost(string, tag = "23")]
    pub minimal_protocol_fee_rate: ::prost::alloc::string::String,
    /// is_instant_derivative_market_launch_enabled defines whether instant
    /// derivative market launch is enabled
    #[prost(bool, tag = "24")]
    pub is_instant_derivative_market_launch_enabled: bool,
    #[prost(int64, tag = "25")]
    pub post_only_mode_height_threshold: i64,
    /// Maximum time in seconds since the last mark price update to allow a
    /// decrease in margin
    #[prost(int64, tag = "26")]
    pub margin_decrease_price_timestamp_threshold_seconds: i64,
    /// List of addresses that are allowed to perform exchange admin operations
    #[prost(string, repeated, tag = "27")]
    pub exchange_admins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// inj_auction_max_cap defines the maximum cap for INJ sent to auction
    #[prost(string, tag = "28")]
    pub inj_auction_max_cap: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFeeMultiplier {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub fee_multiplier: ::prost::alloc::string::String,
}
impl ::prost::Name for MarketFeeMultiplier {
    const NAME: &'static str = "MarketFeeMultiplier";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// An object describing a derivative market in the Injective Futures Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarket {
    /// Ticker for the derivative contract.
    #[prost(string, tag = "1")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag = "2")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "3")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "4")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "5")]
    pub oracle_scale_factor: u32,
    /// Address of the quote currency denomination for the derivative contract
    #[prost(string, tag = "6")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag = "7")]
    pub market_id: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio of a derivative
    /// market
    #[prost(string, tag = "8")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio of a
    /// derivative market
    #[prost(string, tag = "9")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a derivative market
    #[prost(string, tag = "10")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag = "11")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag = "12")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// true if the market is a perpetual market. false if the market is an expiry
    /// futures market
    #[prost(bool, tag = "13")]
    pub is_perpetual: bool,
    /// Status of the market
    #[prost(enumeration = "MarketStatus", tag = "14")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag = "15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag = "16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "17")]
    pub min_notional: ::prost::alloc::string::String,
    /// current market admin
    #[prost(string, tag = "18")]
    pub admin: ::prost::alloc::string::String,
    /// level of admin permissions
    #[prost(uint32, tag = "19")]
    pub admin_permissions: u32,
}
impl ::prost::Name for DerivativeMarket {
    const NAME: &'static str = "DerivativeMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// An object describing a binary options market in Injective Protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarket {
    /// Ticker for the derivative contract.
    #[prost(string, tag = "1")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag = "2")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag = "3")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "4")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "5")]
    pub oracle_scale_factor: u32,
    /// expiration timestamp
    #[prost(int64, tag = "6")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag = "7")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag = "8")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag = "9")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag = "10")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a binary options market
    #[prost(string, tag = "11")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag = "12")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag = "13")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// Status of the market
    #[prost(enumeration = "MarketStatus", tag = "14")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag = "15")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag = "16")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub settlement_price: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "18")]
    pub min_notional: ::prost::alloc::string::String,
    /// level of admin permissions
    #[prost(uint32, tag = "19")]
    pub admin_permissions: u32,
}
impl ::prost::Name for BinaryOptionsMarket {
    const NAME: &'static str = "BinaryOptionsMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketInfo {
    /// market ID.
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// expiration_timestamp defines the expiration time for a time expiry futures
    /// market.
    #[prost(int64, tag = "2")]
    pub expiration_timestamp: i64,
    /// expiration_twap_start_timestamp defines the start time of the TWAP
    /// calculation window
    #[prost(int64, tag = "3")]
    pub twap_start_timestamp: i64,
    /// expiration_twap_start_price_cumulative defines the cumulative price for the
    /// start of the TWAP window
    #[prost(string, tag = "4")]
    pub expiration_twap_start_price_cumulative: ::prost::alloc::string::String,
    /// settlement_price defines the settlement price for a time expiry futures
    /// market.
    #[prost(string, tag = "5")]
    pub settlement_price: ::prost::alloc::string::String,
}
impl ::prost::Name for ExpiryFuturesMarketInfo {
    const NAME: &'static str = "ExpiryFuturesMarketInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketInfo {
    /// market ID.
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// hourly_funding_rate_cap defines the maximum absolute value of the hourly
    /// funding rate
    #[prost(string, tag = "2")]
    pub hourly_funding_rate_cap: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag = "3")]
    pub hourly_interest_rate: ::prost::alloc::string::String,
    /// next_funding_timestamp defines the next funding timestamp in seconds of a
    /// perpetual market
    #[prost(int64, tag = "4")]
    pub next_funding_timestamp: i64,
    /// funding_interval defines the next funding interval in seconds of a
    /// perpetual market.
    #[prost(int64, tag = "5")]
    pub funding_interval: i64,
}
impl ::prost::Name for PerpetualMarketInfo {
    const NAME: &'static str = "PerpetualMarketInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketFunding {
    /// cumulative_funding defines the cumulative funding of a perpetual market.
    #[prost(string, tag = "1")]
    pub cumulative_funding: ::prost::alloc::string::String,
    /// cumulative_price defines the cumulative price for the current hour up to
    /// the last timestamp
    #[prost(string, tag = "2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub last_timestamp: i64,
}
impl ::prost::Name for PerpetualMarketFunding {
    const NAME: &'static str = "PerpetualMarketFunding";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketSettlementInfo {
    /// market ID.
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// settlement_price defines the settlement price
    #[prost(string, tag = "2")]
    pub settlement_price: ::prost::alloc::string::String,
}
impl ::prost::Name for DerivativeMarketSettlementInfo {
    const NAME: &'static str = "DerivativeMarketSettlementInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextFundingTimestamp {
    #[prost(int64, tag = "1")]
    pub next_timestamp: i64,
}
impl ::prost::Name for NextFundingTimestamp {
    const NAME: &'static str = "NextFundingTimestamp";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MidPriceAndTob {
    /// mid price of the market
    #[prost(string, tag = "1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag = "2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag = "3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
impl ::prost::Name for MidPriceAndTob {
    const NAME: &'static str = "MidPriceAndTOB";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// An object describing trade pair of two assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarket {
    /// A name of the pair in format AAA/BBB, where AAA is base asset, BBB is quote
    /// asset.
    #[prost(string, tag = "1")]
    pub ticker: ::prost::alloc::string::String,
    /// Coin denom used for the base asset
    #[prost(string, tag = "2")]
    pub base_denom: ::prost::alloc::string::String,
    /// Coin used for the quote asset
    #[prost(string, tag = "3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// maker_fee_rate defines the fee percentage makers pay when trading
    #[prost(string, tag = "4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the fee percentage takers pay when trading
    #[prost(string, tag = "5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the percentage of the transaction fee shared
    /// with the relayer in a derivative market
    #[prost(string, tag = "6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// Unique market ID.
    #[prost(string, tag = "7")]
    pub market_id: ::prost::alloc::string::String,
    /// Status of the market
    #[prost(enumeration = "MarketStatus", tag = "8")]
    pub status: i32,
    /// min_price_tick_size defines the minimum tick size that the price required
    /// for orders in the market
    #[prost(string, tag = "9")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag = "10")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "11")]
    pub min_notional: ::prost::alloc::string::String,
    /// current market admin
    #[prost(string, tag = "12")]
    pub admin: ::prost::alloc::string::String,
    /// level of admin permissions
    #[prost(uint32, tag = "13")]
    pub admin_permissions: u32,
}
impl ::prost::Name for SpotMarket {
    const NAME: &'static str = "SpotMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A subaccount's deposit for a given base currency
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(string, tag = "1")]
    pub available_balance: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub total_balance: ::prost::alloc::string::String,
}
impl ::prost::Name for Deposit {
    const NAME: &'static str = "Deposit";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountTradeNonce {
    #[prost(uint32, tag = "1")]
    pub nonce: u32,
}
impl ::prost::Name for SubaccountTradeNonce {
    const NAME: &'static str = "SubaccountTradeNonce";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderInfo {
    /// bytes32 subaccount ID that created the order
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// address fee_recipient address that will receive fees for the order
    #[prost(string, tag = "2")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// price of the order
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag = "4")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for OrderInfo {
    const NAME: &'static str = "OrderInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "3")]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "4")]
    pub trigger_price: ::prost::alloc::string::String,
}
impl ::prost::Name for SpotOrder {
    const NAME: &'static str = "SpotOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A valid Spot limit order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "2")]
    pub order_type: i32,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag = "3")]
    pub fillable: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "4")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SpotLimitOrder {
    const NAME: &'static str = "SpotLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A valid Spot market order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    #[prost(string, tag = "2")]
    pub balance_hold: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "4")]
    pub order_type: i32,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "5")]
    pub trigger_price: ::prost::alloc::string::String,
}
impl ::prost::Name for SpotMarketOrder {
    const NAME: &'static str = "SpotMarketOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrder {
    /// market_id represents the unique ID of the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "2")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "3")]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag = "4")]
    pub margin: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "5")]
    pub trigger_price: ::prost::alloc::string::String,
}
impl ::prost::Name for DerivativeOrder {
    const NAME: &'static str = "DerivativeOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderbookMetadata {
    #[prost(uint32, tag = "1")]
    pub vanilla_limit_order_count: u32,
    #[prost(uint32, tag = "2")]
    pub reduce_only_limit_order_count: u32,
    /// AggregateReduceOnlyQuantity is the aggregate fillable quantity of the
    /// subaccount's reduce-only limit orders in the given direction.
    #[prost(string, tag = "3")]
    pub aggregate_reduce_only_quantity: ::prost::alloc::string::String,
    /// AggregateVanillaQuantity is the aggregate fillable quantity of the
    /// subaccount's vanilla limit orders in the given direction.
    #[prost(string, tag = "4")]
    pub aggregate_vanilla_quantity: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub vanilla_conditional_order_count: u32,
    #[prost(uint32, tag = "6")]
    pub reduce_only_conditional_order_count: u32,
}
impl ::prost::Name for SubaccountOrderbookMetadata {
    const NAME: &'static str = "SubaccountOrderbookMetadata";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrder {
    /// price of the order
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_reduce_only: bool,
    #[prost(string, tag = "4")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for SubaccountOrder {
    const NAME: &'static str = "SubaccountOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderData {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<SubaccountOrder>,
    #[prost(bytes = "vec", tag = "2")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SubaccountOrderData {
    const NAME: &'static str = "SubaccountOrderData";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A valid Derivative limit order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "2")]
    pub order_type: i32,
    /// margin is the margin used by the limit order
    #[prost(string, tag = "3")]
    pub margin: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag = "4")]
    pub fillable: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "5")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for DerivativeLimitOrder {
    const NAME: &'static str = "DerivativeLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A valid Derivative market order with Metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrder {
    /// order_info contains the information of the order
    #[prost(message, optional, tag = "1")]
    pub order_info: ::core::option::Option<OrderInfo>,
    /// order types
    #[prost(enumeration = "OrderType", tag = "2")]
    pub order_type: i32,
    #[prost(string, tag = "3")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub margin_hold: ::prost::alloc::string::String,
    /// trigger_price is the trigger price used by stop/take orders
    #[prost(string, tag = "5")]
    pub trigger_price: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for DerivativeMarketOrder {
    const NAME: &'static str = "DerivativeMarketOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(bool, tag = "1")]
    pub is_long: bool,
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub entry_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cumulative_funding_entry: ::prost::alloc::string::String,
}
impl ::prost::Name for Position {
    const NAME: &'static str = "Position";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderIndicator {
    /// market_id represents the unique ID of the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_buy: bool,
}
impl ::prost::Name for MarketOrderIndicator {
    const NAME: &'static str = "MarketOrderIndicator";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLog {
    #[prost(string, tag = "1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    /// bytes32 subaccount ID that executed the trade
    #[prost(bytes = "vec", tag = "3")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub fee: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub fee_recipient_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for TradeLog {
    const NAME: &'static str = "TradeLog";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDelta {
    #[prost(bool, tag = "1")]
    pub is_long: bool,
    #[prost(string, tag = "2")]
    pub execution_quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub execution_margin: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub execution_price: ::prost::alloc::string::String,
}
impl ::prost::Name for PositionDelta {
    const NAME: &'static str = "PositionDelta";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeTradeLog {
    #[prost(bytes = "vec", tag = "1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    #[prost(string, tag = "3")]
    pub payout: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub fee: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub fee_recipient_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub pnl: ::prost::alloc::string::String,
}
impl ::prost::Name for DerivativeTradeLog {
    const NAME: &'static str = "DerivativeTradeLog";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountPosition {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<Position>,
    #[prost(bytes = "vec", tag = "2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SubaccountPosition {
    const NAME: &'static str = "SubaccountPosition";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposit {
    #[prost(bytes = "vec", tag = "1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub deposit: ::core::option::Option<Deposit>,
}
impl ::prost::Name for SubaccountDeposit {
    const NAME: &'static str = "SubaccountDeposit";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositUpdate {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub deposits: ::prost::alloc::vec::Vec<SubaccountDeposit>,
}
impl ::prost::Name for DepositUpdate {
    const NAME: &'static str = "DepositUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsMultiplier {
    #[prost(string, tag = "1")]
    pub maker_points_multiplier: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub taker_points_multiplier: ::prost::alloc::string::String,
}
impl ::prost::Name for PointsMultiplier {
    const NAME: &'static str = "PointsMultiplier";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignBoostInfo {
    #[prost(string, repeated, tag = "1")]
    pub boosted_spot_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub spot_market_multipliers: ::prost::alloc::vec::Vec<PointsMultiplier>,
    #[prost(string, repeated, tag = "3")]
    pub boosted_derivative_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub derivative_market_multipliers: ::prost::alloc::vec::Vec<PointsMultiplier>,
}
impl ::prost::Name for TradingRewardCampaignBoostInfo {
    const NAME: &'static str = "TradingRewardCampaignBoostInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignRewardPool {
    #[prost(int64, tag = "1")]
    pub start_timestamp: i64,
    /// max_campaign_rewards are the maximum reward amounts to be disbursed at the
    /// end of the campaign
    #[prost(message, repeated, tag = "2")]
    pub max_campaign_rewards:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for CampaignRewardPool {
    const NAME: &'static str = "CampaignRewardPool";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignInfo {
    /// number of seconds of the duration of each campaign
    #[prost(int64, tag = "1")]
    pub campaign_duration_seconds: i64,
    /// the trading fee quote denoms which will be counted for the rewards
    #[prost(string, repeated, tag = "2")]
    pub quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the optional boost info for markets
    #[prost(message, optional, tag = "3")]
    pub trading_reward_boost_info: ::core::option::Option<TradingRewardCampaignBoostInfo>,
    /// the marketIDs which are disqualified from being rewarded
    #[prost(string, repeated, tag = "4")]
    pub disqualified_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for TradingRewardCampaignInfo {
    const NAME: &'static str = "TradingRewardCampaignInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountTierInfo {
    #[prost(string, tag = "1")]
    pub maker_discount_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub taker_discount_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub staked_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub volume: ::prost::alloc::string::String,
}
impl ::prost::Name for FeeDiscountTierInfo {
    const NAME: &'static str = "FeeDiscountTierInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountSchedule {
    #[prost(uint64, tag = "1")]
    pub bucket_count: u64,
    #[prost(int64, tag = "2")]
    pub bucket_duration: i64,
    /// the trading fee quote denoms which will be counted for the fee paid
    /// contribution
    #[prost(string, repeated, tag = "3")]
    pub quote_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the fee discount tiers
    #[prost(message, repeated, tag = "4")]
    pub tier_infos: ::prost::alloc::vec::Vec<FeeDiscountTierInfo>,
    /// the marketIDs which are disqualified from contributing to the fee paid
    /// amount
    #[prost(string, repeated, tag = "5")]
    pub disqualified_market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for FeeDiscountSchedule {
    const NAME: &'static str = "FeeDiscountSchedule";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountTierTtl {
    #[prost(uint64, tag = "1")]
    pub tier: u64,
    #[prost(int64, tag = "2")]
    pub ttl_timestamp: i64,
}
impl ::prost::Name for FeeDiscountTierTtl {
    const NAME: &'static str = "FeeDiscountTierTTL";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeRecord {
    #[prost(string, tag = "1")]
    pub maker_volume: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub taker_volume: ::prost::alloc::string::String,
}
impl ::prost::Name for VolumeRecord {
    const NAME: &'static str = "VolumeRecord";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRewards {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for AccountRewards {
    const NAME: &'static str = "AccountRewards";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRecords {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub latest_trade_records: ::prost::alloc::vec::Vec<TradeRecord>,
}
impl ::prost::Name for TradeRecords {
    const NAME: &'static str = "TradeRecords";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountIDs {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for SubaccountIDs {
    const NAME: &'static str = "SubaccountIDs";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeRecord {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quantity: ::prost::alloc::string::String,
}
impl ::prost::Name for TradeRecord {
    const NAME: &'static str = "TradeRecord";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// price
    #[prost(string, tag = "1")]
    pub p: ::prost::alloc::string::String,
    /// quantity
    #[prost(string, tag = "2")]
    pub q: ::prost::alloc::string::String,
}
impl ::prost::Name for Level {
    const NAME: &'static str = "Level";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateSubaccountVolumeRecord {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
impl ::prost::Name for AggregateSubaccountVolumeRecord {
    const NAME: &'static str = "AggregateSubaccountVolumeRecord";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateAccountVolumeRecord {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
impl ::prost::Name for AggregateAccountVolumeRecord {
    const NAME: &'static str = "AggregateAccountVolumeRecord";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketVolume {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub volume: ::core::option::Option<VolumeRecord>,
}
impl ::prost::Name for MarketVolume {
    const NAME: &'static str = "MarketVolume";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomDecimals {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub decimals: u64,
}
impl ::prost::Name for DenomDecimals {
    const NAME: &'static str = "DenomDecimals";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantAuthorization {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for GrantAuthorization {
    const NAME: &'static str = "GrantAuthorization";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGrant {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for ActiveGrant {
    const NAME: &'static str = "ActiveGrant";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveGrant {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub net_granted_stake: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_valid: bool,
}
impl ::prost::Name for EffectiveGrant {
    const NAME: &'static str = "EffectiveGrant";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AtomicMarketOrderAccessLevel {
    Nobody = 0,
    /// currently unsupported
    BeginBlockerSmartContractsOnly = 1,
    SmartContractsOnly = 2,
    Everyone = 3,
}
impl AtomicMarketOrderAccessLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AtomicMarketOrderAccessLevel::Nobody => "Nobody",
            AtomicMarketOrderAccessLevel::BeginBlockerSmartContractsOnly => {
                "BeginBlockerSmartContractsOnly"
            }
            AtomicMarketOrderAccessLevel::SmartContractsOnly => "SmartContractsOnly",
            AtomicMarketOrderAccessLevel::Everyone => "Everyone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Nobody" => Some(Self::Nobody),
            "BeginBlockerSmartContractsOnly" => Some(Self::BeginBlockerSmartContractsOnly),
            "SmartContractsOnly" => Some(Self::SmartContractsOnly),
            "Everyone" => Some(Self::Everyone),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketStatus {
    Unspecified = 0,
    Active = 1,
    Paused = 2,
    Demolished = 3,
    Expired = 4,
}
impl MarketStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketStatus::Unspecified => "Unspecified",
            MarketStatus::Active => "Active",
            MarketStatus::Paused => "Paused",
            MarketStatus::Demolished => "Demolished",
            MarketStatus::Expired => "Expired",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Active" => Some(Self::Active),
            "Paused" => Some(Self::Paused),
            "Demolished" => Some(Self::Demolished),
            "Expired" => Some(Self::Expired),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Unspecified = 0,
    Buy = 1,
    Sell = 2,
    StopBuy = 3,
    StopSell = 4,
    TakeBuy = 5,
    TakeSell = 6,
    BuyPo = 7,
    SellPo = 8,
    BuyAtomic = 9,
    SellAtomic = 10,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "UNSPECIFIED",
            OrderType::Buy => "BUY",
            OrderType::Sell => "SELL",
            OrderType::StopBuy => "STOP_BUY",
            OrderType::StopSell => "STOP_SELL",
            OrderType::TakeBuy => "TAKE_BUY",
            OrderType::TakeSell => "TAKE_SELL",
            OrderType::BuyPo => "BUY_PO",
            OrderType::SellPo => "SELL_PO",
            OrderType::BuyAtomic => "BUY_ATOMIC",
            OrderType::SellAtomic => "SELL_ATOMIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            "STOP_BUY" => Some(Self::StopBuy),
            "STOP_SELL" => Some(Self::StopSell),
            "TAKE_BUY" => Some(Self::TakeBuy),
            "TAKE_SELL" => Some(Self::TakeSell),
            "BUY_PO" => Some(Self::BuyPo),
            "SELL_PO" => Some(Self::SellPo),
            "BUY_ATOMIC" => Some(Self::BuyAtomic),
            "SELL_ATOMIC" => Some(Self::SellAtomic),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    UnspecifiedExecutionType = 0,
    Market = 1,
    LimitFill = 2,
    LimitMatchRestingOrder = 3,
    LimitMatchNewOrder = 4,
    MarketLiquidation = 5,
    ExpiryMarketSettlement = 6,
}
impl ExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionType::UnspecifiedExecutionType => "UnspecifiedExecutionType",
            ExecutionType::Market => "Market",
            ExecutionType::LimitFill => "LimitFill",
            ExecutionType::LimitMatchRestingOrder => "LimitMatchRestingOrder",
            ExecutionType::LimitMatchNewOrder => "LimitMatchNewOrder",
            ExecutionType::MarketLiquidation => "MarketLiquidation",
            ExecutionType::ExpiryMarketSettlement => "ExpiryMarketSettlement",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnspecifiedExecutionType" => Some(Self::UnspecifiedExecutionType),
            "Market" => Some(Self::Market),
            "LimitFill" => Some(Self::LimitFill),
            "LimitMatchRestingOrder" => Some(Self::LimitMatchRestingOrder),
            "LimitMatchNewOrder" => Some(Self::LimitMatchNewOrder),
            "MarketLiquidation" => Some(Self::MarketLiquidation),
            "ExpiryMarketSettlement" => Some(Self::ExpiryMarketSettlement),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderMask {
    Unused = 0,
    Any = 1,
    Regular = 2,
    Conditional = 4,
    /// for conditional orders means HIGHER
    DirectionBuyOrHigher = 8,
    /// for conditional orders means LOWER
    DirectionSellOrLower = 16,
    TypeMarket = 32,
    TypeLimit = 64,
}
impl OrderMask {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderMask::Unused => "UNUSED",
            OrderMask::Any => "ANY",
            OrderMask::Regular => "REGULAR",
            OrderMask::Conditional => "CONDITIONAL",
            OrderMask::DirectionBuyOrHigher => "DIRECTION_BUY_OR_HIGHER",
            OrderMask::DirectionSellOrLower => "DIRECTION_SELL_OR_LOWER",
            OrderMask::TypeMarket => "TYPE_MARKET",
            OrderMask::TypeLimit => "TYPE_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNUSED" => Some(Self::Unused),
            "ANY" => Some(Self::Any),
            "REGULAR" => Some(Self::Regular),
            "CONDITIONAL" => Some(Self::Conditional),
            "DIRECTION_BUY_OR_HIGHER" => Some(Self::DirectionBuyOrHigher),
            "DIRECTION_SELL_OR_LOWER" => Some(Self::DirectionSellOrLower),
            "TYPE_MARKET" => Some(Self::TypeMarket),
            "TYPE_LIMIT" => Some(Self::TypeLimit),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchSpotExecution {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_buy: bool,
    #[prost(enumeration = "ExecutionType", tag = "3")]
    pub execution_type: i32,
    #[prost(message, repeated, tag = "4")]
    pub trades: ::prost::alloc::vec::Vec<TradeLog>,
}
impl ::prost::Name for EventBatchSpotExecution {
    const NAME: &'static str = "EventBatchSpotExecution";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchDerivativeExecution {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_buy: bool,
    #[prost(bool, tag = "3")]
    pub is_liquidation: bool,
    /// nil for time expiry futures
    #[prost(string, tag = "4")]
    pub cumulative_funding: ::prost::alloc::string::String,
    #[prost(enumeration = "ExecutionType", tag = "5")]
    pub execution_type: i32,
    #[prost(message, repeated, tag = "6")]
    pub trades: ::prost::alloc::vec::Vec<DerivativeTradeLog>,
}
impl ::prost::Name for EventBatchDerivativeExecution {
    const NAME: &'static str = "EventBatchDerivativeExecution";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLostFundsFromLiquidation {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub lost_funds_from_available_during_payout: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub lost_funds_from_order_cancels: ::prost::alloc::string::String,
}
impl ::prost::Name for EventLostFundsFromLiquidation {
    const NAME: &'static str = "EventLostFundsFromLiquidation";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchDerivativePosition {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub positions: ::prost::alloc::vec::Vec<SubaccountPosition>,
}
impl ::prost::Name for EventBatchDerivativePosition {
    const NAME: &'static str = "EventBatchDerivativePosition";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDerivativeMarketPaused {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub settle_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total_missing_funds: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub missing_funds_rate: ::prost::alloc::string::String,
}
impl ::prost::Name for EventDerivativeMarketPaused {
    const NAME: &'static str = "EventDerivativeMarketPaused";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarketBeyondBankruptcy {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub settle_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub missing_market_funds: ::prost::alloc::string::String,
}
impl ::prost::Name for EventMarketBeyondBankruptcy {
    const NAME: &'static str = "EventMarketBeyondBankruptcy";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAllPositionsHaircut {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub settle_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub missing_funds_rate: ::prost::alloc::string::String,
}
impl ::prost::Name for EventAllPositionsHaircut {
    const NAME: &'static str = "EventAllPositionsHaircut";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBinaryOptionsMarketUpdate {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<BinaryOptionsMarket>,
}
impl ::prost::Name for EventBinaryOptionsMarketUpdate {
    const NAME: &'static str = "EventBinaryOptionsMarketUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewSpotOrders {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub buy_orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
    #[prost(message, repeated, tag = "3")]
    pub sell_orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
}
impl ::prost::Name for EventNewSpotOrders {
    const NAME: &'static str = "EventNewSpotOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewDerivativeOrders {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub buy_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, repeated, tag = "3")]
    pub sell_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
}
impl ::prost::Name for EventNewDerivativeOrders {
    const NAME: &'static str = "EventNewDerivativeOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelSpotOrder {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<SpotLimitOrder>,
}
impl ::prost::Name for EventCancelSpotOrder {
    const NAME: &'static str = "EventCancelSpotOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSpotMarketUpdate {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<SpotMarket>,
}
impl ::prost::Name for EventSpotMarketUpdate {
    const NAME: &'static str = "EventSpotMarketUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPerpetualMarketUpdate {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<DerivativeMarket>,
    #[prost(message, optional, tag = "2")]
    pub perpetual_market_info: ::core::option::Option<PerpetualMarketInfo>,
    #[prost(message, optional, tag = "3")]
    pub funding: ::core::option::Option<PerpetualMarketFunding>,
}
impl ::prost::Name for EventPerpetualMarketUpdate {
    const NAME: &'static str = "EventPerpetualMarketUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExpiryFuturesMarketUpdate {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<DerivativeMarket>,
    #[prost(message, optional, tag = "3")]
    pub expiry_futures_market_info: ::core::option::Option<ExpiryFuturesMarketInfo>,
}
impl ::prost::Name for EventExpiryFuturesMarketUpdate {
    const NAME: &'static str = "EventExpiryFuturesMarketUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventPerpetualMarketFundingUpdate {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub funding: ::core::option::Option<PerpetualMarketFunding>,
    #[prost(bool, tag = "3")]
    pub is_hourly_funding: bool,
    #[prost(string, tag = "4")]
    pub funding_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub mark_price: ::prost::alloc::string::String,
}
impl ::prost::Name for EventPerpetualMarketFundingUpdate {
    const NAME: &'static str = "EventPerpetualMarketFundingUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubaccountDeposit {
    #[prost(string, tag = "1")]
    pub src_address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventSubaccountDeposit {
    const NAME: &'static str = "EventSubaccountDeposit";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubaccountWithdraw {
    #[prost(bytes = "vec", tag = "1")]
    pub subaccount_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub dst_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventSubaccountWithdraw {
    const NAME: &'static str = "EventSubaccountWithdraw";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubaccountBalanceTransfer {
    #[prost(string, tag = "1")]
    pub src_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dst_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventSubaccountBalanceTransfer {
    const NAME: &'static str = "EventSubaccountBalanceTransfer";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBatchDepositUpdate {
    #[prost(message, repeated, tag = "1")]
    pub deposit_updates: ::prost::alloc::vec::Vec<DepositUpdate>,
}
impl ::prost::Name for EventBatchDepositUpdate {
    const NAME: &'static str = "EventBatchDepositUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrderCancel {
    #[prost(message, optional, tag = "1")]
    pub market_order: ::core::option::Option<DerivativeMarketOrder>,
    #[prost(string, tag = "2")]
    pub cancel_quantity: ::prost::alloc::string::String,
}
impl ::prost::Name for DerivativeMarketOrderCancel {
    const NAME: &'static str = "DerivativeMarketOrderCancel";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelDerivativeOrder {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_limit_cancel: bool,
    #[prost(message, optional, tag = "3")]
    pub limit_order: ::core::option::Option<DerivativeLimitOrder>,
    #[prost(message, optional, tag = "4")]
    pub market_order_cancel: ::core::option::Option<DerivativeMarketOrderCancel>,
}
impl ::prost::Name for EventCancelDerivativeOrder {
    const NAME: &'static str = "EventCancelDerivativeOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFeeDiscountSchedule {
    #[prost(message, optional, tag = "1")]
    pub schedule: ::core::option::Option<FeeDiscountSchedule>,
}
impl ::prost::Name for EventFeeDiscountSchedule {
    const NAME: &'static str = "EventFeeDiscountSchedule";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTradingRewardCampaignUpdate {
    #[prost(message, optional, tag = "1")]
    pub campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag = "2")]
    pub campaign_reward_pools: ::prost::alloc::vec::Vec<CampaignRewardPool>,
}
impl ::prost::Name for EventTradingRewardCampaignUpdate {
    const NAME: &'static str = "EventTradingRewardCampaignUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTradingRewardDistribution {
    #[prost(message, repeated, tag = "1")]
    pub account_rewards: ::prost::alloc::vec::Vec<AccountRewards>,
}
impl ::prost::Name for EventTradingRewardDistribution {
    const NAME: &'static str = "EventTradingRewardDistribution";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewConditionalDerivativeOrder {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<DerivativeOrder>,
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub is_market: bool,
}
impl ::prost::Name for EventNewConditionalDerivativeOrder {
    const NAME: &'static str = "EventNewConditionalDerivativeOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelConditionalDerivativeOrder {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_limit_cancel: bool,
    #[prost(message, optional, tag = "3")]
    pub limit_order: ::core::option::Option<DerivativeLimitOrder>,
    #[prost(message, optional, tag = "4")]
    pub market_order: ::core::option::Option<DerivativeMarketOrder>,
}
impl ::prost::Name for EventCancelConditionalDerivativeOrder {
    const NAME: &'static str = "EventCancelConditionalDerivativeOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConditionalDerivativeOrderTrigger {
    #[prost(bytes = "vec", tag = "1")]
    pub market_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub is_limit_trigger: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub triggered_order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub placed_order_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub triggered_order_cid: ::prost::alloc::string::String,
}
impl ::prost::Name for EventConditionalDerivativeOrderTrigger {
    const NAME: &'static str = "EventConditionalDerivativeOrderTrigger";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOrderFail {
    #[prost(bytes = "vec", tag = "1")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, tag = "3")]
    pub flags: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, repeated, tag = "4")]
    pub cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for EventOrderFail {
    const NAME: &'static str = "EventOrderFail";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAtomicMarketOrderFeeMultipliersUpdated {
    #[prost(message, repeated, tag = "1")]
    pub market_fee_multipliers: ::prost::alloc::vec::Vec<MarketFeeMultiplier>,
}
impl ::prost::Name for EventAtomicMarketOrderFeeMultipliersUpdated {
    const NAME: &'static str = "EventAtomicMarketOrderFeeMultipliersUpdated";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOrderbookUpdate {
    #[prost(message, repeated, tag = "1")]
    pub spot_updates: ::prost::alloc::vec::Vec<OrderbookUpdate>,
    #[prost(message, repeated, tag = "2")]
    pub derivative_updates: ::prost::alloc::vec::Vec<OrderbookUpdate>,
}
impl ::prost::Name for EventOrderbookUpdate {
    const NAME: &'static str = "EventOrderbookUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookUpdate {
    #[prost(uint64, tag = "1")]
    pub seq: u64,
    #[prost(message, optional, tag = "2")]
    pub orderbook: ::core::option::Option<Orderbook>,
}
impl ::prost::Name for OrderbookUpdate {
    const NAME: &'static str = "OrderbookUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orderbook {
    #[prost(bytes = "vec", tag = "1")]
    pub market_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub buy_levels: ::prost::alloc::vec::Vec<Level>,
    #[prost(message, repeated, tag = "3")]
    pub sell_levels: ::prost::alloc::vec::Vec<Level>,
}
impl ::prost::Name for Orderbook {
    const NAME: &'static str = "Orderbook";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGrantAuthorizations {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
impl ::prost::Name for EventGrantAuthorizations {
    const NAME: &'static str = "EventGrantAuthorizations";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGrantActivation {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventGrantActivation {
    const NAME: &'static str = "EventGrantActivation";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInvalidGrant {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
}
impl ::prost::Name for EventInvalidGrant {
    const NAME: &'static str = "EventInvalidGrant";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOrderCancelFail {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
impl ::prost::Name for EventOrderCancelFail {
    const NAME: &'static str = "EventOrderCancelFail";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSpotMarket {
    /// current admin address of the associated market
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// id of the market to be updated
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// (optional) updated ticker value
    #[prost(string, tag = "3")]
    pub new_ticker: ::prost::alloc::string::String,
    /// (optional) updated min price tick size value
    #[prost(string, tag = "4")]
    pub new_min_price_tick_size: ::prost::alloc::string::String,
    /// (optional) updated min quantity tick size value
    #[prost(string, tag = "5")]
    pub new_min_quantity_tick_size: ::prost::alloc::string::String,
    /// (optional) updated min notional
    #[prost(string, tag = "6")]
    pub new_min_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateSpotMarket {
    const NAME: &'static str = "MsgUpdateSpotMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSpotMarketResponse {}
impl ::prost::Name for MsgUpdateSpotMarketResponse {
    const NAME: &'static str = "MsgUpdateSpotMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateDerivativeMarket {
    /// current admin address of the associated market
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// id of the market to be updated
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// (optional) updated value for ticker
    #[prost(string, tag = "3")]
    pub new_ticker: ::prost::alloc::string::String,
    /// (optional) updated value for min_price_tick_size
    #[prost(string, tag = "4")]
    pub new_min_price_tick_size: ::prost::alloc::string::String,
    /// (optional) updated value min_quantity_tick_size
    #[prost(string, tag = "5")]
    pub new_min_quantity_tick_size: ::prost::alloc::string::String,
    /// (optional) updated min notional
    #[prost(string, tag = "6")]
    pub new_min_notional: ::prost::alloc::string::String,
    /// (optional) updated value for initial_margin_ratio
    #[prost(string, tag = "7")]
    pub new_initial_margin_ratio: ::prost::alloc::string::String,
    /// (optional) updated value for maintenance_margin_ratio
    #[prost(string, tag = "8")]
    pub new_maintenance_margin_ratio: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateDerivativeMarket {
    const NAME: &'static str = "MsgUpdateDerivativeMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateDerivativeMarketResponse {}
impl ::prost::Name for MsgUpdateDerivativeMarketResponse {
    const NAME: &'static str = "MsgUpdateDerivativeMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the exchange parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgDeposit defines a SDK message for transferring coins from the sender's
/// bank balance into the subaccount's exchange deposits
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeposit {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// (Optional) bytes32 subaccount ID to deposit funds into. If empty, the coin
    /// will be deposited to the sender's default subaccount address.
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgDeposit {
    const NAME: &'static str = "MsgDeposit";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgDepositResponse defines the Msg/Deposit response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositResponse {}
impl ::prost::Name for MsgDepositResponse {
    const NAME: &'static str = "MsgDepositResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgWithdraw defines a SDK message for withdrawing coins from a subaccount's
/// deposits to the user's bank balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdraw {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// bytes32 subaccount ID to withdraw funds from
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgWithdraw {
    const NAME: &'static str = "MsgWithdraw";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgWithdraw defines the Msg/Withdraw response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {}
impl ::prost::Name for MsgWithdrawResponse {
    const NAME: &'static str = "MsgWithdrawResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateSpotLimitOrder defines a SDK message for creating a new spot limit
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotLimitOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<SpotOrder>,
}
impl ::prost::Name for MsgCreateSpotLimitOrder {
    const NAME: &'static str = "MsgCreateSpotLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateSpotLimitOrderResponse defines the Msg/CreateSpotOrder response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotLimitOrderResponse {
    #[prost(string, tag = "1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateSpotLimitOrderResponse {
    const NAME: &'static str = "MsgCreateSpotLimitOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCreateSpotLimitOrders defines a SDK message for creating a new batch
/// of spot limit orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateSpotLimitOrders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub orders: ::prost::alloc::vec::Vec<SpotOrder>,
}
impl ::prost::Name for MsgBatchCreateSpotLimitOrders {
    const NAME: &'static str = "MsgBatchCreateSpotLimitOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCreateSpotLimitOrdersResponse defines the
/// Msg/BatchCreateSpotLimitOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateSpotLimitOrdersResponse {
    #[prost(string, repeated, tag = "1")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub created_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub failed_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgBatchCreateSpotLimitOrdersResponse {
    const NAME: &'static str = "MsgBatchCreateSpotLimitOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantSpotMarketLaunch defines a SDK message for creating a new spot
/// market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantSpotMarketLaunch {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the spot market.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag = "3")]
    pub base_denom: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag = "4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price
    #[prost(string, tag = "5")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "6")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "7")]
    pub min_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgInstantSpotMarketLaunch {
    const NAME: &'static str = "MsgInstantSpotMarketLaunch";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantSpotMarketLaunchResponse defines the Msg/InstantSpotMarketLaunch
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantSpotMarketLaunchResponse {}
impl ::prost::Name for MsgInstantSpotMarketLaunchResponse {
    const NAME: &'static str = "MsgInstantSpotMarketLaunchResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantPerpetualMarketLaunch defines a SDK message for creating a new
/// perpetual futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantPerpetualMarketLaunch {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag = "3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag = "4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "6")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "7")]
    pub oracle_type: i32,
    /// maker_fee_rate defines the trade fee rate for makers on the perpetual
    /// market
    #[prost(string, tag = "8")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the perpetual
    /// market
    #[prost(string, tag = "9")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the perpetual
    /// market
    #[prost(string, tag = "10")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// perpetual market
    #[prost(string, tag = "11")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "12")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "13")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "14")]
    pub min_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgInstantPerpetualMarketLaunch {
    const NAME: &'static str = "MsgInstantPerpetualMarketLaunch";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantPerpetualMarketLaunchResponse defines the
/// Msg/InstantPerpetualMarketLaunchResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantPerpetualMarketLaunchResponse {}
impl ::prost::Name for MsgInstantPerpetualMarketLaunchResponse {
    const NAME: &'static str = "MsgInstantPerpetualMarketLaunchResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantBinaryOptionsMarketLaunch defines a SDK message for creating a new
/// perpetual futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantBinaryOptionsMarketLaunch {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative contract.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag = "3")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag = "4")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "5")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "6")]
    pub oracle_scale_factor: u32,
    /// maker_fee_rate defines the trade fee rate for makers on the perpetual
    /// market
    #[prost(string, tag = "7")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the perpetual
    /// market
    #[prost(string, tag = "8")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag = "9")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag = "10")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag = "11")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag = "12")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag = "13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag = "14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "15")]
    pub min_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgInstantBinaryOptionsMarketLaunch {
    const NAME: &'static str = "MsgInstantBinaryOptionsMarketLaunch";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantBinaryOptionsMarketLaunchResponse defines the
/// Msg/InstantBinaryOptionsMarketLaunchResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantBinaryOptionsMarketLaunchResponse {}
impl ::prost::Name for MsgInstantBinaryOptionsMarketLaunchResponse {
    const NAME: &'static str = "MsgInstantBinaryOptionsMarketLaunchResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantExpiryFuturesMarketLaunch defines a SDK message for creating a new
/// expiry futures market by paying listing fee without governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantExpiryFuturesMarketLaunch {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag = "3")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag = "4")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "5")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "6")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "7")]
    pub oracle_scale_factor: u32,
    /// Expiration time of the market
    #[prost(int64, tag = "8")]
    pub expiry: i64,
    /// maker_fee_rate defines the trade fee rate for makers on the expiry futures
    /// market
    #[prost(string, tag = "9")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the expiry futures
    /// market
    #[prost(string, tag = "10")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag = "11")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag = "12")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "15")]
    pub min_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgInstantExpiryFuturesMarketLaunch {
    const NAME: &'static str = "MsgInstantExpiryFuturesMarketLaunch";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgInstantExpiryFuturesMarketLaunchResponse defines the
/// Msg/InstantExpiryFuturesMarketLaunch response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantExpiryFuturesMarketLaunchResponse {}
impl ::prost::Name for MsgInstantExpiryFuturesMarketLaunchResponse {
    const NAME: &'static str = "MsgInstantExpiryFuturesMarketLaunchResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateSpotMarketOrder defines a SDK message for creating a new spot market
/// order.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotMarketOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<SpotOrder>,
}
impl ::prost::Name for MsgCreateSpotMarketOrder {
    const NAME: &'static str = "MsgCreateSpotMarketOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateSpotMarketOrderResponse defines the Msg/CreateSpotMarketLimitOrder
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSpotMarketOrderResponse {
    #[prost(string, tag = "1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub results: ::core::option::Option<SpotMarketOrderResults>,
    #[prost(string, tag = "3")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateSpotMarketOrderResponse {
    const NAME: &'static str = "MsgCreateSpotMarketOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketOrderResults {
    #[prost(string, tag = "1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub fee: ::prost::alloc::string::String,
}
impl ::prost::Name for SpotMarketOrderResults {
    const NAME: &'static str = "SpotMarketOrderResults";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgCreateDerivativeLimitOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeLimitOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
impl ::prost::Name for MsgCreateDerivativeLimitOrder {
    const NAME: &'static str = "MsgCreateDerivativeLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateDerivativeLimitOrderResponse defines the
/// Msg/CreateDerivativeMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeLimitOrderResponse {
    #[prost(string, tag = "1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateDerivativeLimitOrderResponse {
    const NAME: &'static str = "MsgCreateDerivativeLimitOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgCreateBinaryOptionsLimitOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsLimitOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
impl ::prost::Name for MsgCreateBinaryOptionsLimitOrder {
    const NAME: &'static str = "MsgCreateBinaryOptionsLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateBinaryOptionsLimitOrderResponse defines the
/// Msg/CreateBinaryOptionsLimitOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsLimitOrderResponse {
    #[prost(string, tag = "1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateBinaryOptionsLimitOrderResponse {
    const NAME: &'static str = "MsgCreateBinaryOptionsLimitOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgBatchCreateDerivativeLimitOrders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateDerivativeLimitOrders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeOrder>,
}
impl ::prost::Name for MsgBatchCreateDerivativeLimitOrders {
    const NAME: &'static str = "MsgBatchCreateDerivativeLimitOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCreateDerivativeLimitOrdersResponse defines the
/// Msg/BatchCreateDerivativeLimitOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCreateDerivativeLimitOrdersResponse {
    #[prost(string, repeated, tag = "1")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub created_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub failed_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgBatchCreateDerivativeLimitOrdersResponse {
    const NAME: &'static str = "MsgBatchCreateDerivativeLimitOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCancelSpotOrder defines the Msg/CancelSpotOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSpotOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCancelSpotOrder {
    const NAME: &'static str = "MsgCancelSpotOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCancelSpotOrderResponse defines the Msg/CancelSpotOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSpotOrderResponse {}
impl ::prost::Name for MsgCancelSpotOrderResponse {
    const NAME: &'static str = "MsgCancelSpotOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCancelSpotOrders defines the Msg/BatchCancelSpotOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelSpotOrders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
impl ::prost::Name for MsgBatchCancelSpotOrders {
    const NAME: &'static str = "MsgBatchCancelSpotOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCancelSpotOrdersResponse defines the Msg/BatchCancelSpotOrders
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelSpotOrdersResponse {
    #[prost(bool, repeated, tag = "1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
impl ::prost::Name for MsgBatchCancelSpotOrdersResponse {
    const NAME: &'static str = "MsgBatchCancelSpotOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCancelBinaryOptionsOrders defines the
/// Msg/BatchCancelBinaryOptionsOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelBinaryOptionsOrders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
impl ::prost::Name for MsgBatchCancelBinaryOptionsOrders {
    const NAME: &'static str = "MsgBatchCancelBinaryOptionsOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// BatchCancelBinaryOptionsOrdersResponse defines the
/// Msg/BatchCancelBinaryOptionsOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelBinaryOptionsOrdersResponse {
    #[prost(bool, repeated, tag = "1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
impl ::prost::Name for MsgBatchCancelBinaryOptionsOrdersResponse {
    const NAME: &'static str = "MsgBatchCancelBinaryOptionsOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchUpdateOrders defines the Msg/BatchUpdateOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchUpdateOrders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subaccount_id only used for the spot_market_ids_to_cancel_all and
    /// derivative_market_ids_to_cancel_all.
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub spot_market_ids_to_cancel_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub derivative_market_ids_to_cancel_all:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub spot_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(message, repeated, tag = "6")]
    pub derivative_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(message, repeated, tag = "7")]
    pub spot_orders_to_create: ::prost::alloc::vec::Vec<SpotOrder>,
    #[prost(message, repeated, tag = "8")]
    pub derivative_orders_to_create: ::prost::alloc::vec::Vec<DerivativeOrder>,
    #[prost(message, repeated, tag = "9")]
    pub binary_options_orders_to_cancel: ::prost::alloc::vec::Vec<OrderData>,
    #[prost(string, repeated, tag = "10")]
    pub binary_options_market_ids_to_cancel_all:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "11")]
    pub binary_options_orders_to_create: ::prost::alloc::vec::Vec<DerivativeOrder>,
}
impl ::prost::Name for MsgBatchUpdateOrders {
    const NAME: &'static str = "MsgBatchUpdateOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchUpdateOrdersResponse defines the Msg/BatchUpdateOrders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchUpdateOrdersResponse {
    #[prost(bool, repeated, tag = "1")]
    pub spot_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(bool, repeated, tag = "2")]
    pub derivative_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag = "3")]
    pub spot_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub derivative_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, repeated, tag = "5")]
    pub binary_options_cancel_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag = "6")]
    pub binary_options_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub created_spot_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub failed_spot_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "9")]
    pub created_derivative_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "10")]
    pub failed_derivative_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "11")]
    pub created_binary_options_orders_cids:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "12")]
    pub failed_binary_options_orders_cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgBatchUpdateOrdersResponse {
    const NAME: &'static str = "MsgBatchUpdateOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgCreateDerivativeMarketOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeMarketOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
impl ::prost::Name for MsgCreateDerivativeMarketOrder {
    const NAME: &'static str = "MsgCreateDerivativeMarketOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateDerivativeMarketOrderResponse defines the
/// Msg/CreateDerivativeMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDerivativeMarketOrderResponse {
    #[prost(string, tag = "1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub results: ::core::option::Option<DerivativeMarketOrderResults>,
    #[prost(string, tag = "3")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateDerivativeMarketOrderResponse {
    const NAME: &'static str = "MsgCreateDerivativeMarketOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketOrderResults {
    #[prost(string, tag = "1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub fee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    #[prost(string, tag = "5")]
    pub payout: ::prost::alloc::string::String,
}
impl ::prost::Name for DerivativeMarketOrderResults {
    const NAME: &'static str = "DerivativeMarketOrderResults";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgCreateBinaryOptionsMarketOrder
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsMarketOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
impl ::prost::Name for MsgCreateBinaryOptionsMarketOrder {
    const NAME: &'static str = "MsgCreateBinaryOptionsMarketOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCreateBinaryOptionsMarketOrderResponse defines the
/// Msg/CreateBinaryOptionsMarketOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBinaryOptionsMarketOrderResponse {
    #[prost(string, tag = "1")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub results: ::core::option::Option<DerivativeMarketOrderResults>,
    #[prost(string, tag = "3")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateBinaryOptionsMarketOrderResponse {
    const NAME: &'static str = "MsgCreateBinaryOptionsMarketOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCancelDerivativeOrder defines the Msg/CancelDerivativeOrder response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelDerivativeOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag = "5")]
    pub order_mask: i32,
    #[prost(string, tag = "6")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCancelDerivativeOrder {
    const NAME: &'static str = "MsgCancelDerivativeOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCancelDerivativeOrderResponse defines the
/// Msg/CancelDerivativeOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelDerivativeOrderResponse {}
impl ::prost::Name for MsgCancelDerivativeOrderResponse {
    const NAME: &'static str = "MsgCancelDerivativeOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCancelBinaryOptionsOrder defines the Msg/CancelBinaryOptionsOrder response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBinaryOptionsOrder {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag = "5")]
    pub order_mask: i32,
    #[prost(string, tag = "6")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCancelBinaryOptionsOrder {
    const NAME: &'static str = "MsgCancelBinaryOptionsOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgCancelBinaryOptionsOrderResponse defines the
/// Msg/CancelBinaryOptionsOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBinaryOptionsOrderResponse {}
impl ::prost::Name for MsgCancelBinaryOptionsOrderResponse {
    const NAME: &'static str = "MsgCancelBinaryOptionsOrderResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderData {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub order_hash: ::prost::alloc::string::String,
    /// bitwise combination of OrderMask enum values
    #[prost(int32, tag = "4")]
    pub order_mask: i32,
    #[prost(string, tag = "5")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for OrderData {
    const NAME: &'static str = "OrderData";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCancelDerivativeOrders defines the Msg/CancelDerivativeOrders
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelDerivativeOrders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<OrderData>,
}
impl ::prost::Name for MsgBatchCancelDerivativeOrders {
    const NAME: &'static str = "MsgBatchCancelDerivativeOrders";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgBatchCancelDerivativeOrdersResponse defines the
/// Msg/CancelDerivativeOrderResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBatchCancelDerivativeOrdersResponse {
    #[prost(bool, repeated, tag = "1")]
    pub success: ::prost::alloc::vec::Vec<bool>,
}
impl ::prost::Name for MsgBatchCancelDerivativeOrdersResponse {
    const NAME: &'static str = "MsgBatchCancelDerivativeOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgSubaccountTransfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubaccountTransfer {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgSubaccountTransfer {
    const NAME: &'static str = "MsgSubaccountTransfer";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgSubaccountTransferResponse defines the Msg/SubaccountTransfer response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubaccountTransferResponse {}
impl ::prost::Name for MsgSubaccountTransferResponse {
    const NAME: &'static str = "MsgSubaccountTransferResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgExternalTransfer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExternalTransfer {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgExternalTransfer {
    const NAME: &'static str = "MsgExternalTransfer";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgExternalTransferResponse defines the Msg/ExternalTransfer response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExternalTransferResponse {}
impl ::prost::Name for MsgExternalTransferResponse {
    const NAME: &'static str = "MsgExternalTransferResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgLiquidatePosition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidatePosition {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_id: ::prost::alloc::string::String,
    /// optional order to provide for liquidation
    #[prost(message, optional, tag = "4")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
impl ::prost::Name for MsgLiquidatePosition {
    const NAME: &'static str = "MsgLiquidatePosition";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgLiquidatePositionResponse defines the Msg/LiquidatePosition response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidatePositionResponse {}
impl ::prost::Name for MsgLiquidatePositionResponse {
    const NAME: &'static str = "MsgLiquidatePositionResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgEmergencySettleMarket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEmergencySettleMarket {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgEmergencySettleMarket {
    const NAME: &'static str = "MsgEmergencySettleMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgEmergencySettleMarketResponse defines the Msg/EmergencySettleMarket
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEmergencySettleMarketResponse {}
impl ::prost::Name for MsgEmergencySettleMarketResponse {
    const NAME: &'static str = "MsgEmergencySettleMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgIncreasePositionMargin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreasePositionMargin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub market_id: ::prost::alloc::string::String,
    /// amount defines the amount of margin to add to the position
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgIncreasePositionMargin {
    const NAME: &'static str = "MsgIncreasePositionMargin";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgIncreasePositionMarginResponse defines the Msg/IncreasePositionMargin
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIncreasePositionMarginResponse {}
impl ::prost::Name for MsgIncreasePositionMarginResponse {
    const NAME: &'static str = "MsgIncreasePositionMarginResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgDecreasePositionMargin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDecreasePositionMargin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub destination_subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub market_id: ::prost::alloc::string::String,
    /// amount defines the amount of margin to withdraw from the position
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgDecreasePositionMargin {
    const NAME: &'static str = "MsgDecreasePositionMargin";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgDecreasePositionMarginResponse defines the Msg/MsgDecreasePositionMargin
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDecreasePositionMarginResponse {}
impl ::prost::Name for MsgDecreasePositionMarginResponse {
    const NAME: &'static str = "MsgDecreasePositionMarginResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgPrivilegedExecuteContract defines the Msg/Exec message type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPrivilegedExecuteContract {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// funds defines the user's bank coins used to fund the execution (e.g.
    /// 100inj).
    #[prost(string, tag = "2")]
    pub funds: ::prost::alloc::string::String,
    /// contract_address defines the contract address to execute
    #[prost(string, tag = "3")]
    pub contract_address: ::prost::alloc::string::String,
    /// data defines the call data used when executing the contract
    #[prost(string, tag = "4")]
    pub data: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgPrivilegedExecuteContract {
    const NAME: &'static str = "MsgPrivilegedExecuteContract";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgPrivilegedExecuteContractResponse defines the Msg/Exec response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPrivilegedExecuteContractResponse {
    #[prost(message, repeated, tag = "1")]
    pub funds_diff: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgPrivilegedExecuteContractResponse {
    const NAME: &'static str = "MsgPrivilegedExecuteContractResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgRewardsOptOut
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRewardsOptOut {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRewardsOptOut {
    const NAME: &'static str = "MsgRewardsOptOut";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgRewardsOptOutResponse defines the Msg/RewardsOptOut response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRewardsOptOutResponse {}
impl ::prost::Name for MsgRewardsOptOutResponse {
    const NAME: &'static str = "MsgRewardsOptOutResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// A Cosmos-SDK MsgReclaimLockedFunds
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReclaimLockedFunds {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub locked_account_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MsgReclaimLockedFunds {
    const NAME: &'static str = "MsgReclaimLockedFunds";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgReclaimLockedFundsResponse defines the Msg/ReclaimLockedFunds response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReclaimLockedFundsResponse {}
impl ::prost::Name for MsgReclaimLockedFundsResponse {
    const NAME: &'static str = "MsgReclaimLockedFundsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgSignData defines an arbitrary, general-purpose, off-chain message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignData {
    /// Signer is the sdk.AccAddress of the message signer
    #[prost(bytes = "vec", tag = "1")]
    pub signer: ::prost::alloc::vec::Vec<u8>,
    /// Data represents the raw bytes of the content that is signed (text, json,
    /// etc)
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MsgSignData {
    const NAME: &'static str = "MsgSignData";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgSignDoc defines an arbitrary, general-purpose, off-chain message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSignDoc {
    #[prost(string, tag = "1")]
    pub sign_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<MsgSignData>,
}
impl ::prost::Name for MsgSignDoc {
    const NAME: &'static str = "MsgSignDoc";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgAdminUpdateBinaryOptionsMarket is used by the market Admin to operate the
/// market
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAdminUpdateBinaryOptionsMarket {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    /// new price at which market will be settled
    #[prost(string, tag = "3")]
    pub settlement_price: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag = "4")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag = "5")]
    pub settlement_timestamp: i64,
    /// Status of the market
    #[prost(enumeration = "MarketStatus", tag = "6")]
    pub status: i32,
}
impl ::prost::Name for MsgAdminUpdateBinaryOptionsMarket {
    const NAME: &'static str = "MsgAdminUpdateBinaryOptionsMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgAdminUpdateBinaryOptionsMarketResponse is the response for
/// AdminUpdateBinaryOptionsMarket rpc method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAdminUpdateBinaryOptionsMarketResponse {}
impl ::prost::Name for MsgAdminUpdateBinaryOptionsMarketResponse {
    const NAME: &'static str = "MsgAdminUpdateBinaryOptionsMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgAuthorizeStakeGrants grants stakes to grantees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAuthorizeStakeGrants {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
impl ::prost::Name for MsgAuthorizeStakeGrants {
    const NAME: &'static str = "MsgAuthorizeStakeGrants";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAuthorizeStakeGrantsResponse {}
impl ::prost::Name for MsgAuthorizeStakeGrantsResponse {
    const NAME: &'static str = "MsgAuthorizeStakeGrantsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MsgActivateStakeGrant allows a grantee to activate a stake grant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateStakeGrant {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub granter: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgActivateStakeGrant {
    const NAME: &'static str = "MsgActivateStakeGrant";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateStakeGrantResponse {}
impl ::prost::Name for MsgActivateStakeGrantResponse {
    const NAME: &'static str = "MsgActivateStakeGrantResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// GenesisState defines the exchange module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to exchange.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// spot_markets is an array containing the genesis trade pairs
    #[prost(message, repeated, tag = "2")]
    pub spot_markets: ::prost::alloc::vec::Vec<SpotMarket>,
    /// derivative_markets is an array containing the genesis derivative markets
    #[prost(message, repeated, tag = "3")]
    pub derivative_markets: ::prost::alloc::vec::Vec<DerivativeMarket>,
    /// spot_orderbook defines the spot exchange limit orderbook active at genesis.
    #[prost(message, repeated, tag = "4")]
    pub spot_orderbook: ::prost::alloc::vec::Vec<SpotOrderBook>,
    /// derivative_orderbook defines the derivative exchange limit orderbook active
    /// at genesis.
    #[prost(message, repeated, tag = "5")]
    pub derivative_orderbook: ::prost::alloc::vec::Vec<DerivativeOrderBook>,
    /// balances defines the exchange users balances active at genesis.
    #[prost(message, repeated, tag = "6")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    /// positions defines the exchange derivative positions at genesis
    #[prost(message, repeated, tag = "7")]
    pub positions: ::prost::alloc::vec::Vec<DerivativePosition>,
    /// subaccount_trade_nonces defines the subaccount trade nonces for the
    /// subaccounts at genesis
    #[prost(message, repeated, tag = "8")]
    pub subaccount_trade_nonces: ::prost::alloc::vec::Vec<SubaccountNonce>,
    /// expiry_futures_market_info defines the market info for the expiry futures
    /// markets at genesis
    #[prost(message, repeated, tag = "9")]
    pub expiry_futures_market_info_state: ::prost::alloc::vec::Vec<ExpiryFuturesMarketInfoState>,
    /// perpetual_market_info defines the market info for the perpetual derivative
    /// markets at genesis
    #[prost(message, repeated, tag = "10")]
    pub perpetual_market_info: ::prost::alloc::vec::Vec<PerpetualMarketInfo>,
    /// perpetual_market_funding_state defines the funding state for the perpetual
    /// derivative markets at genesis
    #[prost(message, repeated, tag = "11")]
    pub perpetual_market_funding_state: ::prost::alloc::vec::Vec<PerpetualMarketFundingState>,
    /// derivative_market_settlement_scheduled defines the scheduled markets for
    /// settlement at genesis
    #[prost(message, repeated, tag = "12")]
    pub derivative_market_settlement_scheduled:
        ::prost::alloc::vec::Vec<DerivativeMarketSettlementInfo>,
    /// sets spot markets as enabled
    #[prost(bool, tag = "13")]
    pub is_spot_exchange_enabled: bool,
    /// sets derivative markets as enabled
    #[prost(bool, tag = "14")]
    pub is_derivatives_exchange_enabled: bool,
    /// the current trading reward campaign info
    #[prost(message, optional, tag = "15")]
    pub trading_reward_campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    /// the current and upcoming trading reward campaign pools
    #[prost(message, repeated, tag = "16")]
    pub trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    /// the current trading reward account points
    #[prost(message, repeated, tag = "17")]
    pub trading_reward_campaign_account_points:
        ::prost::alloc::vec::Vec<TradingRewardCampaignAccountPoints>,
    /// the fee discount schedule
    #[prost(message, optional, tag = "18")]
    pub fee_discount_schedule: ::core::option::Option<FeeDiscountSchedule>,
    /// the cached fee discount account tiers with TTL
    #[prost(message, repeated, tag = "19")]
    pub fee_discount_account_tier_ttl: ::prost::alloc::vec::Vec<FeeDiscountAccountTierTtl>,
    /// the fee discount paid by accounts in all buckets
    #[prost(message, repeated, tag = "20")]
    pub fee_discount_bucket_volume_accounts:
        ::prost::alloc::vec::Vec<FeeDiscountBucketVolumeAccounts>,
    /// sets the first fee cycle as finished
    #[prost(bool, tag = "21")]
    pub is_first_fee_cycle_finished: bool,
    /// the current and upcoming trading reward campaign pending pools
    #[prost(message, repeated, tag = "22")]
    pub pending_trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    /// the pending trading reward account points
    #[prost(message, repeated, tag = "23")]
    pub pending_trading_reward_campaign_account_points:
        ::prost::alloc::vec::Vec<TradingRewardCampaignAccountPendingPoints>,
    /// the addresses opting out of trading rewards
    #[prost(string, repeated, tag = "24")]
    pub rewards_opt_out_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "25")]
    pub historical_trade_records: ::prost::alloc::vec::Vec<TradeRecords>,
    /// binary_options_markets is an array containing the genesis binary options
    /// markets
    #[prost(message, repeated, tag = "26")]
    pub binary_options_markets: ::prost::alloc::vec::Vec<BinaryOptionsMarket>,
    /// binary_options_markets_scheduled_for_settlement contains the marketIDs of
    /// binary options markets scheduled for next-block settlement
    #[prost(string, repeated, tag = "27")]
    pub binary_options_market_ids_scheduled_for_settlement:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// spot_market_ids_scheduled_to_force_close defines the scheduled markets for
    /// forced closings at genesis
    #[prost(string, repeated, tag = "28")]
    pub spot_market_ids_scheduled_to_force_close:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// denom_decimals defines the denom decimals for the exchange.
    #[prost(message, repeated, tag = "29")]
    pub denom_decimals: ::prost::alloc::vec::Vec<DenomDecimals>,
    /// conditional_derivative_orderbook contains conditional orderbooks for all
    /// markets (both lmit and market conditional orders)
    #[prost(message, repeated, tag = "30")]
    pub conditional_derivative_orderbooks: ::prost::alloc::vec::Vec<ConditionalDerivativeOrderBook>,
    /// market_fee_multipliers contains any non-default atomic order fee
    /// multipliers
    #[prost(message, repeated, tag = "31")]
    pub market_fee_multipliers: ::prost::alloc::vec::Vec<MarketFeeMultiplier>,
    #[prost(message, repeated, tag = "32")]
    pub orderbook_sequences: ::prost::alloc::vec::Vec<OrderbookSequence>,
    #[prost(message, repeated, tag = "33")]
    pub subaccount_volumes: ::prost::alloc::vec::Vec<AggregateSubaccountVolumeRecord>,
    #[prost(message, repeated, tag = "34")]
    pub market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
    #[prost(message, repeated, tag = "35")]
    pub grant_authorizations: ::prost::alloc::vec::Vec<FullGrantAuthorizations>,
    #[prost(message, repeated, tag = "36")]
    pub active_grants: ::prost::alloc::vec::Vec<FullActiveGrant>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookSequence {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for OrderbookSequence {
    const NAME: &'static str = "OrderbookSequence";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountAccountTierTtl {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub tier_ttl: ::core::option::Option<FeeDiscountTierTtl>,
}
impl ::prost::Name for FeeDiscountAccountTierTtl {
    const NAME: &'static str = "FeeDiscountAccountTierTTL";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountBucketVolumeAccounts {
    #[prost(int64, tag = "1")]
    pub bucket_start_timestamp: i64,
    #[prost(message, repeated, tag = "2")]
    pub account_volume: ::prost::alloc::vec::Vec<AccountVolume>,
}
impl ::prost::Name for FeeDiscountBucketVolumeAccounts {
    const NAME: &'static str = "FeeDiscountBucketVolumeAccounts";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountVolume {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub volume: ::prost::alloc::string::String,
}
impl ::prost::Name for AccountVolume {
    const NAME: &'static str = "AccountVolume";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignAccountPoints {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub points: ::prost::alloc::string::String,
}
impl ::prost::Name for TradingRewardCampaignAccountPoints {
    const NAME: &'static str = "TradingRewardCampaignAccountPoints";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignAccountPendingPoints {
    #[prost(int64, tag = "1")]
    pub reward_pool_start_timestamp: i64,
    #[prost(message, repeated, tag = "2")]
    pub account_points: ::prost::alloc::vec::Vec<TradingRewardCampaignAccountPoints>,
}
impl ::prost::Name for TradingRewardCampaignAccountPendingPoints {
    const NAME: &'static str = "TradingRewardCampaignAccountPendingPoints";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// Spot Exchange Limit Orderbook
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrderBook {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_buy_side: bool,
    #[prost(message, repeated, tag = "3")]
    pub orders: ::prost::alloc::vec::Vec<SpotLimitOrder>,
}
impl ::prost::Name for SpotOrderBook {
    const NAME: &'static str = "SpotOrderBook";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// Derivative Exchange Limit Orderbook
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrderBook {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_buy_side: bool,
    #[prost(message, repeated, tag = "3")]
    pub orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
}
impl ::prost::Name for DerivativeOrderBook {
    const NAME: &'static str = "DerivativeOrderBook";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// Orderbook containing limit & market conditional orders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalDerivativeOrderBook {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub limit_buy_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, repeated, tag = "3")]
    pub market_buy_orders: ::prost::alloc::vec::Vec<DerivativeMarketOrder>,
    #[prost(message, repeated, tag = "4")]
    pub limit_sell_orders: ::prost::alloc::vec::Vec<DerivativeLimitOrder>,
    #[prost(message, repeated, tag = "5")]
    pub market_sell_orders: ::prost::alloc::vec::Vec<DerivativeMarketOrder>,
}
impl ::prost::Name for ConditionalDerivativeOrderBook {
    const NAME: &'static str = "ConditionalDerivativeOrderBook";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub deposits: ::core::option::Option<Deposit>,
}
impl ::prost::Name for Balance {
    const NAME: &'static str = "Balance";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativePosition {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub position: ::core::option::Option<Position>,
}
impl ::prost::Name for DerivativePosition {
    const NAME: &'static str = "DerivativePosition";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountNonce {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub subaccount_trade_nonce: ::core::option::Option<SubaccountTradeNonce>,
}
impl ::prost::Name for SubaccountNonce {
    const NAME: &'static str = "SubaccountNonce";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketInfoState {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub market_info: ::core::option::Option<ExpiryFuturesMarketInfo>,
}
impl ::prost::Name for ExpiryFuturesMarketInfoState {
    const NAME: &'static str = "ExpiryFuturesMarketInfoState";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketFundingState {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub funding: ::core::option::Option<PerpetualMarketFunding>,
}
impl ::prost::Name for PerpetualMarketFundingState {
    const NAME: &'static str = "PerpetualMarketFundingState";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullGrantAuthorizations {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub total_grant_amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub last_delegations_checked_time: i64,
    #[prost(message, repeated, tag = "4")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
impl ::prost::Name for FullGrantAuthorizations {
    const NAME: &'static str = "FullGrantAuthorizations";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullActiveGrant {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub active_grant: ::core::option::Option<ActiveGrant>,
}
impl ::prost::Name for FullActiveGrant {
    const NAME: &'static str = "FullActiveGrant";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketParamUpdateProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the trade fee rate for makers on the spot market
    #[prost(string, tag = "4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the trade fee rate for takers on the spot market
    #[prost(string, tag = "5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the relayer fee share rate for the spot
    /// market
    #[prost(string, tag = "6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "7")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "8")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    #[prost(enumeration = "MarketStatus", tag = "9")]
    pub status: i32,
    #[prost(string, tag = "10")]
    pub ticker: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "11")]
    pub min_notional: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "12")]
    pub admin_info: ::core::option::Option<AdminInfo>,
}
impl ::prost::Name for SpotMarketParamUpdateProposal {
    const NAME: &'static str = "SpotMarketParamUpdateProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeEnableProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "ExchangeType", tag = "3")]
    pub exchange_type: i32,
}
impl ::prost::Name for ExchangeEnableProposal {
    const NAME: &'static str = "ExchangeEnableProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchExchangeModificationProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub spot_market_param_update_proposals: ::prost::alloc::vec::Vec<SpotMarketParamUpdateProposal>,
    #[prost(message, repeated, tag = "4")]
    pub derivative_market_param_update_proposals:
        ::prost::alloc::vec::Vec<DerivativeMarketParamUpdateProposal>,
    #[prost(message, repeated, tag = "5")]
    pub spot_market_launch_proposals: ::prost::alloc::vec::Vec<SpotMarketLaunchProposal>,
    #[prost(message, repeated, tag = "6")]
    pub perpetual_market_launch_proposals: ::prost::alloc::vec::Vec<PerpetualMarketLaunchProposal>,
    #[prost(message, repeated, tag = "7")]
    pub expiry_futures_market_launch_proposals:
        ::prost::alloc::vec::Vec<ExpiryFuturesMarketLaunchProposal>,
    #[prost(message, optional, tag = "8")]
    pub trading_reward_campaign_update_proposal:
        ::core::option::Option<TradingRewardCampaignUpdateProposal>,
    #[prost(message, repeated, tag = "9")]
    pub binary_options_market_launch_proposals:
        ::prost::alloc::vec::Vec<BinaryOptionsMarketLaunchProposal>,
    #[prost(message, repeated, tag = "10")]
    pub binary_options_param_update_proposals:
        ::prost::alloc::vec::Vec<BinaryOptionsMarketParamUpdateProposal>,
    #[prost(message, optional, tag = "11")]
    pub denom_decimals_update_proposal: ::core::option::Option<UpdateDenomDecimalsProposal>,
    #[prost(message, optional, tag = "12")]
    pub fee_discount_proposal: ::core::option::Option<FeeDiscountProposal>,
    #[prost(message, repeated, tag = "13")]
    pub market_forced_settlement_proposals:
        ::prost::alloc::vec::Vec<MarketForcedSettlementProposal>,
}
impl ::prost::Name for BatchExchangeModificationProposal {
    const NAME: &'static str = "BatchExchangeModificationProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// SpotMarketLaunchProposal defines a SDK message for proposing a new spot
/// market through governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketLaunchProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the spot market.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag = "4")]
    pub base_denom: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag = "5")]
    pub quote_denom: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price
    #[prost(string, tag = "6")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "7")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// maker_fee_rate defines the fee percentage makers pay when trading
    #[prost(string, tag = "8")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the fee percentage takers pay when trading
    #[prost(string, tag = "9")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional for orders in the market
    #[prost(string, tag = "10")]
    pub min_notional: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub admin_info: ::core::option::Option<AdminInfo>,
}
impl ::prost::Name for SpotMarketLaunchProposal {
    const NAME: &'static str = "SpotMarketLaunchProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// PerpetualMarketLaunchProposal defines a SDK message for proposing a new
/// perpetual futures market through governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketLaunchProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the base currency
    #[prost(string, tag = "4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag = "5")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "6")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "7")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "8")]
    pub oracle_type: i32,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag = "9")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag = "10")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag = "11")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag = "12")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "13")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "14")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "15")]
    pub min_notional: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "16")]
    pub admin_info: ::core::option::Option<AdminInfo>,
}
impl ::prost::Name for PerpetualMarketLaunchProposal {
    const NAME: &'static str = "PerpetualMarketLaunchProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketLaunchProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the derivative contract.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// Oracle symbol
    #[prost(string, tag = "4")]
    pub oracle_symbol: ::prost::alloc::string::String,
    /// Oracle Provider
    #[prost(string, tag = "5")]
    pub oracle_provider: ::prost::alloc::string::String,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "6")]
    pub oracle_type: i32,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "7")]
    pub oracle_scale_factor: u32,
    /// expiration timestamp
    #[prost(int64, tag = "8")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag = "9")]
    pub settlement_timestamp: i64,
    /// admin of the market
    #[prost(string, tag = "10")]
    pub admin: ::prost::alloc::string::String,
    /// Address of the quote currency denomination for the binary options contract
    #[prost(string, tag = "11")]
    pub quote_denom: ::prost::alloc::string::String,
    /// maker_fee_rate defines the maker fee rate of a binary options market
    #[prost(string, tag = "12")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the taker fee rate of a derivative market
    #[prost(string, tag = "13")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size that the price and margin
    /// required for orders in the market
    #[prost(string, tag = "14")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the quantity
    /// required for orders in the market
    #[prost(string, tag = "15")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "16")]
    pub min_notional: ::prost::alloc::string::String,
    #[prost(uint32, tag = "17")]
    pub admin_permissions: u32,
}
impl ::prost::Name for BinaryOptionsMarketLaunchProposal {
    const NAME: &'static str = "BinaryOptionsMarketLaunchProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// ExpiryFuturesMarketLaunchProposal defines a SDK message for proposing a new
/// expiry futures market through governance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiryFuturesMarketLaunchProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Ticker for the derivative market.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// type of coin to use as the quote currency
    #[prost(string, tag = "4")]
    pub quote_denom: ::prost::alloc::string::String,
    /// Oracle base currency
    #[prost(string, tag = "5")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "6")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "7")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "8")]
    pub oracle_type: i32,
    /// Expiration time of the market
    #[prost(int64, tag = "9")]
    pub expiry: i64,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag = "10")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag = "11")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag = "12")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag = "13")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "14")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "15")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "16")]
    pub min_notional: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "17")]
    pub admin_info: ::core::option::Option<AdminInfo>,
}
impl ::prost::Name for ExpiryFuturesMarketLaunchProposal {
    const NAME: &'static str = "ExpiryFuturesMarketLaunchProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketParamUpdateProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_id: ::prost::alloc::string::String,
    /// initial_margin_ratio defines the initial margin ratio for the derivative
    /// market
    #[prost(string, tag = "4")]
    pub initial_margin_ratio: ::prost::alloc::string::String,
    /// maintenance_margin_ratio defines the maintenance margin ratio for the
    /// derivative market
    #[prost(string, tag = "5")]
    pub maintenance_margin_ratio: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag = "6")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag = "7")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the relayer fee share rate for the
    /// derivative market
    #[prost(string, tag = "8")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "9")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "10")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// hourly_interest_rate defines the hourly interest rate
    #[prost(string, tag = "11")]
    pub hourly_interest_rate: ::prost::alloc::string::String,
    /// hourly_funding_rate_cap defines the maximum absolute value of the hourly
    /// funding rate
    #[prost(string, tag = "12")]
    pub hourly_funding_rate_cap: ::prost::alloc::string::String,
    #[prost(enumeration = "MarketStatus", tag = "13")]
    pub status: i32,
    #[prost(message, optional, tag = "14")]
    pub oracle_params: ::core::option::Option<OracleParams>,
    #[prost(string, tag = "15")]
    pub ticker: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "16")]
    pub min_notional: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "17")]
    pub admin_info: ::core::option::Option<AdminInfo>,
}
impl ::prost::Name for DerivativeMarketParamUpdateProposal {
    const NAME: &'static str = "DerivativeMarketParamUpdateProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminInfo {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub admin_permissions: u32,
}
impl ::prost::Name for AdminInfo {
    const NAME: &'static str = "AdminInfo";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketForcedSettlementProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub settlement_price: ::prost::alloc::string::String,
}
impl ::prost::Name for MarketForcedSettlementProposal {
    const NAME: &'static str = "MarketForcedSettlementProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDenomDecimalsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub denom_decimals: ::prost::alloc::vec::Vec<DenomDecimals>,
}
impl ::prost::Name for UpdateDenomDecimalsProposal {
    const NAME: &'static str = "UpdateDenomDecimalsProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryOptionsMarketParamUpdateProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_id: ::prost::alloc::string::String,
    /// maker_fee_rate defines the exchange trade fee for makers for the derivative
    /// market
    #[prost(string, tag = "4")]
    pub maker_fee_rate: ::prost::alloc::string::String,
    /// taker_fee_rate defines the exchange trade fee for takers for the derivative
    /// market
    #[prost(string, tag = "5")]
    pub taker_fee_rate: ::prost::alloc::string::String,
    /// relayer_fee_share_rate defines the relayer fee share rate for the
    /// derivative market
    #[prost(string, tag = "6")]
    pub relayer_fee_share_rate: ::prost::alloc::string::String,
    /// min_price_tick_size defines the minimum tick size of the order's price and
    /// margin
    #[prost(string, tag = "7")]
    pub min_price_tick_size: ::prost::alloc::string::String,
    /// min_quantity_tick_size defines the minimum tick size of the order's
    /// quantity
    #[prost(string, tag = "8")]
    pub min_quantity_tick_size: ::prost::alloc::string::String,
    /// expiration timestamp
    #[prost(int64, tag = "9")]
    pub expiration_timestamp: i64,
    /// expiration timestamp
    #[prost(int64, tag = "10")]
    pub settlement_timestamp: i64,
    /// new price at which market will be settled
    #[prost(string, tag = "11")]
    pub settlement_price: ::prost::alloc::string::String,
    /// admin of the market
    #[prost(string, tag = "12")]
    pub admin: ::prost::alloc::string::String,
    #[prost(enumeration = "MarketStatus", tag = "13")]
    pub status: i32,
    #[prost(message, optional, tag = "14")]
    pub oracle_params: ::core::option::Option<ProviderOracleParams>,
    #[prost(string, tag = "15")]
    pub ticker: ::prost::alloc::string::String,
    /// min_notional defines the minimum notional (in quote asset) required for
    /// orders in the market
    #[prost(string, tag = "16")]
    pub min_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for BinaryOptionsMarketParamUpdateProposal {
    const NAME: &'static str = "BinaryOptionsMarketParamUpdateProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderOracleParams {
    /// Oracle base currency
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "2")]
    pub provider: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "3")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "4")]
    pub oracle_type: i32,
}
impl ::prost::Name for ProviderOracleParams {
    const NAME: &'static str = "ProviderOracleParams";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleParams {
    /// Oracle base currency
    #[prost(string, tag = "1")]
    pub oracle_base: ::prost::alloc::string::String,
    /// Oracle quote currency
    #[prost(string, tag = "2")]
    pub oracle_quote: ::prost::alloc::string::String,
    /// Scale factor for oracle prices.
    #[prost(uint32, tag = "3")]
    pub oracle_scale_factor: u32,
    /// Oracle type
    #[prost(enumeration = "super::super::oracle::v1beta1::OracleType", tag = "4")]
    pub oracle_type: i32,
}
impl ::prost::Name for OracleParams {
    const NAME: &'static str = "OracleParams";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignLaunchProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag = "4")]
    pub campaign_reward_pools: ::prost::alloc::vec::Vec<CampaignRewardPool>,
}
impl ::prost::Name for TradingRewardCampaignLaunchProposal {
    const NAME: &'static str = "TradingRewardCampaignLaunchProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardCampaignUpdateProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag = "4")]
    pub campaign_reward_pools_additions: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    #[prost(message, repeated, tag = "5")]
    pub campaign_reward_pools_updates: ::prost::alloc::vec::Vec<CampaignRewardPool>,
}
impl ::prost::Name for TradingRewardCampaignUpdateProposal {
    const NAME: &'static str = "TradingRewardCampaignUpdateProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardPointUpdate {
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
    /// new_points overwrites the current trading reward points for the account
    #[prost(string, tag = "12")]
    pub new_points: ::prost::alloc::string::String,
}
impl ::prost::Name for RewardPointUpdate {
    const NAME: &'static str = "RewardPointUpdate";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingRewardPendingPointsUpdateProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub pending_pool_timestamp: i64,
    #[prost(message, repeated, tag = "4")]
    pub reward_point_updates: ::prost::alloc::vec::Vec<RewardPointUpdate>,
}
impl ::prost::Name for TradingRewardPendingPointsUpdateProposal {
    const NAME: &'static str = "TradingRewardPendingPointsUpdateProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeDiscountProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub schedule: ::core::option::Option<FeeDiscountSchedule>,
}
impl ::prost::Name for FeeDiscountProposal {
    const NAME: &'static str = "FeeDiscountProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub proposals: ::prost::alloc::vec::Vec<
        super::super::super::cosmos::distribution::v1beta1::CommunityPoolSpendProposal,
    >,
}
impl ::prost::Name for BatchCommunityPoolSpendProposal {
    const NAME: &'static str = "BatchCommunityPoolSpendProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// AtomicMarketOrderFeeMultiplierScheduleProposal defines a SDK message for
/// proposing new atomic take fee multipliers for specified markets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtomicMarketOrderFeeMultiplierScheduleProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub market_fee_multipliers: ::prost::alloc::vec::Vec<MarketFeeMultiplier>,
}
impl ::prost::Name for AtomicMarketOrderFeeMultiplierScheduleProposal {
    const NAME: &'static str = "AtomicMarketOrderFeeMultiplierScheduleProposal";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExchangeType {
    ExchangeUnspecified = 0,
    Spot = 1,
    Derivatives = 2,
}
impl ExchangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExchangeType::ExchangeUnspecified => "EXCHANGE_UNSPECIFIED",
            ExchangeType::Spot => "SPOT",
            ExchangeType::Derivatives => "DERIVATIVES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXCHANGE_UNSPECIFIED" => Some(Self::ExchangeUnspecified),
            "SPOT" => Some(Self::Spot),
            "DERIVATIVES" => Some(Self::Derivatives),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subaccount {
    #[prost(string, tag = "1")]
    pub trader: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub subaccount_nonce: u32,
}
impl ::prost::Name for Subaccount {
    const NAME: &'static str = "Subaccount";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrdersRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountOrdersRequest {
    const NAME: &'static str = "QuerySubaccountOrdersRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub buy_orders: ::prost::alloc::vec::Vec<SubaccountOrderData>,
    #[prost(message, repeated, tag = "2")]
    pub sell_orders: ::prost::alloc::vec::Vec<SubaccountOrderData>,
}
impl ::prost::Name for QuerySubaccountOrdersResponse {
    const NAME: &'static str = "QuerySubaccountOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderbookMetadataWithMarket {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<SubaccountOrderbookMetadata>,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_buy: bool,
}
impl ::prost::Name for SubaccountOrderbookMetadataWithMarket {
    const NAME: &'static str = "SubaccountOrderbookMetadataWithMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryExchangeParamsRequest is the request type for the Query/ExchangeParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeParamsRequest {}
impl ::prost::Name for QueryExchangeParamsRequest {
    const NAME: &'static str = "QueryExchangeParamsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryExchangeParamsRequest is the response type for the Query/ExchangeParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryExchangeParamsResponse {
    const NAME: &'static str = "QueryExchangeParamsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountDepositsRequest is the request type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositsRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub subaccount: ::core::option::Option<Subaccount>,
}
impl ::prost::Name for QuerySubaccountDepositsRequest {
    const NAME: &'static str = "QuerySubaccountDepositsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountDepositsResponse is the response type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositsResponse {
    #[prost(map = "string, message", tag = "1")]
    pub deposits: ::std::collections::HashMap<::prost::alloc::string::String, Deposit>,
}
impl ::prost::Name for QuerySubaccountDepositsResponse {
    const NAME: &'static str = "QuerySubaccountDepositsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryExchangeBalancesRequest is the request type for the
/// Query/ExchangeBalances RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeBalancesRequest {}
impl ::prost::Name for QueryExchangeBalancesRequest {
    const NAME: &'static str = "QueryExchangeBalancesRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountDepositsResponse is the response type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExchangeBalancesResponse {
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
impl ::prost::Name for QueryExchangeBalancesResponse {
    const NAME: &'static str = "QueryExchangeBalancesResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateVolumeRequest is the request type for the Query/AggregateVolume
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumeRequest {
    /// can either be an address or a subaccount
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAggregateVolumeRequest {
    const NAME: &'static str = "QueryAggregateVolumeRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateVolumeResponse is the response type for the
/// Query/AggregateVolume RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumeResponse {
    /// if an address is specified, then the aggregate_volumes will aggregate the
    /// volumes across all subaccounts for the address
    #[prost(message, repeated, tag = "1")]
    pub aggregate_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
impl ::prost::Name for QueryAggregateVolumeResponse {
    const NAME: &'static str = "QueryAggregateVolumeResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateVolumesRequest is the request type for the
/// Query/AggregateVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumesRequest {
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryAggregateVolumesRequest {
    const NAME: &'static str = "QueryAggregateVolumesRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateVolumesResponse is the response type for the
/// Query/AggregateVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateVolumesResponse {
    /// the aggregate volume records for the accounts specified
    #[prost(message, repeated, tag = "1")]
    pub aggregate_account_volumes: ::prost::alloc::vec::Vec<AggregateAccountVolumeRecord>,
    /// the aggregate volumes for the markets specified
    #[prost(message, repeated, tag = "2")]
    pub aggregate_market_volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
impl ::prost::Name for QueryAggregateVolumesResponse {
    const NAME: &'static str = "QueryAggregateVolumesResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateMarketVolumeRequest is the request type for the
/// Query/AggregateMarketVolume RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumeRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAggregateMarketVolumeRequest {
    const NAME: &'static str = "QueryAggregateMarketVolumeRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateMarketVolumeResponse is the response type for the
/// Query/AggregateMarketVolume RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumeResponse {
    #[prost(message, optional, tag = "1")]
    pub volume: ::core::option::Option<VolumeRecord>,
}
impl ::prost::Name for QueryAggregateMarketVolumeResponse {
    const NAME: &'static str = "QueryAggregateMarketVolumeResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDenomDecimalRequest is the request type for the Query/DenomDecimal RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDenomDecimalRequest {
    const NAME: &'static str = "QueryDenomDecimalRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDenomDecimalResponse is the response type for the Query/DenomDecimal RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalResponse {
    #[prost(uint64, tag = "1")]
    pub decimal: u64,
}
impl ::prost::Name for QueryDenomDecimalResponse {
    const NAME: &'static str = "QueryDenomDecimalResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDenomDecimalsRequest is the request type for the Query/DenomDecimals RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalsRequest {
    /// denoms can be empty to query all denom decimals
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryDenomDecimalsRequest {
    const NAME: &'static str = "QueryDenomDecimalsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDenomDecimalsRequest is the response type for the Query/DenomDecimals
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomDecimalsResponse {
    #[prost(message, repeated, tag = "1")]
    pub denom_decimals: ::prost::alloc::vec::Vec<DenomDecimals>,
}
impl ::prost::Name for QueryDenomDecimalsResponse {
    const NAME: &'static str = "QueryDenomDecimalsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateMarketVolumesRequest is the request type for the
/// Query/AggregateMarketVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumesRequest {
    #[prost(string, repeated, tag = "1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryAggregateMarketVolumesRequest {
    const NAME: &'static str = "QueryAggregateMarketVolumesRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAggregateMarketVolumesResponse is the response type for the
/// Query/AggregateMarketVolumes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAggregateMarketVolumesResponse {
    /// the aggregate volumes for the entire market
    #[prost(message, repeated, tag = "1")]
    pub volumes: ::prost::alloc::vec::Vec<MarketVolume>,
}
impl ::prost::Name for QueryAggregateMarketVolumesResponse {
    const NAME: &'static str = "QueryAggregateMarketVolumesResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountDepositsRequest is the request type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountDepositRequest {
    const NAME: &'static str = "QuerySubaccountDepositRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountDepositsResponse is the response type for the
/// Query/SubaccountDeposits RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountDepositResponse {
    #[prost(message, optional, tag = "1")]
    pub deposits: ::core::option::Option<Deposit>,
}
impl ::prost::Name for QuerySubaccountDepositResponse {
    const NAME: &'static str = "QuerySubaccountDepositResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMarketsRequest is the request type for the Query/SpotMarkets RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// Filter by market IDs
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QuerySpotMarketsRequest {
    const NAME: &'static str = "QuerySpotMarketsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMarketsResponse is the response type for the Query/SpotMarkets RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<SpotMarket>,
}
impl ::prost::Name for QuerySpotMarketsResponse {
    const NAME: &'static str = "QuerySpotMarketsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMarketRequest is the request type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySpotMarketRequest {
    const NAME: &'static str = "QuerySpotMarketRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMarketResponse is the response type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMarketResponse {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<SpotMarket>,
}
impl ::prost::Name for QuerySpotMarketResponse {
    const NAME: &'static str = "QuerySpotMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotOrderbookRequest is the request type for the Query/SpotOrderbook RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrderbookRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub limit: u64,
    #[prost(enumeration = "OrderSide", tag = "3")]
    pub order_side: i32,
    #[prost(string, tag = "4")]
    pub limit_cumulative_notional: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub limit_cumulative_quantity: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySpotOrderbookRequest {
    const NAME: &'static str = "QuerySpotOrderbookRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotOrderbookResponse is the response type for the Query/SpotOrderbook
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrderbookResponse {
    #[prost(message, repeated, tag = "1")]
    pub buys_price_level: ::prost::alloc::vec::Vec<Level>,
    #[prost(message, repeated, tag = "2")]
    pub sells_price_level: ::prost::alloc::vec::Vec<Level>,
}
impl ::prost::Name for QuerySpotOrderbookResponse {
    const NAME: &'static str = "QuerySpotOrderbookResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullSpotMarket {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<SpotMarket>,
    /// mid_price_and_tob defines the mid price for this market and the best ask
    /// and bid orders
    #[prost(message, optional, tag = "2")]
    pub mid_price_and_tob: ::core::option::Option<MidPriceAndTob>,
}
impl ::prost::Name for FullSpotMarket {
    const NAME: &'static str = "FullSpotMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFullSpotMarketsRequest is the request type for the Query/FullSpotMarkets
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// Filter by market IDs
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Flag to return the markets mid price and top of the book buy and sell
    /// orders.
    #[prost(bool, tag = "3")]
    pub with_mid_price_and_tob: bool,
}
impl ::prost::Name for QueryFullSpotMarketsRequest {
    const NAME: &'static str = "QueryFullSpotMarketsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFullSpotMarketsResponse is the response type for the
/// Query/FullSpotMarkets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<FullSpotMarket>,
}
impl ::prost::Name for QueryFullSpotMarketsResponse {
    const NAME: &'static str = "QueryFullSpotMarketsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMarketRequest is the request type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// Flag to return the markets mid price and top of the book buy and sell
    /// orders.
    #[prost(bool, tag = "2")]
    pub with_mid_price_and_tob: bool,
}
impl ::prost::Name for QueryFullSpotMarketRequest {
    const NAME: &'static str = "QueryFullSpotMarketRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMarketResponse is the response type for the Query/SpotMarket RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFullSpotMarketResponse {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<FullSpotMarket>,
}
impl ::prost::Name for QueryFullSpotMarketResponse {
    const NAME: &'static str = "QueryFullSpotMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotOrdersByHashesRequest is the request type for the
/// Query/SpotOrdersByHashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrdersByHashesRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the order hashes
    #[prost(string, repeated, tag = "3")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QuerySpotOrdersByHashesRequest {
    const NAME: &'static str = "QuerySpotOrdersByHashesRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotOrdersByHashesResponse is the response type for the
/// Query/SpotOrdersByHashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotOrdersByHashesResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedSpotLimitOrder>,
}
impl ::prost::Name for QuerySpotOrdersByHashesResponse {
    const NAME: &'static str = "QuerySpotOrdersByHashesResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderSpotOrdersRequest is the request type for the
/// Query/TraderSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderSpotOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTraderSpotOrdersRequest {
    const NAME: &'static str = "QueryTraderSpotOrdersRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAccountAddressSpotOrdersRequest is the request type for the
/// Query/AccountAddressSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressSpotOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// Account address of the trader
    #[prost(string, tag = "2")]
    pub account_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAccountAddressSpotOrdersRequest {
    const NAME: &'static str = "QueryAccountAddressSpotOrdersRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimmedSpotLimitOrder {
    /// price of the order
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag = "3")]
    pub fillable: ::prost::alloc::string::String,
    /// true if the order is a buy
    #[prost(bool, tag = "4")]
    pub is_buy: bool,
    #[prost(string, tag = "5")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for TrimmedSpotLimitOrder {
    const NAME: &'static str = "TrimmedSpotLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderSpotOrdersResponse is the response type for the
/// Query/TraderSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderSpotOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedSpotLimitOrder>,
}
impl ::prost::Name for QueryTraderSpotOrdersResponse {
    const NAME: &'static str = "QueryTraderSpotOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAccountAddressSpotOrdersResponse is the response type for the
/// Query/AccountAddressSpotOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressSpotOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedSpotLimitOrder>,
}
impl ::prost::Name for QueryAccountAddressSpotOrdersResponse {
    const NAME: &'static str = "QueryAccountAddressSpotOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMidPriceAndTOBRequest is the request type for the
/// Query/SpotMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMidPriceAndTobRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySpotMidPriceAndTobRequest {
    const NAME: &'static str = "QuerySpotMidPriceAndTOBRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySpotMidPriceAndTOBResponse is the response type for the
/// Query/SpotMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySpotMidPriceAndTobResponse {
    /// mid price of the market
    #[prost(string, tag = "1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag = "2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag = "3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySpotMidPriceAndTobResponse {
    const NAME: &'static str = "QuerySpotMidPriceAndTOBResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMidPriceAndTOBRequest is the request type for the
/// Query/GetDerivativeMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMidPriceAndTobRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDerivativeMidPriceAndTobRequest {
    const NAME: &'static str = "QueryDerivativeMidPriceAndTOBRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMidPriceAndTOBResponse is the response type for the
/// Query/GetDerivativeMidPriceAndTOB RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMidPriceAndTobResponse {
    /// mid price of the market
    #[prost(string, tag = "1")]
    pub mid_price: ::prost::alloc::string::String,
    /// best buy price of the market
    #[prost(string, tag = "2")]
    pub best_buy_price: ::prost::alloc::string::String,
    /// best sell price of the market
    #[prost(string, tag = "3")]
    pub best_sell_price: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDerivativeMidPriceAndTobResponse {
    const NAME: &'static str = "QueryDerivativeMidPriceAndTOBResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeOrderbookRequest is the request type for the
/// Query/DerivativeOrderbook RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrderbookRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub limit: u64,
    #[prost(string, tag = "3")]
    pub limit_cumulative_notional: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDerivativeOrderbookRequest {
    const NAME: &'static str = "QueryDerivativeOrderbookRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeOrderbookResponse is the response type for the
/// Query/DerivativeOrderbook RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrderbookResponse {
    #[prost(message, repeated, tag = "1")]
    pub buys_price_level: ::prost::alloc::vec::Vec<Level>,
    #[prost(message, repeated, tag = "2")]
    pub sells_price_level: ::prost::alloc::vec::Vec<Level>,
}
impl ::prost::Name for QueryDerivativeOrderbookResponse {
    const NAME: &'static str = "QueryDerivativeOrderbookResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderSpotOrdersToCancelUpToAmountRequest is the request type for the
/// Query/TraderSpotOrdersToCancelUpToAmountRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderSpotOrdersToCancelUpToAmountRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the base amount to cancel (free up)
    #[prost(string, tag = "3")]
    pub base_amount: ::prost::alloc::string::String,
    /// the quote amount to cancel (free up)
    #[prost(string, tag = "4")]
    pub quote_amount: ::prost::alloc::string::String,
    /// The cancellation strategy
    #[prost(enumeration = "CancellationStrategy", tag = "5")]
    pub strategy: i32,
    /// The reference price for the cancellation strategy, e.g. mid price or mark
    /// price
    #[prost(string, tag = "6")]
    pub reference_price: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTraderSpotOrdersToCancelUpToAmountRequest {
    const NAME: &'static str = "QueryTraderSpotOrdersToCancelUpToAmountRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderDerivativeOrdersToCancelUpToAmountRequest is the request type for
/// the Query/TraderDerivativeOrdersToCancelUpToAmountRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeOrdersToCancelUpToAmountRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the quote amount to cancel (free up)
    #[prost(string, tag = "3")]
    pub quote_amount: ::prost::alloc::string::String,
    /// The cancellation strategy
    #[prost(enumeration = "CancellationStrategy", tag = "4")]
    pub strategy: i32,
    /// The reference price for the cancellation strategy, e.g. mid price or mark
    /// price
    #[prost(string, tag = "5")]
    pub reference_price: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTraderDerivativeOrdersToCancelUpToAmountRequest {
    const NAME: &'static str = "QueryTraderDerivativeOrdersToCancelUpToAmountRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderDerivativeOrdersRequest is the request type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTraderDerivativeOrdersRequest {
    const NAME: &'static str = "QueryTraderDerivativeOrdersRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAccountAddressSpotOrdersRequest is the request type for the
/// Query/AccountAddressDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressDerivativeOrdersRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// Account address of the trader
    #[prost(string, tag = "2")]
    pub account_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAccountAddressDerivativeOrdersRequest {
    const NAME: &'static str = "QueryAccountAddressDerivativeOrdersRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimmedDerivativeLimitOrder {
    /// price of the order
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
    /// margin of the order
    #[prost(string, tag = "3")]
    pub margin: ::prost::alloc::string::String,
    /// the amount of the quantity remaining fillable
    #[prost(string, tag = "4")]
    pub fillable: ::prost::alloc::string::String,
    /// true if the order is a buy
    ///
    /// ensure omitempty is not in jsontag
    #[prost(bool, tag = "5")]
    pub is_buy: bool,
    #[prost(string, tag = "6")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for TrimmedDerivativeLimitOrder {
    const NAME: &'static str = "TrimmedDerivativeLimitOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderDerivativeOrdersResponse is the response type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeLimitOrder>,
}
impl ::prost::Name for QueryTraderDerivativeOrdersResponse {
    const NAME: &'static str = "QueryTraderDerivativeOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryAccountAddressDerivativeOrdersResponse is the response type for the
/// Query/AccountAddressDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressDerivativeOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeLimitOrder>,
}
impl ::prost::Name for QueryAccountAddressDerivativeOrdersResponse {
    const NAME: &'static str = "QueryAccountAddressDerivativeOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderDerivativeOrdersRequest is the request type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrdersByHashesRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    /// SubaccountID of the trader
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// the order hashes
    #[prost(string, repeated, tag = "3")]
    pub order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryDerivativeOrdersByHashesRequest {
    const NAME: &'static str = "QueryDerivativeOrdersByHashesRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeOrdersByHashesResponse is the response type for the
/// Query/DerivativeOrdersByHashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeOrdersByHashesResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeLimitOrder>,
}
impl ::prost::Name for QueryDerivativeOrdersByHashesResponse {
    const NAME: &'static str = "QueryDerivativeOrdersByHashesResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMarketsRequest is the request type for the
/// Query/DerivativeMarkets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// Filter by market IDs
    #[prost(string, repeated, tag = "2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Flag to return the markets mid price and top of the book buy and sell
    /// orders.
    #[prost(bool, tag = "3")]
    pub with_mid_price_and_tob: bool,
}
impl ::prost::Name for QueryDerivativeMarketsRequest {
    const NAME: &'static str = "QueryDerivativeMarketsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceLevel {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    /// quantity
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
}
impl ::prost::Name for PriceLevel {
    const NAME: &'static str = "PriceLevel";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualMarketState {
    #[prost(message, optional, tag = "1")]
    pub market_info: ::core::option::Option<PerpetualMarketInfo>,
    #[prost(message, optional, tag = "2")]
    pub funding_info: ::core::option::Option<PerpetualMarketFunding>,
}
impl ::prost::Name for PerpetualMarketState {
    const NAME: &'static str = "PerpetualMarketState";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullDerivativeMarket {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<DerivativeMarket>,
    #[prost(string, tag = "4")]
    pub mark_price: ::prost::alloc::string::String,
    /// mid_price_and_tob defines the mid price for this market and the best ask
    /// and bid orders
    #[prost(message, optional, tag = "5")]
    pub mid_price_and_tob: ::core::option::Option<MidPriceAndTob>,
    #[prost(oneof = "full_derivative_market::Info", tags = "2, 3")]
    pub info: ::core::option::Option<full_derivative_market::Info>,
}
/// Nested message and enum types in `FullDerivativeMarket`.
pub mod full_derivative_market {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Info {
        #[prost(message, tag = "2")]
        PerpetualInfo(super::PerpetualMarketState),
        #[prost(message, tag = "3")]
        FuturesInfo(super::ExpiryFuturesMarketInfo),
    }
}
impl ::prost::Name for FullDerivativeMarket {
    const NAME: &'static str = "FullDerivativeMarket";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMarketsResponse is the response type for the
/// Query/DerivativeMarkets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<FullDerivativeMarket>,
}
impl ::prost::Name for QueryDerivativeMarketsResponse {
    const NAME: &'static str = "QueryDerivativeMarketsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMarketRequest is the request type for the
/// Query/DerivativeMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDerivativeMarketRequest {
    const NAME: &'static str = "QueryDerivativeMarketRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMarketResponse is the response type for the
/// Query/DerivativeMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketResponse {
    #[prost(message, optional, tag = "1")]
    pub market: ::core::option::Option<FullDerivativeMarket>,
}
impl ::prost::Name for QueryDerivativeMarketResponse {
    const NAME: &'static str = "QueryDerivativeMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMarketAddressRequest is the request type for the
/// Query/DerivativeMarketAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketAddressRequest {
    /// Market ID for the market
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDerivativeMarketAddressRequest {
    const NAME: &'static str = "QueryDerivativeMarketAddressRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryDerivativeMarketAddressResponse is the response type for the
/// Query/DerivativeMarketAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDerivativeMarketAddressResponse {
    /// address for the market
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// subaccountID for the market
    #[prost(string, tag = "2")]
    pub subaccount_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDerivativeMarketAddressResponse {
    const NAME: &'static str = "QueryDerivativeMarketAddressResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountTradeNonceRequest is the request type for the
/// Query/SubaccountTradeNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountTradeNonceRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountTradeNonceRequest {
    const NAME: &'static str = "QuerySubaccountTradeNonceRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountPositionsRequest is the request type for the
/// Query/SubaccountPositions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionsRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountPositionsRequest {
    const NAME: &'static str = "QuerySubaccountPositionsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountPositionInMarketRequest is the request type for the
/// Query/SubaccountPositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionInMarketRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountPositionInMarketRequest {
    const NAME: &'static str = "QuerySubaccountPositionInMarketRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountEffectivePositionInMarketRequest is the request type for the
/// Query/SubaccountEffectivePositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountEffectivePositionInMarketRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountEffectivePositionInMarketRequest {
    const NAME: &'static str = "QuerySubaccountEffectivePositionInMarketRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountOrderMetadataRequest is the request type for the
/// Query/SubaccountOrderMetadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrderMetadataRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySubaccountOrderMetadataRequest {
    const NAME: &'static str = "QuerySubaccountOrderMetadataRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountPositionsResponse is the response type for the
/// Query/SubaccountPositions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub state: ::prost::alloc::vec::Vec<DerivativePosition>,
}
impl ::prost::Name for QuerySubaccountPositionsResponse {
    const NAME: &'static str = "QuerySubaccountPositionsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountPositionInMarketResponse is the response type for the
/// Query/SubaccountPositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountPositionInMarketResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<Position>,
}
impl ::prost::Name for QuerySubaccountPositionInMarketResponse {
    const NAME: &'static str = "QuerySubaccountPositionInMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectivePosition {
    #[prost(bool, tag = "1")]
    pub is_long: bool,
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub entry_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub effective_margin: ::prost::alloc::string::String,
}
impl ::prost::Name for EffectivePosition {
    const NAME: &'static str = "EffectivePosition";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountEffectivePositionInMarketResponse is the response type for the
/// Query/SubaccountEffectivePositionInMarket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountEffectivePositionInMarketResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<EffectivePosition>,
}
impl ::prost::Name for QuerySubaccountEffectivePositionInMarketResponse {
    const NAME: &'static str = "QuerySubaccountEffectivePositionInMarketResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryPerpetualMarketInfoRequest is the request type for the
/// Query/PerpetualMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketInfoRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPerpetualMarketInfoRequest {
    const NAME: &'static str = "QueryPerpetualMarketInfoRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryPerpetualMarketInfoResponse is the response type for the
/// Query/PerpetualMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<PerpetualMarketInfo>,
}
impl ::prost::Name for QueryPerpetualMarketInfoResponse {
    const NAME: &'static str = "QueryPerpetualMarketInfoResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryExpiryFuturesMarketInfoRequest is the request type for the Query/
/// ExpiryFuturesMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExpiryFuturesMarketInfoRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryExpiryFuturesMarketInfoRequest {
    const NAME: &'static str = "QueryExpiryFuturesMarketInfoRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryExpiryFuturesMarketInfoResponse is the response type for the Query/
/// ExpiryFuturesMarketInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryExpiryFuturesMarketInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<ExpiryFuturesMarketInfo>,
}
impl ::prost::Name for QueryExpiryFuturesMarketInfoResponse {
    const NAME: &'static str = "QueryExpiryFuturesMarketInfoResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryPerpetualMarketFundingRequest is the request type for the
/// Query/PerpetualMarketFunding RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketFundingRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPerpetualMarketFundingRequest {
    const NAME: &'static str = "QueryPerpetualMarketFundingRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryPerpetualMarketFundingResponse is the response type for the
/// Query/PerpetualMarketFunding RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPerpetualMarketFundingResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<PerpetualMarketFunding>,
}
impl ::prost::Name for QueryPerpetualMarketFundingResponse {
    const NAME: &'static str = "QueryPerpetualMarketFundingResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountOrderMetadataResponse is the response type for the
/// Query/SubaccountOrderMetadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountOrderMetadataResponse {
    #[prost(message, repeated, tag = "1")]
    pub metadata: ::prost::alloc::vec::Vec<SubaccountOrderbookMetadataWithMarket>,
}
impl ::prost::Name for QuerySubaccountOrderMetadataResponse {
    const NAME: &'static str = "QuerySubaccountOrderMetadataResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerySubaccountTradeNonceResponse is the response type for the
/// Query/SubaccountTradeNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySubaccountTradeNonceResponse {
    #[prost(uint32, tag = "1")]
    pub nonce: u32,
}
impl ::prost::Name for QuerySubaccountTradeNonceResponse {
    const NAME: &'static str = "QuerySubaccountTradeNonceResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateRequest is the request type for the Query/ExchangeModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {}
impl ::prost::Name for QueryModuleStateRequest {
    const NAME: &'static str = "QueryModuleStateRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateResponse is the response type for the
/// Query/ExchangeModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
impl ::prost::Name for QueryModuleStateResponse {
    const NAME: &'static str = "QueryModuleStateResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryPositionsRequest is the request type for the Query/Positions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPositionsRequest {}
impl ::prost::Name for QueryPositionsRequest {
    const NAME: &'static str = "QueryPositionsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryPositionsResponse is the response type for the Query/Positions RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPositionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub state: ::prost::alloc::vec::Vec<DerivativePosition>,
}
impl ::prost::Name for QueryPositionsResponse {
    const NAME: &'static str = "QueryPositionsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTradeRewardPointsRequest is the request type for the
/// Query/TradeRewardPoints RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardPointsRequest {
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "2")]
    pub pending_pool_timestamp: i64,
}
impl ::prost::Name for QueryTradeRewardPointsRequest {
    const NAME: &'static str = "QueryTradeRewardPointsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTradeRewardPointsResponse is the response type for the
/// Query/TradeRewardPoints RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardPointsResponse {
    #[prost(string, repeated, tag = "1")]
    pub account_trade_reward_points: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryTradeRewardPointsResponse {
    const NAME: &'static str = "QueryTradeRewardPointsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTradeRewardCampaignRequest is the request type for the
/// Query/TradeRewardCampaign RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardCampaignRequest {}
impl ::prost::Name for QueryTradeRewardCampaignRequest {
    const NAME: &'static str = "QueryTradeRewardCampaignRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTradeRewardCampaignResponse is the response type for the
/// Query/TradeRewardCampaign RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTradeRewardCampaignResponse {
    #[prost(message, optional, tag = "1")]
    pub trading_reward_campaign_info: ::core::option::Option<TradingRewardCampaignInfo>,
    #[prost(message, repeated, tag = "2")]
    pub trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    #[prost(string, tag = "3")]
    pub total_trade_reward_points: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub pending_trading_reward_pool_campaign_schedule: ::prost::alloc::vec::Vec<CampaignRewardPool>,
    #[prost(string, repeated, tag = "5")]
    pub pending_total_trade_reward_points: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryTradeRewardCampaignResponse {
    const NAME: &'static str = "QueryTradeRewardCampaignResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryIsRegisteredDMMRequest is the request type for the Query/IsRegisteredDMM
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsOptedOutOfRewardsRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryIsOptedOutOfRewardsRequest {
    const NAME: &'static str = "QueryIsOptedOutOfRewardsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryIsRegisteredDMMResponse is the response type for the
/// Query/IsRegisteredDMM RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsOptedOutOfRewardsResponse {
    #[prost(bool, tag = "1")]
    pub is_opted_out: bool,
}
impl ::prost::Name for QueryIsOptedOutOfRewardsResponse {
    const NAME: &'static str = "QueryIsOptedOutOfRewardsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryRegisteredDMMsRequest is the request type for the Query/RegisteredDMMs
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOptedOutOfRewardsAccountsRequest {}
impl ::prost::Name for QueryOptedOutOfRewardsAccountsRequest {
    const NAME: &'static str = "QueryOptedOutOfRewardsAccountsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryRegisteredDMMsResponse is the response type for the Query/RegisteredDMMs
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOptedOutOfRewardsAccountsResponse {
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryOptedOutOfRewardsAccountsResponse {
    const NAME: &'static str = "QueryOptedOutOfRewardsAccountsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFeeDiscountAccountInfoRequest is the request type for the
/// Query/FeeDiscountAccountInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountAccountInfoRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryFeeDiscountAccountInfoRequest {
    const NAME: &'static str = "QueryFeeDiscountAccountInfoRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFeeDiscountAccountInfoResponse is the response type for the
/// Query/FeeDiscountAccountInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountAccountInfoResponse {
    #[prost(uint64, tag = "1")]
    pub tier_level: u64,
    #[prost(message, optional, tag = "2")]
    pub account_info: ::core::option::Option<FeeDiscountTierInfo>,
    #[prost(message, optional, tag = "3")]
    pub account_ttl: ::core::option::Option<FeeDiscountTierTtl>,
}
impl ::prost::Name for QueryFeeDiscountAccountInfoResponse {
    const NAME: &'static str = "QueryFeeDiscountAccountInfoResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFeeDiscountScheduleRequest is the request type for the
/// Query/FeeDiscountSchedule RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountScheduleRequest {}
impl ::prost::Name for QueryFeeDiscountScheduleRequest {
    const NAME: &'static str = "QueryFeeDiscountScheduleRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFeeDiscountScheduleResponse is the response type for the
/// Query/FeeDiscountSchedule RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountScheduleResponse {
    #[prost(message, optional, tag = "1")]
    pub fee_discount_schedule: ::core::option::Option<FeeDiscountSchedule>,
}
impl ::prost::Name for QueryFeeDiscountScheduleResponse {
    const NAME: &'static str = "QueryFeeDiscountScheduleResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryBalanceMismatchesRequest is the request type for the
/// Query/QueryBalanceMismatches RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceMismatchesRequest {
    #[prost(int64, tag = "1")]
    pub dust_factor: i64,
}
impl ::prost::Name for QueryBalanceMismatchesRequest {
    const NAME: &'static str = "QueryBalanceMismatchesRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceMismatch {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub available: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub balance_hold: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub expected_total: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub difference: ::prost::alloc::string::String,
}
impl ::prost::Name for BalanceMismatch {
    const NAME: &'static str = "BalanceMismatch";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryBalanceMismatchesResponse is the response type for the
/// Query/QueryBalanceMismatches RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceMismatchesResponse {
    #[prost(message, repeated, tag = "1")]
    pub balance_mismatches: ::prost::alloc::vec::Vec<BalanceMismatch>,
}
impl ::prost::Name for QueryBalanceMismatchesResponse {
    const NAME: &'static str = "QueryBalanceMismatchesResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryBalanceWithBalanceHoldsRequest is the request type for the
/// Query/QueryBalanceWithBalanceHolds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceWithBalanceHoldsRequest {}
impl ::prost::Name for QueryBalanceWithBalanceHoldsRequest {
    const NAME: &'static str = "QueryBalanceWithBalanceHoldsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceWithMarginHold {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub available: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub balance_hold: ::prost::alloc::string::String,
}
impl ::prost::Name for BalanceWithMarginHold {
    const NAME: &'static str = "BalanceWithMarginHold";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryBalanceWithBalanceHoldsResponse is the response type for the
/// Query/QueryBalanceWithBalanceHolds RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceWithBalanceHoldsResponse {
    #[prost(message, repeated, tag = "1")]
    pub balance_with_balance_holds: ::prost::alloc::vec::Vec<BalanceWithMarginHold>,
}
impl ::prost::Name for QueryBalanceWithBalanceHoldsResponse {
    const NAME: &'static str = "QueryBalanceWithBalanceHoldsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFeeDiscountTierStatisticsRequest is the request type for the
/// Query/QueryFeeDiscountTierStatistics RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountTierStatisticsRequest {}
impl ::prost::Name for QueryFeeDiscountTierStatisticsRequest {
    const NAME: &'static str = "QueryFeeDiscountTierStatisticsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TierStatistic {
    #[prost(uint64, tag = "1")]
    pub tier: u64,
    #[prost(uint64, tag = "2")]
    pub count: u64,
}
impl ::prost::Name for TierStatistic {
    const NAME: &'static str = "TierStatistic";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryFeeDiscountTierStatisticsResponse is the response type for the
/// Query/QueryFeeDiscountTierStatistics RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeDiscountTierStatisticsResponse {
    #[prost(message, repeated, tag = "1")]
    pub statistics: ::prost::alloc::vec::Vec<TierStatistic>,
}
impl ::prost::Name for QueryFeeDiscountTierStatisticsResponse {
    const NAME: &'static str = "QueryFeeDiscountTierStatisticsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MitoVaultInfosRequest is the request type for the Query/MitoVaultInfos RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MitoVaultInfosRequest {}
impl ::prost::Name for MitoVaultInfosRequest {
    const NAME: &'static str = "MitoVaultInfosRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// MitoVaultInfosResponse is the response type for the Query/MitoVaultInfos RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MitoVaultInfosResponse {
    #[prost(string, repeated, tag = "1")]
    pub master_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub derivative_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub spot_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub cw20_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MitoVaultInfosResponse {
    const NAME: &'static str = "MitoVaultInfosResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryMarketIDFromVaultRequest is the request type for the
/// Query/QueryMarketIDFromVault RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketIdFromVaultRequest {
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMarketIdFromVaultRequest {
    const NAME: &'static str = "QueryMarketIDFromVaultRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryMarketIDFromVaultResponse is the response type for the
/// Query/QueryMarketIDFromVault RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketIdFromVaultResponse {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMarketIdFromVaultResponse {
    const NAME: &'static str = "QueryMarketIDFromVaultResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalTradeRecordsRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryHistoricalTradeRecordsRequest {
    const NAME: &'static str = "QueryHistoricalTradeRecordsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalTradeRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub trade_records: ::prost::alloc::vec::Vec<TradeRecords>,
}
impl ::prost::Name for QueryHistoricalTradeRecordsResponse {
    const NAME: &'static str = "QueryHistoricalTradeRecordsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// TradeHistoryOptions are the optional params for Query/MarketVolatility RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeHistoryOptions {
    /// TradeGroupingSec of 0 means use the chain's default grouping
    #[prost(uint64, tag = "1")]
    pub trade_grouping_sec: u64,
    /// MaxAge restricts the trade records oldest age in seconds from the current
    /// block time to consider. A value of 0 means use all the records present on
    /// the chain.
    #[prost(uint64, tag = "2")]
    pub max_age: u64,
    /// If IncludeRawHistory is true, the raw underlying data used for the
    /// computation is included in the response
    #[prost(bool, tag = "4")]
    pub include_raw_history: bool,
    /// If IncludeMetadata is true, metadata on the computation is included in the
    /// response
    #[prost(bool, tag = "5")]
    pub include_metadata: bool,
}
impl ::prost::Name for TradeHistoryOptions {
    const NAME: &'static str = "TradeHistoryOptions";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryMarketVolatilityRequest are the request params for the
/// Query/MarketVolatility RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketVolatilityRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub trade_history_options: ::core::option::Option<TradeHistoryOptions>,
}
impl ::prost::Name for QueryMarketVolatilityRequest {
    const NAME: &'static str = "QueryMarketVolatilityRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryMarketVolatilityResponse is the response type for the
/// Query/MarketVolatility RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketVolatilityResponse {
    #[prost(string, tag = "1")]
    pub volatility: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub history_metadata: ::core::option::Option<super::super::oracle::v1beta1::MetadataStatistics>,
    #[prost(message, repeated, tag = "3")]
    pub raw_history: ::prost::alloc::vec::Vec<TradeRecord>,
}
impl ::prost::Name for QueryMarketVolatilityResponse {
    const NAME: &'static str = "QueryMarketVolatilityResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QuerBinaryMarketsRequest is the request type for the Query/BinaryMarkets RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBinaryMarketsRequest {
    /// Status of the market, for convenience it is set to string - not enum
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBinaryMarketsRequest {
    const NAME: &'static str = "QueryBinaryMarketsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryBinaryMarketsResponse is the response type for the Query/BinaryMarkets
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBinaryMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<BinaryOptionsMarket>,
}
impl ::prost::Name for QueryBinaryMarketsResponse {
    const NAME: &'static str = "QueryBinaryMarketsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryConditionalOrdersRequest is the request type for the
/// Query/ConditionalOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeConditionalOrdersRequest {
    #[prost(string, tag = "1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTraderDerivativeConditionalOrdersRequest {
    const NAME: &'static str = "QueryTraderDerivativeConditionalOrdersRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrimmedDerivativeConditionalOrder {
    /// price of the order
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    /// quantity of the order
    #[prost(string, tag = "2")]
    pub quantity: ::prost::alloc::string::String,
    /// margin of the order
    #[prost(string, tag = "3")]
    pub margin: ::prost::alloc::string::String,
    /// price to trigger the order
    #[prost(string, tag = "4")]
    pub trigger_price: ::prost::alloc::string::String,
    /// true if the order is a buy
    ///
    /// ensure omitempty is not in jsontag
    #[prost(bool, tag = "5")]
    pub is_buy: bool,
    #[prost(bool, tag = "6")]
    pub is_limit: bool,
    #[prost(string, tag = "7")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub cid: ::prost::alloc::string::String,
}
impl ::prost::Name for TrimmedDerivativeConditionalOrder {
    const NAME: &'static str = "TrimmedDerivativeConditionalOrder";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
/// QueryTraderDerivativeOrdersResponse is the response type for the
/// Query/TraderDerivativeOrders RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraderDerivativeConditionalOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<TrimmedDerivativeConditionalOrder>,
}
impl ::prost::Name for QueryTraderDerivativeConditionalOrdersResponse {
    const NAME: &'static str = "QueryTraderDerivativeConditionalOrdersResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketAtomicExecutionFeeMultiplierRequest {
    #[prost(string, tag = "1")]
    pub market_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMarketAtomicExecutionFeeMultiplierRequest {
    const NAME: &'static str = "QueryMarketAtomicExecutionFeeMultiplierRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarketAtomicExecutionFeeMultiplierResponse {
    #[prost(string, tag = "1")]
    pub multiplier: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMarketAtomicExecutionFeeMultiplierResponse {
    const NAME: &'static str = "QueryMarketAtomicExecutionFeeMultiplierResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveStakeGrantRequest {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryActiveStakeGrantRequest {
    const NAME: &'static str = "QueryActiveStakeGrantRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActiveStakeGrantResponse {
    #[prost(message, optional, tag = "1")]
    pub grant: ::core::option::Option<ActiveGrant>,
    #[prost(message, optional, tag = "2")]
    pub effective_grant: ::core::option::Option<EffectiveGrant>,
}
impl ::prost::Name for QueryActiveStakeGrantResponse {
    const NAME: &'static str = "QueryActiveStakeGrantResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantAuthorizationRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryGrantAuthorizationRequest {
    const NAME: &'static str = "QueryGrantAuthorizationRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantAuthorizationResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryGrantAuthorizationResponse {
    const NAME: &'static str = "QueryGrantAuthorizationResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantAuthorizationsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryGrantAuthorizationsRequest {
    const NAME: &'static str = "QueryGrantAuthorizationsRequest";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantAuthorizationsResponse {
    #[prost(string, tag = "1")]
    pub total_grant_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
impl ::prost::Name for QueryGrantAuthorizationsResponse {
    const NAME: &'static str = "QueryGrantAuthorizationsResponse";
    const PACKAGE: &'static str = "injective.exchange.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.exchange.v1beta1.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    /// will return both
    SideUnspecified = 0,
    Buy = 1,
    Sell = 2,
}
impl OrderSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderSide::SideUnspecified => "Side_Unspecified",
            OrderSide::Buy => "Buy",
            OrderSide::Sell => "Sell",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Side_Unspecified" => Some(Self::SideUnspecified),
            "Buy" => Some(Self::Buy),
            "Sell" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// CancellationStrategy is the list of cancellation strategies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CancellationStrategy {
    /// just cancelling in random order in most efficient way
    UnspecifiedOrder = 0,
    /// e.g. for buy orders from lowest to highest price
    FromWorstToBest = 1,
    /// e.g. for buy orders from higest to lowest price
    FromBestToWorst = 2,
}
impl CancellationStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CancellationStrategy::UnspecifiedOrder => "UnspecifiedOrder",
            CancellationStrategy::FromWorstToBest => "FromWorstToBest",
            CancellationStrategy::FromBestToWorst => "FromBestToWorst",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnspecifiedOrder" => Some(Self::UnspecifiedOrder),
            "FromWorstToBest" => Some(Self::FromWorstToBest),
            "FromBestToWorst" => Some(Self::FromBestToWorst),
            _ => None,
        }
    }
}
include!("injective.exchange.v1beta1.serde.rs");
include!("injective.exchange.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
