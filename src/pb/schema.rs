// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidSubmitted {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub metadata_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidsSubmitted {
    #[prost(message, repeated, tag="1")]
    pub bids: ::prost::alloc::vec::Vec<BidSubmitted>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCreated {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketsCreated {
    #[prost(message, repeated, tag="1")]
    pub markets_created: ::prost::alloc::vec::Vec<MarketCreated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentCycleDuration {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub duration: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentCycleDurations {
    #[prost(message, repeated, tag="1")]
    pub payment_cycle_durations: ::prost::alloc::vec::Vec<PaymentCycleDuration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentDefaultDuration {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub duration: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentDefaultDurations {
    #[prost(message, repeated, tag="1")]
    pub payment_default_durations: ::prost::alloc::vec::Vec<PaymentDefaultDuration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidExpirationTime {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub duration: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidExpirationTimes {
    #[prost(message, repeated, tag="1")]
    pub bid_expiration_times: ::prost::alloc::vec::Vec<BidExpirationTime>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFee {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub fee_pct: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFees {
    #[prost(message, repeated, tag="1")]
    pub market_fees: ::prost::alloc::vec::Vec<MarketFee>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LenderAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LenderAttestations {
    #[prost(message, repeated, tag="1")]
    pub lender_attestations: ::prost::alloc::vec::Vec<LenderAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowerAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowerAttestations {
    #[prost(message, repeated, tag="1")]
    pub borrower_attestations: ::prost::alloc::vec::Vec<BorrowerAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LenderRevocation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LenderRevocations {
    #[prost(message, repeated, tag="1")]
    pub lender_revocations: ::prost::alloc::vec::Vec<LenderRevocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowerRevocation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowerRevocations {
    #[prost(message, repeated, tag="1")]
    pub borrower_revocations: ::prost::alloc::vec::Vec<BorrowerRevocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketClosed {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketsClosed {
    #[prost(message, repeated, tag="1")]
    pub markets_closed: ::prost::alloc::vec::Vec<MarketClosed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LenderExitMarket {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LenderExitMarkets {
    #[prost(message, repeated, tag="1")]
    pub lender_exit_markets: ::prost::alloc::vec::Vec<LenderExitMarket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowerExitMarket {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorrowerExitMarkets {
    #[prost(message, repeated, tag="1")]
    pub borrower_exit_markets: ::prost::alloc::vec::Vec<BorrowerExitMarket>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketOwner {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketOwners {
    #[prost(message, repeated, tag="1")]
    pub set_market_owners: ::prost::alloc::vec::Vec<SetMarketOwner>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketFeeRecipient {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_recipient: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketFeeRecipients {
    #[prost(message, repeated, tag="1")]
    pub set_market_fee_recipients: ::prost::alloc::vec::Vec<SetMarketFeeRecipient>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketLenderAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub attestation_required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketLenderAttestations {
    #[prost(message, repeated, tag="1")]
    pub set_market_lender_attestations: ::prost::alloc::vec::Vec<SetMarketLenderAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketBorrowerAttestation {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub attestation_required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketBorrowerAttestations {
    #[prost(message, repeated, tag="1")]
    pub set_market_borrower_attestations: ::prost::alloc::vec::Vec<SetMarketBorrowerAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketPaymentType {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(enumeration="PaymentType", tag="2")]
    pub payment_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketPaymentTypes {
    #[prost(message, repeated, tag="1")]
    pub set_market_payment_types: ::prost::alloc::vec::Vec<SetMarketPaymentType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPaymentCycle {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(enumeration="PaymentCycleType", tag="2")]
    pub payment_cycle_type: i32,
    #[prost(uint32, tag="3")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPaymentCycles {
    #[prost(message, repeated, tag="1")]
    pub set_market_payment_cycles: ::prost::alloc::vec::Vec<SetPaymentCycle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketUri {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMarketUris {
    #[prost(message, repeated, tag="1")]
    pub set_market_uris: ::prost::alloc::vec::Vec<SetMarketUri>,
}
// Below contains things for the subgraph schema
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Protocol {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub active_commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub active_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag="4")]
    pub duration_total: u64,
    #[prost(uint64, tag="5")]
    pub duration_average: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Protocols {
    #[prost(message, repeated, tag="1")]
    pub protocols: ::prost::alloc::vec::Vec<Protocol>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolCollateral {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// This is the id of a collateral token in the collateral_tokens table.
    #[prost(string, tag="2")]
    pub collateral_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPlace {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// This is the id of a marketplace in the marketplaces table.
    #[prost(string, tag="2")]
    pub marketplace_id: ::prost::alloc::string::String,
    /// address
    #[prost(bytes="vec", tag="3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    /// address
    #[prost(bytes="vec", tag="4")]
    pub fee_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub metadata_uri: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub payment_default_duration: u64,
    #[prost(uint64, tag="7")]
    pub payment_cycle_duration: u64,
    #[prost(string, tag="8")]
    pub payment_cycle_type: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub payment_type: ::prost::alloc::string::String,
    #[prost(uint64, tag="10")]
    pub bid_expiration_time: u64,
    #[prost(bool, tag="11")]
    pub borrower_attestation_required: bool,
    #[prost(bool, tag="12")]
    pub lender_attestation_required: bool,
    #[prost(uint64, tag="13")]
    pub marketplace_fee_percent: u64,
    #[prost(uint64, tag="14")]
    pub duration_total: u64,
    #[prost(uint64, tag="15")]
    pub duration_averagre: u64,
    #[prost(uint64, tag="16")]
    pub total_number_of_lenders: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    /// Required fields
    ///
    /// ID of the Bid entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bid_id: u64,
    #[prost(uint64, tag="3")]
    pub created_at: u64,
    #[prost(uint64, tag="4")]
    pub expires_at: u64,
    #[prost(string, tag="5")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower_address: ::prost::alloc::vec::Vec<u8>,
    /// ID of the Borrower entity
    #[prost(string, tag="7")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
    /// ID of the Token entity
    #[prost(string, tag="9")]
    pub lending_token: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub lending_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="11")]
    pub marketplace_id: u64,
    #[prost(string, tag="12")]
    pub metadata_uri: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub status: ::prost::alloc::string::String,
    #[prost(uint64, tag="14")]
    pub principal: u64,
    #[prost(uint64, tag="15")]
    pub accepted_timestamp: u64,
    #[prost(uint64, tag="16")]
    pub last_repaid_timestamp: u64,
    #[prost(uint64, tag="17")]
    pub loan_duration: u64,
    #[prost(uint64, tag="18")]
    pub payment_cycle: u64,
    #[prost(uint64, tag="19")]
    pub payment_cycle_amount: u64,
    #[prost(uint64, tag="20")]
    pub apr: u64,
    #[prost(uint64, tag="21")]
    pub total_repaid_principal: u64,
    #[prost(uint64, tag="22")]
    pub total_repaid_interest: u64,
    #[prost(uint64, tag="23")]
    pub last_total_repaid_amount: u64,
    #[prost(uint64, tag="24")]
    pub last_total_repaid_interest_amount: u64,
    #[prost(uint64, tag="25")]
    pub payment_default_duration: u64,
    /// Optional fields
    #[prost(uint64, tag="26")]
    pub updated_at: u64,
    #[prost(bytes="vec", tag="27")]
    pub liquidator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="28")]
    pub lender_address: ::prost::alloc::vec::Vec<u8>,
    /// ID of the Lender entity
    #[prost(string, tag="29")]
    pub lender: ::prost::alloc::string::String,
    #[prost(uint64, tag="30")]
    pub end_date: u64,
    #[prost(uint64, tag="31")]
    pub next_due_date: u64,
    /// ID of the MarketPlace entity
    #[prost(string, tag="32")]
    pub marketplace: ::prost::alloc::string::String,
    /// ID of the Commitment entity
    #[prost(string, tag="33")]
    pub commitment: ::prost::alloc::string::String,
    #[prost(string, tag="34")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="35")]
    pub collateral_escrow: ::prost::alloc::vec::Vec<u8>,
    /// Repeated fields
    ///
    /// IDs of the Payment entities
    #[prost(string, repeated, tag="36")]
    pub payments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IDs of the BidCollateral entities
    #[prost(string, repeated, tag="37")]
    pub collateral: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bids {
    #[prost(message, repeated, tag="1")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmittedBid {
    #[prost(string, tag="1")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub metadata_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmittedBids {
    #[prost(message, repeated, tag="1")]
    pub bids: ::prost::alloc::vec::Vec<SubmittedBid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidCollateral {
    /// Required fields
    ///
    /// ID of the BidCollateral entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(bytes="vec", tag="3")]
    pub collateral_address: ::prost::alloc::vec::Vec<u8>,
    /// ID of the Token entity
    #[prost(string, tag="4")]
    pub token: ::prost::alloc::string::String,
    /// ID of the Bid entity
    #[prost(string, tag="5")]
    pub bid: ::prost::alloc::string::String,
    /// Optional fields
    #[prost(uint64, tag="6")]
    pub token_id: u64,
    #[prost(string, tag="7")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub status: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub receiver: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    /// Required fields
    ///
    /// ID of the Token entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// Optional fields
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub nft_id: u64,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub decimals: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundedTx {
    /// Required fields
    ///
    /// ID of the FundedTx entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the Bid entity
    #[prost(string, tag="2")]
    pub bid: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    /// Required fields
    ///
    /// ID of the Payment entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the Bid entity
    #[prost(string, tag="2")]
    pub bid: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub principal: u64,
    #[prost(uint64, tag="4")]
    pub interest: u64,
    #[prost(uint64, tag="5")]
    pub payment_date: u64,
    #[prost(uint64, tag="6")]
    pub outstanding_capital: u64,
    #[prost(string, tag="7")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Required fields
    ///
    /// ID of the User entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub first_interaction_date: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lender {
    /// Required fields
    ///
    /// ID of the Lender entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub marketplace_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub lender_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub first_interaction_date: u64,
    #[prost(bool, tag="5")]
    pub is_attested: bool,
    #[prost(uint64, tag="6")]
    pub duration_total: u64,
    #[prost(uint64, tag="7")]
    pub duration_average: u64,
    /// Optional fields
    #[prost(uint64, tag="8")]
    pub attested_timestamp: u64,
    /// ID of the MarketPlace entity
    #[prost(string, tag="9")]
    pub marketplace: ::prost::alloc::string::String,
    /// ID of the User entity
    #[prost(string, tag="10")]
    pub user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Borrower {
    /// Required fields
    ///
    /// ID of the Borrower entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub marketplace_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub borrower_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub first_interaction_date: u64,
    #[prost(bool, tag="5")]
    pub is_attested: bool,
    #[prost(uint64, tag="6")]
    pub duration_total: u64,
    #[prost(uint64, tag="7")]
    pub duration_average: u64,
    /// Optional fields
    #[prost(uint64, tag="8")]
    pub attested_timestamp: u64,
    /// ID of the MarketPlace entity
    #[prost(string, tag="9")]
    pub marketplace: ::prost::alloc::string::String,
    /// ID of the User entity
    #[prost(string, tag="10")]
    pub user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoanStatusCount {
    /// Required fields
    ///
    /// ID of the LoanStatusCount entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_count: u64,
    #[prost(uint64, tag="3")]
    pub submitted_count: u64,
    #[prost(uint64, tag="4")]
    pub expired_count: u64,
    #[prost(uint64, tag="5")]
    pub cancelled_count: u64,
    #[prost(uint64, tag="6")]
    pub accepted_count: u64,
    #[prost(uint64, tag="7")]
    pub due_soon_count: u64,
    #[prost(uint64, tag="8")]
    pub late_count: u64,
    #[prost(uint64, tag="9")]
    pub defaulted_count: u64,
    #[prost(uint64, tag="10")]
    pub repaid_count: u64,
    #[prost(uint64, tag="11")]
    pub liquidated_count: u64,
    /// Entity Relationships
    ///
    /// ID of the Protocol entity
    #[prost(string, tag="12")]
    pub protocol: ::prost::alloc::string::String,
    /// ID of the MarketPlace entity
    #[prost(string, tag="13")]
    pub market: ::prost::alloc::string::String,
    /// ID of the Lender entity
    #[prost(string, tag="14")]
    pub lender: ::prost::alloc::string::String,
    /// ID of the Borrower entity
    #[prost(string, tag="15")]
    pub borrower: ::prost::alloc::string::String,
    /// ID of the TokenVolume entity
    #[prost(string, tag="16")]
    pub token_volume: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenVolume {
    /// Required fields
    ///
    /// ID of the TokenVolume entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the Token entity
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub lending_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub outstanding_capital: u64,
    #[prost(uint64, tag="5")]
    pub total_available: u64,
    #[prost(uint64, tag="6")]
    pub total_loaned: u64,
    #[prost(uint64, tag="7")]
    pub total_active: u64,
    #[prost(uint64, tag="8")]
    pub total_accepted: u64,
    #[prost(uint64, tag="9")]
    pub total_due_soon: u64,
    #[prost(uint64, tag="10")]
    pub total_late: u64,
    #[prost(uint64, tag="11")]
    pub total_defaulted: u64,
    #[prost(uint64, tag="12")]
    pub total_repaid: u64,
    #[prost(uint64, tag="13")]
    pub total_liquidated: u64,
    #[prost(uint64, tag="14")]
    pub loan_accepted_count: u64,
    #[prost(uint64, tag="15")]
    pub loan_average: u64,
    #[prost(uint64, tag="16")]
    pub commission_earned: u64,
    #[prost(uint64, tag="17")]
    pub total_repaid_interest: u64,
    #[prost(uint64, tag="18")]
    pub apr_weighted_total: u64,
    #[prost(uint64, tag="19")]
    pub apr_average: u64,
    #[prost(uint64, tag="20")]
    pub apr_active_weighted_total: u64,
    #[prost(uint64, tag="21")]
    pub apr_active_average: u64,
    #[prost(uint64, tag="22")]
    pub duration_total: u64,
    #[prost(uint64, tag="23")]
    pub duration_average: u64,
    /// Optional fields
    ///
    /// ID of the Token entity, null if there's no collateral
    #[prost(string, tag="24")]
    pub collateral_token: ::prost::alloc::string::String,
    /// Entity Relationships
    ///
    /// ID of the Protocol entity
    #[prost(string, tag="25")]
    pub protocol: ::prost::alloc::string::String,
    /// ID of the MarketPlace entity
    #[prost(string, tag="26")]
    pub market: ::prost::alloc::string::String,
    /// ID of the Lender entity
    #[prost(string, tag="27")]
    pub lender: ::prost::alloc::string::String,
    /// ID of the Borrower entity
    #[prost(string, tag="28")]
    pub borrower: ::prost::alloc::string::String,
    /// ID of the ProtocolCollateral entity
    #[prost(string, tag="29")]
    pub protocol_collateral: ::prost::alloc::string::String,
    /// ID of the TokenVolume entity
    #[prost(string, tag="30")]
    pub linked_parent_token_volume: ::prost::alloc::string::String,
}
/// MarketCommitmentStdDev
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCommitmentStdDev {
    /// ID of the MarketCommitmentStdDev entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the MarketPlace entity
    #[prost(string, tag="2")]
    pub market: ::prost::alloc::string::String,
    /// ID of the Token entity
    #[prost(string, tag="3")]
    pub lending_token: ::prost::alloc::string::String,
    /// ID of the Token entity
    #[prost(string, tag="4")]
    pub collateral_token: ::prost::alloc::string::String,
    /// BigDecimal types are represented as strings for simplicity
    #[prost(string, tag="5")]
    pub max_principal_per_collateral_std_dev: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub max_principal_per_collateral_mean: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub min_apy_std_dev: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub min_apy_mean: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub max_duration_std_dev: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub max_duration_mean: ::prost::alloc::string::String,
}
/// CommitmentZScore
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentZScore {
    /// ID of the CommitmentZScore entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the Commitment entity
    #[prost(string, tag="2")]
    pub commitment: ::prost::alloc::string::String,
    /// BigDecimal is represented as a string for simplicity
    #[prost(string, tag="3")]
    pub z_score: ::prost::alloc::string::String,
}
/// Commitment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitment {
    /// ID of the Commitment entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub created_at: u64,
    #[prost(uint64, tag="3")]
    pub updated_at: u64,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
    /// Offer
    #[prost(uint64, tag="5")]
    pub committed_amount: u64,
    #[prost(uint64, tag="6")]
    pub expiration_timestamp: u64,
    #[prost(uint64, tag="7")]
    pub max_duration: u64,
    #[prost(uint64, tag="8")]
    pub min_apy: u64,
    /// ID of the Token entity
    #[prost(string, tag="9")]
    pub principal_token: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub principal_token_address: ::prost::alloc::vec::Vec<u8>,
    /// Required Collateral
    ///
    /// ID of the Token entity
    #[prost(string, tag="11")]
    pub collateral_token: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="12")]
    pub collateral_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="13")]
    pub collateral_token_type: u64,
    #[prost(uint64, tag="14")]
    pub max_principal_per_collateral_amount: u64,
    /// We use bytes to represent Ethereum addresses
    #[prost(bytes="vec", repeated, tag="15")]
    pub commitment_borrowers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Lender
    ///
    /// ID of the Lender entity
    #[prost(string, tag="16")]
    pub lender: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="17")]
    pub lender_address: ::prost::alloc::vec::Vec<u8>,
    /// Market
    ///
    /// ID of the MarketPlace entity
    #[prost(string, tag="18")]
    pub marketplace: ::prost::alloc::string::String,
    #[prost(uint64, tag="19")]
    pub marketplace_id: u64,
    /// TokenStats
    ///
    /// ID of the TokenVolume entity
    #[prost(string, tag="20")]
    pub token_volume: ::prost::alloc::string::String,
    /// Extra
    #[prost(uint64, tag="21")]
    pub max_principal: u64,
    #[prost(uint64, tag="22")]
    pub accepted_principal: u64,
}
/// CommitmentReward
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentReward {
    /// ID of the CommitmentReward entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub created_at: u64,
    #[prost(uint64, tag="3")]
    pub updated_at: u64,
    /// ID of the RewardAllocation entity
    #[prost(string, tag="4")]
    pub reward: ::prost::alloc::string::String,
    /// ID of the Commitment entity
    #[prost(string, tag="5")]
    pub commitment: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub roi: u64,
    #[prost(uint64, tag="7")]
    pub apy: u64,
}
/// RewardAllocation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardAllocation {
    /// ID of the RewardAllocation entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub created_at: u64,
    #[prost(uint64, tag="3")]
    pub updated_at: u64,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
    /// ID of the User entity
    #[prost(string, tag="5")]
    pub allocator: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub allocator_address: ::prost::alloc::vec::Vec<u8>,
    /// Market
    ///
    /// ID of the MarketPlace entity
    #[prost(string, tag="7")]
    pub marketplace: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub marketplace_id: u64,
    /// TokenStats
    ///
    /// ID of the TokenVolume entity
    #[prost(string, tag="9")]
    pub token_volume: ::prost::alloc::string::String,
    /// ID of the Token entity
    #[prost(string, tag="10")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub reward_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="12")]
    pub reward_token_amount_initial: u64,
    #[prost(uint64, tag="13")]
    pub reward_token_amount_remaining: u64,
    #[prost(bytes="vec", tag="14")]
    pub required_principal_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="15")]
    pub required_collateral_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="16")]
    pub minimum_collateral_per_principal_amount: u64,
    #[prost(uint64, tag="17")]
    pub reward_per_loan_principal_amount: u64,
    #[prost(uint64, tag="18")]
    pub bid_start_time_min: u64,
    #[prost(uint64, tag="19")]
    pub bid_start_time_max: u64,
    #[prost(string, tag="20")]
    pub allocation_strategy: ::prost::alloc::string::String,
}
/// BidReward
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidReward {
    /// ID of the BidReward entity
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub created_at: u64,
    #[prost(uint64, tag="3")]
    pub updated_at: u64,
    /// ID of the RewardAllocation entity
    #[prost(string, tag="4")]
    pub reward: ::prost::alloc::string::String,
    /// ID of the Bid entity
    #[prost(string, tag="5")]
    pub bid: ::prost::alloc::string::String,
    /// ID of the User entity
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub claimed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTransaction {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub event: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub gas_sent: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub block_number: ::prost::alloc::string::String,
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
