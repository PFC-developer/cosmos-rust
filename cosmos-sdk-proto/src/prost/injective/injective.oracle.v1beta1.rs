// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub pyth_contract: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleInfo {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration = "OracleType", tag = "2")]
    pub oracle_type: i32,
}
impl ::prost::Name for OracleInfo {
    const NAME: &'static str = "OracleInfo";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainlinkPriceState {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub answer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
    #[prost(message, optional, tag = "4")]
    pub price_state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for ChainlinkPriceState {
    const NAME: &'static str = "ChainlinkPriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandPriceState {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rate: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub resolve_time: u64,
    #[prost(uint64, tag = "4")]
    pub request_id: u64,
    #[prost(message, optional, tag = "5")]
    pub price_state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for BandPriceState {
    const NAME: &'static str = "BandPriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedState {
    #[prost(string, tag = "1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub quote: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub price_state: ::core::option::Option<PriceState>,
    #[prost(string, repeated, tag = "4")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for PriceFeedState {
    const NAME: &'static str = "PriceFeedState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderInfo {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for ProviderInfo {
    const NAME: &'static str = "ProviderInfo";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderState {
    #[prost(message, optional, tag = "1")]
    pub provider_info: ::core::option::Option<ProviderInfo>,
    #[prost(message, repeated, tag = "2")]
    pub provider_price_states: ::prost::alloc::vec::Vec<ProviderPriceState>,
}
impl ::prost::Name for ProviderState {
    const NAME: &'static str = "ProviderState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderPriceState {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for ProviderPriceState {
    const NAME: &'static str = "ProviderPriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedInfo {
    #[prost(string, tag = "1")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub quote: ::prost::alloc::string::String,
}
impl ::prost::Name for PriceFeedInfo {
    const NAME: &'static str = "PriceFeedInfo";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedPrice {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for PriceFeedPrice {
    const NAME: &'static str = "PriceFeedPrice";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinbasePriceState {
    /// kind should always be "prices"
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// timestamp of the when the price was signed by coinbase
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    /// the symbol of the price, e.g. BTC
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
    /// the value of the price scaled by 1e6
    #[prost(uint64, tag = "4")]
    pub value: u64,
    /// the price state
    #[prost(message, optional, tag = "5")]
    pub price_state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for CoinbasePriceState {
    const NAME: &'static str = "CoinbasePriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorkPriceState {
    /// timestamp of the when the price was signed by stork
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// the symbol of the price, e.g. BTC
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    /// the value of the price scaled by 1e18
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    /// the price state
    #[prost(message, optional, tag = "5")]
    pub price_state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for StorkPriceState {
    const NAME: &'static str = "StorkPriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceState {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
impl ::prost::Name for PriceState {
    const NAME: &'static str = "PriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PythPriceState {
    #[prost(string, tag = "1")]
    pub price_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ema_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ema_conf: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub conf: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub publish_time: u64,
    #[prost(message, optional, tag = "6")]
    pub price_state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for PythPriceState {
    const NAME: &'static str = "PythPriceState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandOracleRequest {
    /// Unique Identifier for band ibc oracle request
    #[prost(uint64, tag = "1")]
    pub request_id: u64,
    /// OracleScriptID is the unique identifier of the oracle script to be
    /// executed.
    #[prost(int64, tag = "2")]
    pub oracle_script_id: i64,
    /// Symbols is the list of symbols to prepare in the calldata
    #[prost(string, repeated, tag = "3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AskCount is the number of validators that are requested to respond to this
    /// oracle request. Higher value means more security, at a higher gas cost.
    #[prost(uint64, tag = "4")]
    pub ask_count: u64,
    /// MinCount is the minimum number of validators necessary for the request to
    /// proceed to the execution phase. Higher value means more security, at the
    /// cost of liveness.
    #[prost(uint64, tag = "5")]
    pub min_count: u64,
    /// FeeLimit is the maximum tokens that will be paid to all data source
    /// providers.
    #[prost(message, repeated, tag = "6")]
    pub fee_limit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// PrepareGas is amount of gas to pay to prepare raw requests
    #[prost(uint64, tag = "7")]
    pub prepare_gas: u64,
    /// ExecuteGas is amount of gas to reserve for executing
    #[prost(uint64, tag = "8")]
    pub execute_gas: u64,
    /// MinSourceCount is the minimum number of data sources that must be used by
    /// each validator
    #[prost(uint64, tag = "9")]
    pub min_source_count: u64,
}
impl ::prost::Name for BandOracleRequest {
    const NAME: &'static str = "BandOracleRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandIbcParams {
    /// true if Band IBC should be enabled
    #[prost(bool, tag = "1")]
    pub band_ibc_enabled: bool,
    /// block request interval to send Band IBC prices
    #[prost(int64, tag = "2")]
    pub ibc_request_interval: i64,
    /// band IBC source channel
    #[prost(string, tag = "3")]
    pub ibc_source_channel: ::prost::alloc::string::String,
    /// band IBC version
    #[prost(string, tag = "4")]
    pub ibc_version: ::prost::alloc::string::String,
    /// band IBC portID
    #[prost(string, tag = "5")]
    pub ibc_port_id: ::prost::alloc::string::String,
    ///   legacy oracle scheme ids
    #[prost(int64, repeated, tag = "6")]
    pub legacy_oracle_ids: ::prost::alloc::vec::Vec<i64>,
}
impl ::prost::Name for BandIbcParams {
    const NAME: &'static str = "BandIBCParams";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolPriceTimestamp {
    #[prost(enumeration = "OracleType", tag = "1")]
    pub oracle: i32,
    #[prost(string, tag = "2")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
impl ::prost::Name for SymbolPriceTimestamp {
    const NAME: &'static str = "SymbolPriceTimestamp";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceTimestamps {
    #[prost(message, repeated, tag = "1")]
    pub last_price_timestamps: ::prost::alloc::vec::Vec<SymbolPriceTimestamp>,
}
impl ::prost::Name for LastPriceTimestamps {
    const NAME: &'static str = "LastPriceTimestamps";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRecords {
    #[prost(enumeration = "OracleType", tag = "1")]
    pub oracle: i32,
    #[prost(string, tag = "2")]
    pub symbol_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub latest_price_records: ::prost::alloc::vec::Vec<PriceRecord>,
}
impl ::prost::Name for PriceRecords {
    const NAME: &'static str = "PriceRecords";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceRecord {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for PriceRecord {
    const NAME: &'static str = "PriceRecord";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MetadataStatistics refers to the metadata summary statistics of the
/// historical sample considered
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataStatistics {
    /// GroupCount refers to the number of groups used. Equals RecordsSampleSize if
    /// no grouping is used
    #[prost(uint32, tag = "1")]
    pub group_count: u32,
    /// RecordsSampleSize refers to the total number of records used.
    #[prost(uint32, tag = "2")]
    pub records_sample_size: u32,
    /// Mean refers to the arithmetic mean
    /// For trades, the mean is the VWAP computed over the grouped trade records ∑
    /// (price * quantity) / ∑ quantity For oracle prices, the mean is computed
    /// over the price records ∑ (price) / prices_count
    #[prost(string, tag = "3")]
    pub mean: ::prost::alloc::string::String,
    /// TWAP refers to the time-weighted average price which equals ∑ (price_i *
    /// ∆t_i) / ∑ ∆t_i where ∆t_i = t_i - t_{i-1}
    #[prost(string, tag = "4")]
    pub twap: ::prost::alloc::string::String,
    /// FirstTimestamp is the timestamp of the oldest record considered
    #[prost(int64, tag = "5")]
    pub first_timestamp: i64,
    /// LastTimestamp is the timestamp of the youngest record considered
    #[prost(int64, tag = "6")]
    pub last_timestamp: i64,
    /// MinPrice refers to the smallest individual raw price considered
    #[prost(string, tag = "7")]
    pub min_price: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(string, tag = "8")]
    pub max_price: ::prost::alloc::string::String,
    /// MedianPrice refers to the median individual raw price considered
    #[prost(string, tag = "9")]
    pub median_price: ::prost::alloc::string::String,
}
impl ::prost::Name for MetadataStatistics {
    const NAME: &'static str = "MetadataStatistics";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceAttestation {
    #[prost(string, tag = "1")]
    pub price_id: ::prost::alloc::string::String,
    /// MaxPrice refers to the largest individual raw price considered
    #[prost(int64, tag = "2")]
    pub price: i64,
    #[prost(uint64, tag = "3")]
    pub conf: u64,
    #[prost(int32, tag = "4")]
    pub expo: i32,
    #[prost(int64, tag = "5")]
    pub ema_price: i64,
    #[prost(uint64, tag = "6")]
    pub ema_conf: u64,
    #[prost(int32, tag = "7")]
    pub ema_expo: i32,
    #[prost(int64, tag = "8")]
    pub publish_time: i64,
}
impl ::prost::Name for PriceAttestation {
    const NAME: &'static str = "PriceAttestation";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPair {
    #[prost(string, tag = "1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub signed_prices: ::prost::alloc::vec::Vec<SignedPriceOfAssetPair>,
}
impl ::prost::Name for AssetPair {
    const NAME: &'static str = "AssetPair";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedPriceOfAssetPair {
    #[prost(string, tag = "1")]
    pub publisher_key: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for SignedPriceOfAssetPair {
    const NAME: &'static str = "SignedPriceOfAssetPair";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OracleType {
    Unspecified = 0,
    Band = 1,
    PriceFeed = 2,
    Coinbase = 3,
    Chainlink = 4,
    Razor = 5,
    Dia = 6,
    Api3 = 7,
    Uma = 8,
    Pyth = 9,
    BandIbc = 10,
    Provider = 11,
    Stork = 12,
}
impl OracleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OracleType::Unspecified => "Unspecified",
            OracleType::Band => "Band",
            OracleType::PriceFeed => "PriceFeed",
            OracleType::Coinbase => "Coinbase",
            OracleType::Chainlink => "Chainlink",
            OracleType::Razor => "Razor",
            OracleType::Dia => "Dia",
            OracleType::Api3 => "API3",
            OracleType::Uma => "Uma",
            OracleType::Pyth => "Pyth",
            OracleType::BandIbc => "BandIBC",
            OracleType::Provider => "Provider",
            OracleType::Stork => "Stork",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Band" => Some(Self::Band),
            "PriceFeed" => Some(Self::PriceFeed),
            "Coinbase" => Some(Self::Coinbase),
            "Chainlink" => Some(Self::Chainlink),
            "Razor" => Some(Self::Razor),
            "Dia" => Some(Self::Dia),
            "API3" => Some(Self::Api3),
            "Uma" => Some(Self::Uma),
            "Pyth" => Some(Self::Pyth),
            "BandIBC" => Some(Self::BandIbc),
            "Provider" => Some(Self::Provider),
            "Stork" => Some(Self::Stork),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetChainlinkPriceEvent {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub answer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
}
impl ::prost::Name for SetChainlinkPriceEvent {
    const NAME: &'static str = "SetChainlinkPriceEvent";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// Event type upon set ref
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBandPriceEvent {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub resolve_time: u64,
    #[prost(uint64, tag = "5")]
    pub request_id: u64,
}
impl ::prost::Name for SetBandPriceEvent {
    const NAME: &'static str = "SetBandPriceEvent";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBandIbcPriceEvent {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub prices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    pub resolve_time: u64,
    #[prost(uint64, tag = "5")]
    pub request_id: u64,
    #[prost(int64, tag = "6")]
    pub client_id: i64,
}
impl ::prost::Name for SetBandIbcPriceEvent {
    const NAME: &'static str = "SetBandIBCPriceEvent";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBandIbcAckSuccess {
    #[prost(string, tag = "1")]
    pub ack_result: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub client_id: i64,
}
impl ::prost::Name for EventBandIbcAckSuccess {
    const NAME: &'static str = "EventBandIBCAckSuccess";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBandIbcAckError {
    #[prost(string, tag = "1")]
    pub ack_error: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub client_id: i64,
}
impl ::prost::Name for EventBandIbcAckError {
    const NAME: &'static str = "EventBandIBCAckError";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBandIbcResponseTimeout {
    #[prost(int64, tag = "1")]
    pub client_id: i64,
}
impl ::prost::Name for EventBandIbcResponseTimeout {
    const NAME: &'static str = "EventBandIBCResponseTimeout";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPriceFeedPriceEvent {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote: ::prost::alloc::string::String,
    /// price defines the price of the oracle base and quote
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for SetPriceFeedPriceEvent {
    const NAME: &'static str = "SetPriceFeedPriceEvent";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetProviderPriceEvent {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for SetProviderPriceEvent {
    const NAME: &'static str = "SetProviderPriceEvent";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCoinbasePriceEvent {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
}
impl ::prost::Name for SetCoinbasePriceEvent {
    const NAME: &'static str = "SetCoinbasePriceEvent";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetStorkPrices {
    #[prost(message, repeated, tag = "1")]
    pub prices: ::prost::alloc::vec::Vec<StorkPriceState>,
}
impl ::prost::Name for EventSetStorkPrices {
    const NAME: &'static str = "EventSetStorkPrices";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetPythPrices {
    #[prost(message, repeated, tag = "1")]
    pub prices: ::prost::alloc::vec::Vec<PythPriceState>,
}
impl ::prost::Name for EventSetPythPrices {
    const NAME: &'static str = "EventSetPythPrices";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// GenesisState defines the oracle module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to oracle.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, repeated, tag = "2")]
    pub band_relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub band_price_states: ::prost::alloc::vec::Vec<BandPriceState>,
    #[prost(message, repeated, tag = "4")]
    pub price_feed_price_states: ::prost::alloc::vec::Vec<PriceFeedState>,
    #[prost(message, repeated, tag = "5")]
    pub coinbase_price_states: ::prost::alloc::vec::Vec<CoinbasePriceState>,
    #[prost(message, repeated, tag = "6")]
    pub band_ibc_price_states: ::prost::alloc::vec::Vec<BandPriceState>,
    #[prost(message, repeated, tag = "7")]
    pub band_ibc_oracle_requests: ::prost::alloc::vec::Vec<BandOracleRequest>,
    #[prost(message, optional, tag = "8")]
    pub band_ibc_params: ::core::option::Option<BandIbcParams>,
    #[prost(uint64, tag = "9")]
    pub band_ibc_latest_client_id: u64,
    #[prost(message, repeated, tag = "10")]
    pub calldata_records: ::prost::alloc::vec::Vec<CalldataRecord>,
    #[prost(uint64, tag = "11")]
    pub band_ibc_latest_request_id: u64,
    #[prost(message, repeated, tag = "12")]
    pub chainlink_price_states: ::prost::alloc::vec::Vec<ChainlinkPriceState>,
    #[prost(message, repeated, tag = "13")]
    pub historical_price_records: ::prost::alloc::vec::Vec<PriceRecords>,
    #[prost(message, repeated, tag = "14")]
    pub provider_states: ::prost::alloc::vec::Vec<ProviderState>,
    #[prost(message, repeated, tag = "15")]
    pub pyth_price_states: ::prost::alloc::vec::Vec<PythPriceState>,
    #[prost(message, repeated, tag = "16")]
    pub stork_price_states: ::prost::alloc::vec::Vec<StorkPriceState>,
    #[prost(string, repeated, tag = "17")]
    pub stork_publishers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalldataRecord {
    #[prost(uint64, tag = "1")]
    pub client_id: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub calldata: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CalldataRecord {
    const NAME: &'static str = "CalldataRecord";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantBandOraclePrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GrantBandOraclePrivilegeProposal {
    const NAME: &'static str = "GrantBandOraclePrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeBandOraclePrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for RevokeBandOraclePrivilegeProposal {
    const NAME: &'static str = "RevokeBandOraclePrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantPriceFeederPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub quote: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GrantPriceFeederPrivilegeProposal {
    const NAME: &'static str = "GrantPriceFeederPrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantProviderPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GrantProviderPrivilegeProposal {
    const NAME: &'static str = "GrantProviderPrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeProviderPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for RevokeProviderPrivilegeProposal {
    const NAME: &'static str = "RevokeProviderPrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokePriceFeederPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub quote: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for RevokePriceFeederPrivilegeProposal {
    const NAME: &'static str = "RevokePriceFeederPrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeBandOracleRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<BandOracleRequest>,
}
impl ::prost::Name for AuthorizeBandOracleRequestProposal {
    const NAME: &'static str = "AuthorizeBandOracleRequestProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBandOracleRequestProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "3")]
    pub delete_request_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "4")]
    pub update_oracle_request: ::core::option::Option<BandOracleRequest>,
}
impl ::prost::Name for UpdateBandOracleRequestProposal {
    const NAME: &'static str = "UpdateBandOracleRequestProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableBandIbcProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub band_ibc_params: ::core::option::Option<BandIbcParams>,
}
impl ::prost::Name for EnableBandIbcProposal {
    const NAME: &'static str = "EnableBandIBCProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantStorkPublisherPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub stork_publishers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GrantStorkPublisherPrivilegeProposal {
    const NAME: &'static str = "GrantStorkPublisherPrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeStorkPublisherPrivilegeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub stork_publishers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for RevokeStorkPublisherPrivilegeProposal {
    const NAME: &'static str = "RevokeStorkPublisherPrivilegeProposal";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPythPriceRequest {
    #[prost(string, tag = "1")]
    pub price_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPythPriceRequest {
    const NAME: &'static str = "QueryPythPriceRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPythPriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price_state: ::core::option::Option<PythPriceState>,
}
impl ::prost::Name for QueryPythPriceResponse {
    const NAME: &'static str = "QueryPythPriceResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryOracleParamsRequest is the request type for the Query/OracleParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryOracleParamsResponse is the response type for the Query/OracleParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryBandRelayersRequest is the request type for the Query/BandRelayers RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBandRelayersRequest {}
impl ::prost::Name for QueryBandRelayersRequest {
    const NAME: &'static str = "QueryBandRelayersRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryBandRelayersResponse is the response type for the Query/BandRelayers RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBandRelayersResponse {
    #[prost(string, repeated, tag = "1")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryBandRelayersResponse {
    const NAME: &'static str = "QueryBandRelayersResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryBandPriceStatesRequest is the request type for the Query/BandPriceStates
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBandPriceStatesRequest {}
impl ::prost::Name for QueryBandPriceStatesRequest {
    const NAME: &'static str = "QueryBandPriceStatesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryBandPriceStatesResponse is the response type for the
/// Query/BandPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBandPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<BandPriceState>,
}
impl ::prost::Name for QueryBandPriceStatesResponse {
    const NAME: &'static str = "QueryBandPriceStatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryBandIBCPriceStatesRequest is the request type for the
/// Query/BandIBCPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBandIbcPriceStatesRequest {}
impl ::prost::Name for QueryBandIbcPriceStatesRequest {
    const NAME: &'static str = "QueryBandIBCPriceStatesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryBandIBCPriceStatesResponse is the response type for the
/// Query/BandIBCPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBandIbcPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<BandPriceState>,
}
impl ::prost::Name for QueryBandIbcPriceStatesResponse {
    const NAME: &'static str = "QueryBandIBCPriceStatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryPriceFeedPriceStatesRequest is the request type for the
/// Query/PriceFeedPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPriceFeedPriceStatesRequest {}
impl ::prost::Name for QueryPriceFeedPriceStatesRequest {
    const NAME: &'static str = "QueryPriceFeedPriceStatesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryPriceFeedPriceStatesResponse is the response type for the
/// Query/PriceFeedPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPriceFeedPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<PriceFeedState>,
}
impl ::prost::Name for QueryPriceFeedPriceStatesResponse {
    const NAME: &'static str = "QueryPriceFeedPriceStatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryCoinbasePriceStatesRequest is the request type for the
/// Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCoinbasePriceStatesRequest {}
impl ::prost::Name for QueryCoinbasePriceStatesRequest {
    const NAME: &'static str = "QueryCoinbasePriceStatesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryCoinbasePriceStatesResponse is the response type for the
/// Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCoinbasePriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<CoinbasePriceState>,
}
impl ::prost::Name for QueryCoinbasePriceStatesResponse {
    const NAME: &'static str = "QueryCoinbasePriceStatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryPythPriceStatesRequest is the request type for the
/// Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPythPriceStatesRequest {}
impl ::prost::Name for QueryPythPriceStatesRequest {
    const NAME: &'static str = "QueryPythPriceStatesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryPythPriceStatesResponse is the response type for the
/// Query/CoinbasePriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPythPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<PythPriceState>,
}
impl ::prost::Name for QueryPythPriceStatesResponse {
    const NAME: &'static str = "QueryPythPriceStatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryStorkPriceStatesRequest is the request type for the
/// Query/StorkPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorkPriceStatesRequest {}
impl ::prost::Name for QueryStorkPriceStatesRequest {
    const NAME: &'static str = "QueryStorkPriceStatesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryStorkPriceStatesResponse is the response type for the
/// Query/StorkPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorkPriceStatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_states: ::prost::alloc::vec::Vec<StorkPriceState>,
}
impl ::prost::Name for QueryStorkPriceStatesResponse {
    const NAME: &'static str = "QueryStorkPriceStatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryStorkPublishersRequest is the request type for the
/// Query/StorkPublishers RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorkPublishersRequest {}
impl ::prost::Name for QueryStorkPublishersRequest {
    const NAME: &'static str = "QueryStorkPublishersRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryStorkPublishersResponse is the response type for the
/// Query/StorkPublishers RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorkPublishersResponse {
    #[prost(string, repeated, tag = "1")]
    pub publishers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryStorkPublishersResponse {
    const NAME: &'static str = "QueryStorkPublishersResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryProviderPriceStateRequest is the request type for the
/// Query/ProviderPriceState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProviderPriceStateRequest {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryProviderPriceStateRequest {
    const NAME: &'static str = "QueryProviderPriceStateRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryProviderPriceStatesResponse is the response type for the
/// Query/ProviderPriceStates RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProviderPriceStateResponse {
    #[prost(message, optional, tag = "1")]
    pub price_state: ::core::option::Option<PriceState>,
}
impl ::prost::Name for QueryProviderPriceStateResponse {
    const NAME: &'static str = "QueryProviderPriceStateResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateRequest is the request type for the Query/OracleModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {}
impl ::prost::Name for QueryModuleStateRequest {
    const NAME: &'static str = "QueryModuleStateRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateResponse is the response type for the Query/OracleModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
impl ::prost::Name for QueryModuleStateResponse {
    const NAME: &'static str = "QueryModuleStateResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalPriceRecordsRequest {
    #[prost(enumeration = "OracleType", tag = "1")]
    pub oracle: i32,
    #[prost(string, tag = "2")]
    pub symbol_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryHistoricalPriceRecordsRequest {
    const NAME: &'static str = "QueryHistoricalPriceRecordsRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHistoricalPriceRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub price_records: ::prost::alloc::vec::Vec<PriceRecords>,
}
impl ::prost::Name for QueryHistoricalPriceRecordsResponse {
    const NAME: &'static str = "QueryHistoricalPriceRecordsResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleHistoryOptions {
    /// MaxAge restricts the oracle price records oldest age in seconds from the
    /// current block time to consider. A value of 0 means use all the records
    /// present on the chain.
    #[prost(uint64, tag = "1")]
    pub max_age: u64,
    /// If IncludeRawHistory is true, the raw underlying data used for the
    /// computation is included in the response
    #[prost(bool, tag = "2")]
    pub include_raw_history: bool,
    /// If IncludeMetadata is true, metadata on the computation is included in the
    /// response
    #[prost(bool, tag = "3")]
    pub include_metadata: bool,
}
impl ::prost::Name for OracleHistoryOptions {
    const NAME: &'static str = "OracleHistoryOptions";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryOracleVolatilityRequest is the request type for Query/OracleVolatility
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleVolatilityRequest {
    #[prost(message, optional, tag = "1")]
    pub base_info: ::core::option::Option<OracleInfo>,
    #[prost(message, optional, tag = "2")]
    pub quote_info: ::core::option::Option<OracleInfo>,
    #[prost(message, optional, tag = "3")]
    pub oracle_history_options: ::core::option::Option<OracleHistoryOptions>,
}
impl ::prost::Name for QueryOracleVolatilityRequest {
    const NAME: &'static str = "QueryOracleVolatilityRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryOracleVolatilityResponse is the response type for Query/OracleVolatility
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleVolatilityResponse {
    #[prost(string, tag = "1")]
    pub volatility: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub history_metadata: ::core::option::Option<MetadataStatistics>,
    #[prost(message, repeated, tag = "3")]
    pub raw_history: ::prost::alloc::vec::Vec<PriceRecord>,
}
impl ::prost::Name for QueryOracleVolatilityResponse {
    const NAME: &'static str = "QueryOracleVolatilityResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProvidersInfoRequest {}
impl ::prost::Name for QueryOracleProvidersInfoRequest {
    const NAME: &'static str = "QueryOracleProvidersInfoRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProvidersInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub providers: ::prost::alloc::vec::Vec<ProviderInfo>,
}
impl ::prost::Name for QueryOracleProvidersInfoResponse {
    const NAME: &'static str = "QueryOracleProvidersInfoResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProviderPricesRequest {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryOracleProviderPricesRequest {
    const NAME: &'static str = "QueryOracleProviderPricesRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOracleProviderPricesResponse {
    #[prost(message, repeated, tag = "1")]
    pub provider_state: ::prost::alloc::vec::Vec<ProviderState>,
}
impl ::prost::Name for QueryOracleProviderPricesResponse {
    const NAME: &'static str = "QueryOracleProviderPricesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// ScalingOptions defines optional configuration to avoid precision loss. The
/// oracle result will be returned as base_price * 10^base_decimals / quote_price
/// * 10^quote_decimals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalingOptions {
    #[prost(uint32, tag = "1")]
    pub base_decimals: u32,
    #[prost(uint32, tag = "2")]
    pub quote_decimals: u32,
}
impl ::prost::Name for ScalingOptions {
    const NAME: &'static str = "ScalingOptions";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryOraclePriceRequest is the request type for the Query/OraclePrice RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOraclePriceRequest {
    #[prost(enumeration = "OracleType", tag = "1")]
    pub oracle_type: i32,
    #[prost(string, tag = "2")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub scaling_options: ::core::option::Option<ScalingOptions>,
}
impl ::prost::Name for QueryOraclePriceRequest {
    const NAME: &'static str = "QueryOraclePriceRequest";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricePairState {
    #[prost(string, tag = "1")]
    pub pair_price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub quote_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub base_cumulative_price: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub quote_cumulative_price: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub base_timestamp: i64,
    #[prost(int64, tag = "7")]
    pub quote_timestamp: i64,
}
impl ::prost::Name for PricePairState {
    const NAME: &'static str = "PricePairState";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// QueryOraclePriceResponse is the response type for the Query/OraclePrice RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOraclePriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price_pair_state: ::core::option::Option<PricePairState>,
}
impl ::prost::Name for QueryOraclePriceResponse {
    const NAME: &'static str = "QueryOraclePriceResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRelayProviderPrice defines a SDK message for setting a price through the
/// provider oracle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayProviderPrices {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub prices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgRelayProviderPrices {
    const NAME: &'static str = "MsgRelayProviderPrices";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayProviderPricesResponse {}
impl ::prost::Name for MsgRelayProviderPricesResponse {
    const NAME: &'static str = "MsgRelayProviderPricesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRelayPriceFeedPrice defines a SDK message for setting a price through the
/// pricefeed oracle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPriceFeedPrice {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub base: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub quote: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// price defines the price of the oracle base and quote
    #[prost(string, repeated, tag = "4")]
    pub price: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgRelayPriceFeedPrice {
    const NAME: &'static str = "MsgRelayPriceFeedPrice";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPriceFeedPriceResponse {}
impl ::prost::Name for MsgRelayPriceFeedPriceResponse {
    const NAME: &'static str = "MsgRelayPriceFeedPriceResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayBandRates {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, repeated, tag = "3")]
    pub rates: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "4")]
    pub resolve_times: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "5")]
    pub request_i_ds: ::prost::alloc::vec::Vec<u64>,
}
impl ::prost::Name for MsgRelayBandRates {
    const NAME: &'static str = "MsgRelayBandRates";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayBandRatesResponse {}
impl ::prost::Name for MsgRelayBandRatesResponse {
    const NAME: &'static str = "MsgRelayBandRatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRelayCoinbaseMessages defines a SDK message for relaying price messages
/// from Coinbase API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayCoinbaseMessages {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for MsgRelayCoinbaseMessages {
    const NAME: &'static str = "MsgRelayCoinbaseMessages";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayCoinbaseMessagesResponse {}
impl ::prost::Name for MsgRelayCoinbaseMessagesResponse {
    const NAME: &'static str = "MsgRelayCoinbaseMessagesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRelayStorkPrices defines a SDK message for relaying price message
/// from Stork API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayStorkPrices {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub asset_pairs: ::prost::alloc::vec::Vec<AssetPair>,
}
impl ::prost::Name for MsgRelayStorkPrices {
    const NAME: &'static str = "MsgRelayStorkPrices";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayStorkPricesResponse {}
impl ::prost::Name for MsgRelayStorkPricesResponse {
    const NAME: &'static str = "MsgRelayStorkPricesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRequestBandIBCRates defines a SDK message for requesting data from
/// BandChain using IBC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBandIbcRates {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub request_id: u64,
}
impl ::prost::Name for MsgRequestBandIbcRates {
    const NAME: &'static str = "MsgRequestBandIBCRates";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRequestDataResponse defines the Msg/RequestBandIBCRates response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBandIbcRatesResponse {}
impl ::prost::Name for MsgRequestBandIbcRatesResponse {
    const NAME: &'static str = "MsgRequestBandIBCRatesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRelayPythPrices defines a SDK message for updating Pyth prices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPythPrices {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub price_attestations: ::prost::alloc::vec::Vec<PriceAttestation>,
}
impl ::prost::Name for MsgRelayPythPrices {
    const NAME: &'static str = "MsgRelayPythPrices";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
/// MsgRelayPythPricesResponse defines the Msg/RelayPythPrices response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRelayPythPricesResponse {}
impl ::prost::Name for MsgRelayPythPricesResponse {
    const NAME: &'static str = "MsgRelayPythPricesResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the oracle parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.oracle.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.oracle.v1beta1.{}", Self::NAME)
    }
}
include!("injective.oracle.v1beta1.serde.rs");
include!("injective.oracle.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
