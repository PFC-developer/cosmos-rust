// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Native denom for LINK coin in the bank keeper
    #[prost(string, tag = "1")]
    pub link_denom: ::prost::alloc::string::String,
    /// The block number interval at which payouts are made
    #[prost(uint64, tag = "2")]
    pub payout_block_interval: u64,
    /// The admin for the OCR module
    #[prost(string, tag = "3")]
    pub module_admin: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedConfig {
    /// signers ith element is address ith oracle uses to sign a report
    #[prost(string, repeated, tag = "1")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// transmitters ith element is address ith oracle uses to transmit a report
    /// via the transmit method
    #[prost(string, repeated, tag = "2")]
    pub transmitters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// f maximum number of faulty/dishonest oracles the protocol can tolerate
    /// while still working correctly
    #[prost(uint32, tag = "3")]
    pub f: u32,
    /// onchain_config serialized data with reporting plugin params on chain.
    #[prost(bytes = "vec", tag = "4")]
    pub onchain_config: ::prost::alloc::vec::Vec<u8>,
    /// offchain_config_version version of the serialization format used for
    /// "offchain_config" parameter
    #[prost(uint64, tag = "5")]
    pub offchain_config_version: u64,
    /// offchain_config serialized data used by oracles to configure their offchain
    /// operation
    #[prost(bytes = "vec", tag = "6")]
    pub offchain_config: ::prost::alloc::vec::Vec<u8>,
    /// feed-specific params for the Cosmos module.
    #[prost(message, optional, tag = "7")]
    pub module_params: ::core::option::Option<ModuleParams>,
}
impl ::prost::Name for FeedConfig {
    const NAME: &'static str = "FeedConfig";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedConfigInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub latest_config_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub f: u32,
    #[prost(uint32, tag = "3")]
    pub n: u32,
    /// config_count ordinal number of this config setting among all config
    /// settings
    #[prost(uint64, tag = "4")]
    pub config_count: u64,
    #[prost(int64, tag = "5")]
    pub latest_config_block_number: i64,
}
impl ::prost::Name for FeedConfigInfo {
    const NAME: &'static str = "FeedConfigInfo";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleParams {
    /// feed_id is an unique ID for the target of this config
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    /// lowest answer the median of a report is allowed to be
    #[prost(string, tag = "2")]
    pub min_answer: ::prost::alloc::string::String,
    /// highest answer the median of a report is allowed to be
    #[prost(string, tag = "3")]
    pub max_answer: ::prost::alloc::string::String,
    /// Fixed LINK reward for each observer
    #[prost(string, tag = "4")]
    pub link_per_observation: ::prost::alloc::string::String,
    /// Fixed LINK reward for transmitter
    #[prost(string, tag = "5")]
    pub link_per_transmission: ::prost::alloc::string::String,
    /// Native denom for LINK coin in the bank keeper
    #[prost(string, tag = "6")]
    pub link_denom: ::prost::alloc::string::String,
    /// Enables unique reports
    #[prost(bool, tag = "7")]
    pub unique_reports: bool,
    /// short human-readable description of observable this feed's answers pertain
    /// to
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// feed administrator
    #[prost(string, tag = "9")]
    pub feed_admin: ::prost::alloc::string::String,
    /// feed billing administrator
    #[prost(string, tag = "10")]
    pub billing_admin: ::prost::alloc::string::String,
}
impl ::prost::Name for ModuleParams {
    const NAME: &'static str = "ModuleParams";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractConfig {
    /// config_count ordinal number of this config setting among all config
    /// settings
    #[prost(uint64, tag = "1")]
    pub config_count: u64,
    /// signers ith element is address ith oracle uses to sign a report
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// transmitters ith element is address ith oracle uses to transmit a report
    /// via the transmit method
    #[prost(string, repeated, tag = "3")]
    pub transmitters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// f maximum number of faulty/dishonest oracles the protocol can tolerate
    /// while still working correctly
    #[prost(uint32, tag = "4")]
    pub f: u32,
    /// onchain_config serialized data with reporting plugin params on chain.
    #[prost(bytes = "vec", tag = "5")]
    pub onchain_config: ::prost::alloc::vec::Vec<u8>,
    /// offchain_config_version version of the serialization format used for
    /// "offchain_config" parameter
    #[prost(uint64, tag = "6")]
    pub offchain_config_version: u64,
    /// offchain_config serialized data used by oracles to configure their offchain
    /// operation
    #[prost(bytes = "vec", tag = "7")]
    pub offchain_config: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ContractConfig {
    const NAME: &'static str = "ContractConfig";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<FeedConfig>,
}
impl ::prost::Name for SetConfigProposal {
    const NAME: &'static str = "SetConfigProposal";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedProperties {
    /// feed_id is an unique ID for the target of this config
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    /// f maximum number of faulty/dishonest oracles the protocol can tolerate
    /// while still working correctly
    #[prost(uint32, tag = "2")]
    pub f: u32,
    /// onchain_config serialized data with reporting plugin params on chain.
    #[prost(bytes = "vec", tag = "3")]
    pub onchain_config: ::prost::alloc::vec::Vec<u8>,
    /// offchain_config_version version of the serialization format used for
    /// "offchain_config" parameter
    #[prost(uint64, tag = "4")]
    pub offchain_config_version: u64,
    /// offchain_config serialized data used by oracles to configure their offchain
    /// operation
    #[prost(bytes = "vec", tag = "5")]
    pub offchain_config: ::prost::alloc::vec::Vec<u8>,
    /// lowest answer the median of a report is allowed to be
    #[prost(string, tag = "6")]
    pub min_answer: ::prost::alloc::string::String,
    /// highest answer the median of a report is allowed to be
    #[prost(string, tag = "7")]
    pub max_answer: ::prost::alloc::string::String,
    /// Fixed LINK reward for each observer
    #[prost(string, tag = "8")]
    pub link_per_observation: ::prost::alloc::string::String,
    /// Fixed LINK reward for transmitter
    #[prost(string, tag = "9")]
    pub link_per_transmission: ::prost::alloc::string::String,
    /// Enables unique reports
    #[prost(bool, tag = "10")]
    pub unique_reports: bool,
    /// short human-readable description of observable this feed's answers pertain
    /// to
    #[prost(string, tag = "11")]
    pub description: ::prost::alloc::string::String,
}
impl ::prost::Name for FeedProperties {
    const NAME: &'static str = "FeedProperties";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBatchConfigProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// signers ith element is address ith oracle uses to sign a report
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// transmitters ith element is address ith oracle uses to transmit a report
    /// via the transmit method
    #[prost(string, repeated, tag = "4")]
    pub transmitters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Native denom for LINK coin in the bank keeper
    #[prost(string, tag = "5")]
    pub link_denom: ::prost::alloc::string::String,
    /// feed properties
    #[prost(message, repeated, tag = "6")]
    pub feed_properties: ::prost::alloc::vec::Vec<FeedProperties>,
}
impl ::prost::Name for SetBatchConfigProposal {
    const NAME: &'static str = "SetBatchConfigProposal";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleObservationsCounts {
    #[prost(uint32, repeated, tag = "1")]
    pub counts: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for OracleObservationsCounts {
    const NAME: &'static str = "OracleObservationsCounts";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
/// LINK-INJ-denominated reimbursements for gas used by transmitters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasReimbursements {
    #[prost(message, repeated, tag = "1")]
    pub reimbursements: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for GasReimbursements {
    const NAME: &'static str = "GasReimbursements";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payee {
    #[prost(string, tag = "1")]
    pub transmitter_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub payment_addr: ::prost::alloc::string::String,
}
impl ::prost::Name for Payee {
    const NAME: &'static str = "Payee";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
/// Transmission records the median answer from the transmit transaction at
/// time timestamp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transmission {
    #[prost(string, tag = "1")]
    pub answer: ::prost::alloc::string::String,
    /// when were observations made offchain
    #[prost(int64, tag = "2")]
    pub observations_timestamp: i64,
    /// when was report received onchain
    #[prost(int64, tag = "3")]
    pub transmission_timestamp: i64,
}
impl ::prost::Name for Transmission {
    const NAME: &'static str = "Transmission";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochAndRound {
    #[prost(uint64, tag = "1")]
    pub epoch: u64,
    #[prost(uint64, tag = "2")]
    pub round: u64,
}
impl ::prost::Name for EpochAndRound {
    const NAME: &'static str = "EpochAndRound";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    #[prost(int64, tag = "1")]
    pub observations_timestamp: i64,
    /// ith element is the index of the ith observer
    #[prost(bytes = "vec", tag = "2")]
    pub observers: ::prost::alloc::vec::Vec<u8>,
    /// ith element is the ith observation
    #[prost(string, repeated, tag = "3")]
    pub observations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for Report {
    const NAME: &'static str = "Report";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportToSign {
    #[prost(bytes = "vec", tag = "1")]
    pub config_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub epoch: u64,
    #[prost(uint64, tag = "3")]
    pub round: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub extra_hash: ::prost::alloc::vec::Vec<u8>,
    /// Opaque report
    #[prost(bytes = "vec", tag = "5")]
    pub report: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ReportToSign {
    const NAME: &'static str = "ReportToSign";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOraclePaid {
    #[prost(string, tag = "1")]
    pub transmitter_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub payee_addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventOraclePaid {
    const NAME: &'static str = "EventOraclePaid";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAnswerUpdated {
    #[prost(string, tag = "1")]
    pub current: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub round_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub updated_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for EventAnswerUpdated {
    const NAME: &'static str = "EventAnswerUpdated";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewRound {
    #[prost(string, tag = "1")]
    pub round_id: ::prost::alloc::string::String,
    /// address of starter
    #[prost(string, tag = "2")]
    pub started_by: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub started_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for EventNewRound {
    const NAME: &'static str = "EventNewRound";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTransmitted {
    #[prost(bytes = "vec", tag = "1")]
    pub config_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub epoch: u64,
}
impl ::prost::Name for EventTransmitted {
    const NAME: &'static str = "EventTransmitted";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNewTransmission {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub aggregator_round_id: u32,
    #[prost(string, tag = "3")]
    pub answer: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transmitter: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub observations_timestamp: i64,
    #[prost(string, repeated, tag = "6")]
    pub observations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "7")]
    pub observers: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub config_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "9")]
    pub epoch_and_round: ::core::option::Option<EpochAndRound>,
}
impl ::prost::Name for EventNewTransmission {
    const NAME: &'static str = "EventNewTransmission";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConfigSet {
    /// hash of the config
    #[prost(bytes = "vec", tag = "1")]
    pub config_digest: ::prost::alloc::vec::Vec<u8>,
    /// previous_config_block_number block in which the previous config was set, to
    /// simplify historic analysis
    #[prost(int64, tag = "2")]
    pub previous_config_block_number: i64,
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<FeedConfig>,
    #[prost(message, optional, tag = "4")]
    pub config_info: ::core::option::Option<FeedConfigInfo>,
}
impl ::prost::Name for EventConfigSet {
    const NAME: &'static str = "EventConfigSet";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
/// GenesisState defines the OCR module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to OCR.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// feed_configs stores all of the supported OCR feeds
    #[prost(message, repeated, tag = "2")]
    pub feed_configs: ::prost::alloc::vec::Vec<FeedConfig>,
    /// latest_epoch_and_rounds stores the latest epoch and round for each feedId
    #[prost(message, repeated, tag = "3")]
    pub latest_epoch_and_rounds: ::prost::alloc::vec::Vec<FeedEpochAndRound>,
    /// feed_transmissions stores the last transmission for each feed
    #[prost(message, repeated, tag = "4")]
    pub feed_transmissions: ::prost::alloc::vec::Vec<FeedTransmission>,
    /// latest_aggregator_round_ids stores the latest aggregator round ID for each
    /// feedId
    #[prost(message, repeated, tag = "5")]
    pub latest_aggregator_round_ids: ::prost::alloc::vec::Vec<FeedLatestAggregatorRoundIDs>,
    /// reward_pools stores the reward pools
    #[prost(message, repeated, tag = "6")]
    pub reward_pools: ::prost::alloc::vec::Vec<RewardPool>,
    /// feed_observation_counts stores the feed observation counts
    #[prost(message, repeated, tag = "7")]
    pub feed_observation_counts: ::prost::alloc::vec::Vec<FeedCounts>,
    /// feed_transmission_counts stores the feed transmission counts
    #[prost(message, repeated, tag = "8")]
    pub feed_transmission_counts: ::prost::alloc::vec::Vec<FeedCounts>,
    /// pending_payeeships stores the pending payeeships
    #[prost(message, repeated, tag = "9")]
    pub pending_payeeships: ::prost::alloc::vec::Vec<PendingPayeeship>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedTransmission {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub transmission: ::core::option::Option<Transmission>,
}
impl ::prost::Name for FeedTransmission {
    const NAME: &'static str = "FeedTransmission";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedEpochAndRound {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub epoch_and_round: ::core::option::Option<EpochAndRound>,
}
impl ::prost::Name for FeedEpochAndRound {
    const NAME: &'static str = "FeedEpochAndRound";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedLatestAggregatorRoundIDs {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub aggregator_round_id: u64,
}
impl ::prost::Name for FeedLatestAggregatorRoundIDs {
    const NAME: &'static str = "FeedLatestAggregatorRoundIDs";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardPool {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for RewardPool {
    const NAME: &'static str = "RewardPool";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedCounts {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub counts: ::prost::alloc::vec::Vec<Count>,
}
impl ::prost::Name for FeedCounts {
    const NAME: &'static str = "FeedCounts";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Count {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub count: u64,
}
impl ::prost::Name for Count {
    const NAME: &'static str = "Count";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingPayeeship {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub transmitter: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub proposed_payee: ::prost::alloc::string::String,
}
impl ::prost::Name for PendingPayeeship {
    const NAME: &'static str = "PendingPayeeship";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeedConfigRequest {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryFeedConfigRequest {
    const NAME: &'static str = "QueryFeedConfigRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeedConfigResponse {
    #[prost(message, optional, tag = "1")]
    pub feed_config_info: ::core::option::Option<FeedConfigInfo>,
    #[prost(message, optional, tag = "2")]
    pub feed_config: ::core::option::Option<FeedConfig>,
}
impl ::prost::Name for QueryFeedConfigResponse {
    const NAME: &'static str = "QueryFeedConfigResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeedConfigInfoRequest {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryFeedConfigInfoRequest {
    const NAME: &'static str = "QueryFeedConfigInfoRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeedConfigInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub feed_config_info: ::core::option::Option<FeedConfigInfo>,
    #[prost(message, optional, tag = "2")]
    pub epoch_and_round: ::core::option::Option<EpochAndRound>,
}
impl ::prost::Name for QueryFeedConfigInfoResponse {
    const NAME: &'static str = "QueryFeedConfigInfoResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestRoundRequest {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLatestRoundRequest {
    const NAME: &'static str = "QueryLatestRoundRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestRoundResponse {
    #[prost(uint64, tag = "1")]
    pub latest_round_id: u64,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Transmission>,
}
impl ::prost::Name for QueryLatestRoundResponse {
    const NAME: &'static str = "QueryLatestRoundResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestTransmissionDetailsRequest {
    #[prost(string, tag = "1")]
    pub feed_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLatestTransmissionDetailsRequest {
    const NAME: &'static str = "QueryLatestTransmissionDetailsRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestTransmissionDetailsResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub config_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub epoch_and_round: ::core::option::Option<EpochAndRound>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<Transmission>,
}
impl ::prost::Name for QueryLatestTransmissionDetailsResponse {
    const NAME: &'static str = "QueryLatestTransmissionDetailsResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwedAmountRequest {
    #[prost(string, tag = "1")]
    pub transmitter: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryOwedAmountRequest {
    const NAME: &'static str = "QueryOwedAmountRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwedAmountResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryOwedAmountResponse {
    const NAME: &'static str = "QueryOwedAmountResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {}
impl ::prost::Name for QueryModuleStateRequest {
    const NAME: &'static str = "QueryModuleStateRequest";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
impl ::prost::Name for QueryModuleStateResponse {
    const NAME: &'static str = "QueryModuleStateResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateFeed {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<FeedConfig>,
}
impl ::prost::Name for MsgCreateFeed {
    const NAME: &'static str = "MsgCreateFeed";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateFeedResponse {}
impl ::prost::Name for MsgCreateFeedResponse {
    const NAME: &'static str = "MsgCreateFeedResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateFeed {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// feed_id is an unique ID for the target of this config
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    /// signers ith element is address ith oracle uses to sign a report
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// transmitters ith element is address ith oracle uses to transmit a report
    /// via the transmit method
    #[prost(string, repeated, tag = "4")]
    pub transmitters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Fixed LINK reward for each observer
    #[prost(string, tag = "5")]
    pub link_per_observation: ::prost::alloc::string::String,
    /// Fixed LINK reward for transmitter
    #[prost(string, tag = "6")]
    pub link_per_transmission: ::prost::alloc::string::String,
    /// Native denom for LINK coin in the bank keeper
    #[prost(string, tag = "7")]
    pub link_denom: ::prost::alloc::string::String,
    /// feed administrator
    #[prost(string, tag = "8")]
    pub feed_admin: ::prost::alloc::string::String,
    /// feed billing administrator
    #[prost(string, tag = "9")]
    pub billing_admin: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUpdateFeed {
    const NAME: &'static str = "MsgUpdateFeed";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateFeedResponse {}
impl ::prost::Name for MsgUpdateFeedResponse {
    const NAME: &'static str = "MsgUpdateFeedResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransmit {
    /// Address of the transmitter
    #[prost(string, tag = "1")]
    pub transmitter: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub config_digest: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub epoch: u64,
    #[prost(uint64, tag = "5")]
    pub round: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub extra_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub report: ::core::option::Option<Report>,
    #[prost(bytes = "vec", repeated, tag = "8")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for MsgTransmit {
    const NAME: &'static str = "MsgTransmit";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransmitResponse {}
impl ::prost::Name for MsgTransmitResponse {
    const NAME: &'static str = "MsgTransmitResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundFeedRewardPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgFundFeedRewardPool {
    const NAME: &'static str = "MsgFundFeedRewardPool";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundFeedRewardPoolResponse {}
impl ::prost::Name for MsgFundFeedRewardPoolResponse {
    const NAME: &'static str = "MsgFundFeedRewardPoolResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawFeedRewardPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgWithdrawFeedRewardPool {
    const NAME: &'static str = "MsgWithdrawFeedRewardPool";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawFeedRewardPoolResponse {}
impl ::prost::Name for MsgWithdrawFeedRewardPoolResponse {
    const NAME: &'static str = "MsgWithdrawFeedRewardPoolResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPayees {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    /// addresses oracles use to transmit the reports
    #[prost(string, repeated, tag = "3")]
    pub transmitters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// addresses of payees corresponding to list of transmitters
    #[prost(string, repeated, tag = "4")]
    pub payees: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSetPayees {
    const NAME: &'static str = "MsgSetPayees";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPayeesResponse {}
impl ::prost::Name for MsgSetPayeesResponse {
    const NAME: &'static str = "MsgSetPayeesResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferPayeeship {
    /// transmitter address of oracle whose payee is changing
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub transmitter: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub feed_id: ::prost::alloc::string::String,
    /// new payee address
    #[prost(string, tag = "4")]
    pub proposed: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgTransferPayeeship {
    const NAME: &'static str = "MsgTransferPayeeship";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferPayeeshipResponse {}
impl ::prost::Name for MsgTransferPayeeshipResponse {
    const NAME: &'static str = "MsgTransferPayeeshipResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptPayeeship {
    /// new payee address
    #[prost(string, tag = "1")]
    pub payee: ::prost::alloc::string::String,
    /// transmitter address of oracle whose payee is changing
    #[prost(string, tag = "2")]
    pub transmitter: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub feed_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgAcceptPayeeship {
    const NAME: &'static str = "MsgAcceptPayeeship";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptPayeeshipResponse {}
impl ::prost::Name for MsgAcceptPayeeshipResponse {
    const NAME: &'static str = "MsgAcceptPayeeshipResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the ocr parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.ocr.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.ocr.v1beta1.{}", Self::NAME)
    }
}
include!("injective.ocr.v1beta1.serde.rs");
include!("injective.ocr.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
