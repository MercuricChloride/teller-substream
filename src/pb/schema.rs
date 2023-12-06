use derive_more::{From, Into};
 use ::substreams_helpers_macros::Map;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct Payment {
    #[prost(string, tag="1")]
    pub principal: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub interest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct Terms {
    #[prost(string, tag="1")]
    pub payment_cycle_amount: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub payment_cycle: u32,
    #[prost(uint32, tag="3")]
    pub apr: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanDetails {
    #[prost(string, tag="1")]
    pub lending_token: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub principal: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub total_repaid: ::core::option::Option<Payment>,
    #[prost(uint32, tag="4")]
    pub timestamp: u32,
    #[prost(uint32, tag="5")]
    pub accepted_timestamp: u32,
    #[prost(uint32, tag="6")]
    pub last_repaid_timestamp: u32,
    #[prost(uint32, tag="7")]
    pub loan_duration: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct Bid {
    #[prost(string, tag="1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub lender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub marketplace_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub metadatauri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub loan_details: ::core::option::Option<LoanDetails>,
    #[prost(message, optional, tag="7")]
    pub terms: ::core::option::Option<Terms>,
    #[prost(enumeration="BidState", tag="8")]
    pub state: i32,
    #[prost(enumeration="PaymentType", tag="9")]
    pub payment_type: i32,
}
/// MARKETPLACE EVENTS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketCreated {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketsCreatedArray {
    #[prost(message, repeated, tag="1")]
    pub markets_created: ::prost::alloc::vec::Vec<MarketCreated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct PaymentCycleDuration {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub duration: u32,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct PaymentCycleDurationArray {
    #[prost(message, repeated, tag="1")]
    pub payment_cycle_durations: ::prost::alloc::vec::Vec<PaymentCycleDuration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct PaymentDefaultDuration {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub duration: u32,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct PaymentDefaultDurationArray {
    #[prost(message, repeated, tag="1")]
    pub payment_default_durations: ::prost::alloc::vec::Vec<PaymentDefaultDuration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BidExpirationTime {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub duration: u32,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BidExpirationTimeArray {
    #[prost(message, repeated, tag="1")]
    pub bid_expiration_times: ::prost::alloc::vec::Vec<BidExpirationTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketFee {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub fee_pct: u32,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketFeeArray {
    #[prost(message, repeated, tag="1")]
    pub market_fees: ::prost::alloc::vec::Vec<MarketFee>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LenderAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LenderAttestationArray {
    #[prost(message, repeated, tag="1")]
    pub lender_attestations: ::prost::alloc::vec::Vec<LenderAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BorrowerAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BorrowerAttestationArray {
    #[prost(message, repeated, tag="1")]
    pub borrower_attestations: ::prost::alloc::vec::Vec<BorrowerAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LenderRevocation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LenderRevocationArray {
    #[prost(message, repeated, tag="1")]
    pub lender_revocations: ::prost::alloc::vec::Vec<LenderRevocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BorrowerRevocation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BorrowerRevocationArray {
    #[prost(message, repeated, tag="1")]
    pub borrower_revocations: ::prost::alloc::vec::Vec<BorrowerRevocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketClosed {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketsClosed {
    #[prost(message, repeated, tag="1")]
    pub markets_closed: ::prost::alloc::vec::Vec<MarketClosed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LenderExitMarket {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LenderExitMarketArray {
    #[prost(message, repeated, tag="1")]
    pub lender_exit_markets: ::prost::alloc::vec::Vec<LenderExitMarket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BorrowerExitMarket {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct BorrowerExitMarketArray {
    #[prost(message, repeated, tag="1")]
    pub borrower_exit_markets: ::prost::alloc::vec::Vec<BorrowerExitMarket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketOwner {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketOwnerArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_owners: ::prost::alloc::vec::Vec<SetMarketOwner>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketFeeRecipient {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_recipient: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketFeeRecipientArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_fee_recipients: ::prost::alloc::vec::Vec<SetMarketFeeRecipient>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketLenderAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub attestation_required: bool,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketLenderAttestationArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_lender_attestations: ::prost::alloc::vec::Vec<SetMarketLenderAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketBorrowerAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub attestation_required: bool,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketBorrowerAttestationArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_borrower_attestations: ::prost::alloc::vec::Vec<SetMarketBorrowerAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketPaymentType {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(enumeration="PaymentType", tag="2")]
    pub payment_type: i32,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketPaymentTypeArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_payment_types: ::prost::alloc::vec::Vec<SetMarketPaymentType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetPaymentCycle {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(enumeration="PaymentCycleType", tag="2")]
    pub payment_cycle_type: i32,
    #[prost(uint32, tag="3")]
    pub value: u32,
    #[prost(message, optional, tag="4")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetPaymentCycleArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_payment_cycles: ::prost::alloc::vec::Vec<SetPaymentCycle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketUri {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SetMarketUriArray {
    #[prost(message, repeated, tag="1")]
    pub set_market_uris: ::prost::alloc::vec::Vec<SetMarketUri>,
}
// END OF MARKETPLACE EVENTS

// TELLER V2 EVENTS

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SubmittedBid {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub metadata_uri: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub lending_token: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub marketplace_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub principal: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub duration: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub apr: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="10")]
    pub collateral: ::prost::alloc::vec::Vec<Collateral>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct Collateral {
    #[prost(string, tag="1")]
    pub collateral_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub collateral_token_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub collateral_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct SubmittedBidArray {
    #[prost(message, repeated, tag="1")]
    pub submitted_bids: ::prost::alloc::vec::Vec<SubmittedBid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct AcceptedBid {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct AcceptedBidArray {
    #[prost(message, repeated, tag="1")]
    pub accepted_bids: ::prost::alloc::vec::Vec<AcceptedBid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct CancelledBid {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct CancelledBidArray {
    #[prost(message, repeated, tag="1")]
    pub cancelled_bids: ::prost::alloc::vec::Vec<CancelledBid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketOwnerCancelledBid {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct MarketOwnerCancelledBidArray {
    #[prost(message, repeated, tag="1")]
    pub market_owner_cancelled_bids: ::prost::alloc::vec::Vec<MarketOwnerCancelledBid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanRepayment {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanRepaymentArray {
    #[prost(message, repeated, tag="1")]
    pub loan_repayments: ::prost::alloc::vec::Vec<LoanRepayment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanRepaid {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanRepaidArray {
    #[prost(message, repeated, tag="1")]
    pub loan_repaid: ::prost::alloc::vec::Vec<LoanRepaid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanLiquidated {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub liquidator: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub transaction: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, From, Into)]
pub struct LoanLiquidationArray {
    #[prost(message, repeated, tag="1")]
    pub loan_liquidations: ::prost::alloc::vec::Vec<LoanLiquidated>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidState {
    Nonexistent = 0,
    Pending = 1,
    Cancelled = 2,
    Accepted = 3,
    Paid = 4,
    Liquidated = 5,
}
impl BidState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BidState::Nonexistent => "NONEXISTENT",
            BidState::Pending => "PENDING",
            BidState::Cancelled => "CANCELLED",
            BidState::Accepted => "ACCEPTED",
            BidState::Paid => "PAID",
            BidState::Liquidated => "LIQUIDATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONEXISTENT" => Some(Self::Nonexistent),
            "PENDING" => Some(Self::Pending),
            "CANCELLED" => Some(Self::Cancelled),
            "ACCEPTED" => Some(Self::Accepted),
            "PAID" => Some(Self::Paid),
            "LIQUIDATED" => Some(Self::Liquidated),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentType {
    Emi = 0,
    Bullet = 1,
}
impl PaymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentType::Emi => "EMI",
            PaymentType::Bullet => "BULLET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EMI" => Some(Self::Emi),
            "BULLET" => Some(Self::Bullet),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentCycleType {
    Seconds = 0,
    Monthly = 1,
}
impl PaymentCycleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentCycleType::Seconds => "SECONDS",
            PaymentCycleType::Monthly => "MONTHLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECONDS" => Some(Self::Seconds),
            "MONTHLY" => Some(Self::Monthly),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
