// @generated
/// Params defines the set of params for the distribution module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub community_tax: ::prost::alloc::string::String,
    /// Deprecated: The base_proposer_reward field is deprecated and is no longer used
    /// in the x/distribution module's reward mechanism.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub base_proposer_reward: ::prost::alloc::string::String,
    /// Deprecated: The bonus_proposer_reward field is deprecated and is no longer used
    /// in the x/distribution module's reward mechanism.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub bonus_proposer_reward: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub withdraw_addr_enabled: bool,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// ValidatorHistoricalRewards represents historical rewards for a validator.
/// Height is implicit within the store key.
/// Cumulative reward ratio is the sum from the zeroeth period
/// until this period of rewards / tokens, per the spec.
/// The reference count indicates the number of objects
/// which might need to reference this historical entry at any point.
/// ReferenceCount =
///     number of outstanding delegations which ended the associated period (and
///     might need to read that record)
///   + number of slashes which ended the associated period (and might need to
///   read that record)
///   + one per validator for the zeroeth period, set on initialization
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewards {
    #[prost(message, repeated, tag = "1")]
    pub cumulative_reward_ratio: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint32, tag = "2")]
    pub reference_count: u32,
}
impl ::prost::Name for ValidatorHistoricalRewards {
    const NAME: &'static str = "ValidatorHistoricalRewards";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// ValidatorCurrentRewards represents current rewards and current
/// period for a validator kept as a running counter and incremented
/// each block as long as the validator's tokens remain constant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint64, tag = "2")]
    pub period: u64,
}
impl ::prost::Name for ValidatorCurrentRewards {
    const NAME: &'static str = "ValidatorCurrentRewards";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// ValidatorAccumulatedCommission represents accumulated commission
/// for a validator kept as a running counter, can be withdrawn at any time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommission {
    #[prost(message, repeated, tag = "1")]
    pub commission: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
impl ::prost::Name for ValidatorAccumulatedCommission {
    const NAME: &'static str = "ValidatorAccumulatedCommission";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// ValidatorOutstandingRewards represents outstanding (un-withdrawn) rewards
/// for a validator inexpensive to track, allows simple sanity checks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
impl ::prost::Name for ValidatorOutstandingRewards {
    const NAME: &'static str = "ValidatorOutstandingRewards";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// ValidatorSlashEvent represents a validator slash event.
/// Height is implicit within the store key.
/// This is needed to calculate appropriate amount of staking tokens
/// for delegations which are withdrawn after a slash has occurred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvent {
    #[prost(uint64, tag = "1")]
    pub validator_period: u64,
    #[prost(string, tag = "2")]
    pub fraction: ::prost::alloc::string::String,
}
impl ::prost::Name for ValidatorSlashEvent {
    const NAME: &'static str = "ValidatorSlashEvent";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// ValidatorSlashEvents is a collection of ValidatorSlashEvent messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvents {
    #[prost(message, repeated, tag = "1")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEvent>,
}
impl ::prost::Name for ValidatorSlashEvents {
    const NAME: &'static str = "ValidatorSlashEvents";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// FeePool is the global fee pool for distribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeePool {
    #[prost(message, repeated, tag = "1")]
    pub community_pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
impl ::prost::Name for FeePool {
    const NAME: &'static str = "FeePool";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// CommunityPoolSpendProposal details a proposal for use of community funds,
/// together with how many coins are proposed to be spent, and to which
/// recipient account.
///
/// Deprecated: Do not use. As of the Cosmos SDK release v0.47.x, there is no
/// longer a need for an explicit CommunityPoolSpendProposal. To spend community
/// pool funds, a simple MsgCommunityPoolSpend can be invoked from the x/gov
/// module via a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
impl ::prost::Name for CommunityPoolSpendProposal {
    const NAME: &'static str = "CommunityPoolSpendProposal";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// DelegatorStartingInfo represents the starting info for a delegator reward
/// period. It tracks the previous validator period, the delegation's amount of
/// staking token, and the creation height (to check later on if any slashes have
/// occurred). NOTE: Even though validators are slashed to whole staking tokens,
/// the delegators within the validator may be left with less than a full token,
/// thus sdk.Dec is used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfo {
    #[prost(uint64, tag = "1")]
    pub previous_period: u64,
    #[prost(string, tag = "2")]
    pub stake: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
}
impl ::prost::Name for DelegatorStartingInfo {
    const NAME: &'static str = "DelegatorStartingInfo";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// DelegationDelegatorReward represents the properties
/// of a delegator's delegation reward.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationDelegatorReward {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub reward: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
impl ::prost::Name for DelegationDelegatorReward {
    const NAME: &'static str = "DelegationDelegatorReward";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
/// CommunityPoolSpendProposalWithDeposit defines a CommunityPoolSpendProposal
/// with a deposit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub deposit: ::prost::alloc::string::String,
}
impl ::prost::Name for CommunityPoolSpendProposalWithDeposit {
    const NAME: &'static str = "CommunityPoolSpendProposalWithDeposit";
    const PACKAGE: &'static str = "cosmos.distribution.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.distribution.v1beta1.{}", Self::NAME)
    }
}
include!("cosmos.distribution.v1beta1.serde.rs");
// @@protoc_insertion_point(module)
