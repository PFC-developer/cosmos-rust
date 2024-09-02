// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// auction_period_duration defines the auction period duration
    #[prost(int64, tag = "1")]
    pub auction_period: i64,
    /// min_next_bid_increment_rate defines the minimum increment rate for new bids
    #[prost(string, tag = "2")]
    pub min_next_bid_increment_rate: ::prost::alloc::string::String,
    /// inj_basket_max_cap defines the maximum cap for INJ contained in an auction basket
    #[prost(string, tag = "3")]
    pub inj_basket_max_cap: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for Bid {
    const NAME: &'static str = "Bid";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag = "1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    pub round: u64,
}
impl ::prost::Name for LastAuctionResult {
    const NAME: &'static str = "LastAuctionResult";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBid {
    /// bidder describes the address of bidder
    #[prost(string, tag = "1")]
    pub bidder: ::prost::alloc::string::String,
    /// amount describes the amount the bidder put on the auction
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    pub round: u64,
}
impl ::prost::Name for EventBid {
    const NAME: &'static str = "EventBid";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuctionResult {
    /// winner describes the address of the winner
    #[prost(string, tag = "1")]
    pub winner: ::prost::alloc::string::String,
    /// amount describes the amount the winner get from the auction
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// round defines the round number of auction
    #[prost(uint64, tag = "3")]
    pub round: u64,
}
impl ::prost::Name for EventAuctionResult {
    const NAME: &'static str = "EventAuctionResult";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAuctionStart {
    /// round defines the round number of auction
    #[prost(uint64, tag = "1")]
    pub round: u64,
    /// ending_timestamp describes auction end time
    #[prost(int64, tag = "2")]
    pub ending_timestamp: i64,
    /// new_basket describes auction module balance at the time of new auction
    /// start
    #[prost(message, repeated, tag = "3")]
    pub new_basket: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for EventAuctionStart {
    const NAME: &'static str = "EventAuctionStart";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// GenesisState defines the auction module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of related to auction.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// current auction round
    #[prost(uint64, tag = "2")]
    pub auction_round: u64,
    /// current highest bid
    #[prost(message, optional, tag = "3")]
    pub highest_bid: ::core::option::Option<Bid>,
    /// auction ending timestamp
    #[prost(int64, tag = "4")]
    pub auction_ending_timestamp: i64,
    /// last auction result
    #[prost(message, optional, tag = "5")]
    pub last_auction_result: ::core::option::Option<LastAuctionResult>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// QueryAuctionParamsRequest is the request type for the Query/AuctionParams RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionParamsRequest {}
impl ::prost::Name for QueryAuctionParamsRequest {
    const NAME: &'static str = "QueryAuctionParamsRequest";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// QueryAuctionParamsRequest is the response type for the Query/AuctionParams
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryAuctionParamsResponse {
    const NAME: &'static str = "QueryAuctionParamsResponse";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// QueryCurrentAuctionBasketRequest is the request type for the
/// Query/CurrentAuctionBasket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentAuctionBasketRequest {}
impl ::prost::Name for QueryCurrentAuctionBasketRequest {
    const NAME: &'static str = "QueryCurrentAuctionBasketRequest";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// QueryCurrentAuctionBasketResponse is the response type for the
/// Query/CurrentAuctionBasket RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentAuctionBasketResponse {
    /// amount describes the amount put on auction
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// auctionRound describes current auction round
    #[prost(uint64, tag = "2")]
    pub auction_round: u64,
    /// auctionClosingTime describes auction close time for the round
    #[prost(int64, tag = "3")]
    pub auction_closing_time: i64,
    /// highestBidder describes highest bidder on current round
    #[prost(string, tag = "4")]
    pub highest_bidder: ::prost::alloc::string::String,
    /// highestBidAmount describes highest bid amount on current round
    #[prost(string, tag = "5")]
    pub highest_bid_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryCurrentAuctionBasketResponse {
    const NAME: &'static str = "QueryCurrentAuctionBasketResponse";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateRequest is the request type for the Query/AuctionModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {}
impl ::prost::Name for QueryModuleStateRequest {
    const NAME: &'static str = "QueryModuleStateRequest";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// QueryModuleStateResponse is the response type for the
/// Query/AuctionModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<GenesisState>,
}
impl ::prost::Name for QueryModuleStateResponse {
    const NAME: &'static str = "QueryModuleStateResponse";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastAuctionResultRequest {}
impl ::prost::Name for QueryLastAuctionResultRequest {
    const NAME: &'static str = "QueryLastAuctionResultRequest";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastAuctionResultResponse {
    #[prost(message, optional, tag = "1")]
    pub last_auction_result: ::core::option::Option<LastAuctionResult>,
}
impl ::prost::Name for QueryLastAuctionResultResponse {
    const NAME: &'static str = "QueryLastAuctionResultResponse";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
/// Bid defines a SDK message for placing a bid for an auction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBid {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// amount of the bid in INJ tokens
    #[prost(message, optional, tag = "2")]
    pub bid_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// the current auction round being bid on
    #[prost(uint64, tag = "3")]
    pub round: u64,
}
impl ::prost::Name for MsgBid {
    const NAME: &'static str = "MsgBid";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBidResponse {}
impl ::prost::Name for MsgBidResponse {
    const NAME: &'static str = "MsgBidResponse";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
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
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "injective.auction.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("injective.auction.v1beta1.{}", Self::NAME)
    }
}
include!("injective.auction.v1beta1.serde.rs");
include!("injective.auction.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
