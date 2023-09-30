#[path = "./abi/TellerV2.rs"]
mod teller_v2;

#[path = "./abi/MarketRegistry.rs"]
pub mod market_registry;

mod constants;
mod helpers;
pub mod macros;
mod pb;

use pb::schema::*;
use substreams::log::println;
use substreams::pb::substreams::Clock;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth::v2::TransactionTrace;
use substreams_ethereum::Function;
use substreams_ethereum::{pb::eth, Event};

use helpers::*;

use market_registry::events::{
    BorrowerAttestation as BorrowerAttestationEvent, BorrowerExitMarket as BorrowerExitMarketEvent,
    BorrowerRevocation as BorrowerRevocationEvent, LenderAttestation as LenderAttestationEvent,
    LenderExitMarket as LenderExitMarketEvent, LenderRevocation as LenderRevocationEvent,
    MarketClosed as MarketClosedEvent, MarketCreated as MarketCreatedEvent, SetBidExpirationTime,
    SetMarketBorrowerAttestation as SetMarketBorrowerAttestationEvent,
    SetMarketFee as SetMarketFeeEvent, SetMarketFeeRecipient as SetMarketFeeRecipientEvent,
    SetMarketLenderAttestation as SetMarketLenderAttestationEvent,
    SetMarketOwner as SetMarketOwnerEvent, SetMarketPaymentType as SetMarketPaymentTypeEvent,
    SetMarketUri as SetMarketUriEvent, SetPaymentCycle as SetPaymentCycleEvent,
    SetPaymentCycleDuration as SetPaymentCycleDurationEvent,
};

use teller_v2::events::{
    AcceptedBid as AcceptedBidEvent, CancelledBid as CancelledBidEvent,
    LoanLiquidated as LoanLiquidatedEvent, LoanRepaid as LoanRepaidEvent,
    LoanRepayment as LoanRepaymentEvent, MarketOwnerCancelledBid as MarketOwnerCancelledBidEvent,
    SubmittedBid as SubmittedBidEvent,
};

use teller_v2::functions::{SubmitBid1, SubmitBid2};

use constants::{MARKET_REGISTRY, START_BLOCK, TELLER_V2};

#[derive(Debug)]
enum SubmitBid {
    V1(SubmitBid1),
    V2(SubmitBid2),
}

impl SubmitBid {
    pub fn lending_token(&self) -> String {
        match self {
            SubmitBid::V1(function) => format_hex(&function.lending_token),
            SubmitBid::V2(function) => format_hex(&function.lending_token),
        }
    }

    pub fn marketplace_id(&self) -> String {
        match self {
            SubmitBid::V1(function) => function.marketplace_id.to_string(),
            SubmitBid::V2(function) => function.marketplace_id.to_string(),
        }
    }

    pub fn principal(&self) -> String {
        match self {
            SubmitBid::V1(function) => function.principal.to_string(),
            SubmitBid::V2(function) => function.principal.to_string(),
        }
    }

    pub fn duration(&self) -> String {
        match self {
            SubmitBid::V1(function) => function.duration.to_string(),
            SubmitBid::V2(function) => function.duration.to_string(),
        }
    }

    pub fn apr(&self) -> String {
        match self {
            SubmitBid::V1(function) => function.apr.to_string(),
            SubmitBid::V2(function) => function.apr.to_string(),
        }
    }

    pub fn collateral_info(&self) -> Vec<Collateral> {
        match self {
            SubmitBid::V1(_) => Vec::new(),
            SubmitBid::V2(function) => function
                .collateral_info
                .iter()
                .map(
                    |(collateral_type, collateral_amount, token_id, token_address)| Collateral {
                        collateral_type: collateral_type.to_string(),
                        collateral_amount: collateral_amount.to_string(),
                        collateral_token_id: token_id.to_string(),
                        collateral_address: format_hex(token_address),
                    },
                )
                .collect(),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TELLER V2 EVENTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[substreams::handlers::map]
fn map_submitted_bid(block: eth::v2::Block) -> Result<SubmittedBids, substreams::errors::Error> {
    let submitted_bids: Vec<SubmittedBid> = block
        .transaction_traces
        .into_iter()
        .flat_map(|traces| {
            let logs_with_calls = traces.logs_with_calls();
            logs_with_calls
                .filter_map(|(log, call)| {
                    let mut function_call = None;
                    if let Some(function) = SubmitBid1::match_and_decode(call) {
                        function_call = Some(SubmitBid::V1(function));
                    } else if let Some(function) = SubmitBid2::match_and_decode(call) {
                        function_call = Some(SubmitBid::V2(function));
                    }

                    let event = SubmittedBidEvent::match_and_decode(log);

                    if format_hex(&log.address) == TELLER_V2.to_lowercase() {
                        if let (Some(event), Some(function_call)) = (event, function_call) {
                            println(format!("event: {:?}", event));
                            println(format!("function_call: {:?}", function_call));
                            Some((event, function_call))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<(SubmittedBidEvent, SubmitBid)>>()
        })
        .map(|(bid, function_call)| SubmittedBid {
            bid_id: bid.bid_id.to_string(),
            borrower: format_hex(&bid.borrower),
            receiver: format_hex(&bid.receiver),
            metadata_uri: format_hex(&bid.metadata_uri),
            lending_token: function_call.lending_token(),
            marketplace_id: function_call.marketplace_id(),
            principal: function_call.principal(),
            duration: function_call.duration(),
            apr: function_call.apr(),
            collateral: function_call.collateral_info(),
        })
        .collect();

    Ok(SubmittedBids { submitted_bids })
}

// extract the accepted bid event
map_event_to_proto!(
    map_accepted_bid, // function name
    AcceptedBidEvent, // event type
    AcceptedBid,      // singular proto type
    AcceptedBids,     // plural proto type
    accepted_bids,    // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| AcceptedBid {
        bid_id: bid.bid_id.to_string(),
        lender: format_hex(&bid.lender),
    }  // closure to map the event to the proto type
);

// extract the cancelled bid event
map_event_to_proto!(
    map_cancelled_bid, // function name
    CancelledBidEvent, // event type
    CancelledBid,      // singular proto type
    CancelledBids,     // plural proto type
    cancelled_bids,    // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| CancelledBid {
        bid_id: bid.bid_id.to_string(),
    }  // closure to map the event to the proto type
);

// extract the market owner cancelled bid event
map_event_to_proto!(
    map_market_owner_cancelled_bid, // function name
    MarketOwnerCancelledBidEvent,   // event type
    MarketOwnerCancelledBid,        // singular proto type
    MarketOwnerCancelledBids,       // plural proto type
    market_owner_cancelled_bids, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| MarketOwnerCancelledBid {
        bid_id: bid.bid_id.to_string(),
    }  // closure to map the event to the proto type
);

// extract the loan repayment event
map_event_to_proto!(
    map_loan_repayment, // function name
    LoanRepaymentEvent, // event type
    LoanRepayment,      // singular proto type
    LoanRepayments,     // plural proto type
    loan_repayments,    // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| LoanRepayment {
        bid_id: bid.bid_id.to_string(),
    }  // closure to map the event to the proto type
);

// extract the loan repaid event
map_event_to_proto!(
    map_loan_repaid, // function name
    LoanRepaidEvent, // event type
    LoanRepaid,      // singular proto type
    LoanRepaids,     // plural proto type
    loan_repaid,     // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| LoanRepaid {
        bid_id: bid.bid_id.to_string(),
    }  // closure to map the event to the proto type
);

// extract the loan liquidated event
map_event_to_proto!(
    map_loan_liquidated, // function name
    LoanLiquidatedEvent, // event type
    LoanLiquidated,      // singular proto type
    LoanLiquidations,    // plural proto type
    loan_liquidations,   // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| LoanLiquidated {
        bid_id: bid.bid_id.to_string(),
        liquidator: format_hex(&bid.liquidator),
    }  // closure to map the event to the proto type
);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MARKET REGISTRY EVENTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// extract the markets created event
map_event_to_proto!(
    map_markets_created, // function name
    MarketCreatedEvent,  // event type
    MarketCreated,       // singular proto type
    MarketsCreated,      // plural proto type
    markets_created,     // plural proto ident (also the name of the field in the plural proto type)
    |(market, _)| MarketCreated {
        market_id: market.market_id.to_string(),
        owner: format_hex(&market.owner),
    }  // closure to map the event to the proto type
);

// extract the payment cycle durations event
map_event_to_proto!(
    map_payment_cycle_durations,  // function name
    SetPaymentCycleDurationEvent, // event type
    PaymentCycleDuration,         // singular proto type
    PaymentCycleDurations,        // plural proto type
    payment_cycle_durations, // plural proto ident (also the name of the field in the plural proto type)
    |(payment_cycle, _)| PaymentCycleDuration {
        market_id: payment_cycle.market_id.to_string(),
        duration: payment_cycle.duration.into(),
    }  // closure to map the event to the proto type
);

// extract the default payment cycle durations event
map_event_to_proto!(
    map_default_payment_cycle_durations, // function name
    SetPaymentCycleDurationEvent,        // event type
    PaymentDefaultDuration,              // singular proto type
    PaymentDefaultDurations,             // plural proto type
    payment_default_durations, // plural proto ident (also the name of the field in the plural proto type)
    |(payment_cycle, _)| PaymentDefaultDuration {
        market_id: payment_cycle.market_id.to_string(),
        duration: payment_cycle.duration.into(),
    }  // closure to map the event to the proto type
);

// extract the bid expiration time event
map_event_to_proto!(
    map_bid_expiration_time, // function name
    SetBidExpirationTime,    // event type
    BidExpirationTime,       // singular proto type
    BidExpirationTimes,      // plural proto type
    bid_expiration_times, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| BidExpirationTime {
        market_id: bid.market_id.to_string(),
        duration: bid.duration.into(),
    }  // closure to map the event to the proto type
);

// extract the market fee event
map_event_to_proto!(
    map_market_fee,    // function name
    SetMarketFeeEvent, // event type
    MarketFee,         // singular proto type
    MarketFees,        // plural proto type
    market_fees,       // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| MarketFee {
        market_id: bid.market_id.to_string(),
        fee_pct: bid.fee_pct.into(),
    }  // closure to map the event to the proto type
);

// extract the lender attestations event
map_event_to_proto!(
    map_lender_attestations, // function name
    LenderAttestationEvent,  // event type
    LenderAttestation,       // singular proto type
    LenderAttestations,      // plural proto type
    lender_attestations, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| LenderAttestation {
        market_id: bid.market_id.to_string(),
        lender: format_hex(&bid.lender),
    }  // closure to map the event to the proto type
);

// extract the borrower attestation event
map_event_to_proto!(
    map_borrower_attestations, // function name
    BorrowerAttestationEvent,  // event type
    BorrowerAttestation,       // singular proto type
    BorrowerAttestations,      // plural proto type
    borrower_attestations, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| BorrowerAttestation {
        market_id: bid.market_id.to_string(),
        borrower: format_hex(&bid.borrower),
    }  // closure to map the event to the proto type
);

// extract the lender revocation event
map_event_to_proto!(
    map_lender_revocations, // function name
    LenderRevocationEvent,  // event type
    LenderRevocation,       // singular proto type
    LenderRevocations,      // plural proto type
    lender_revocations, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| LenderRevocation {
        market_id: bid.market_id.to_string(),
        lender: format_hex(&bid.lender),
    }  // closure to map the event to the proto type
);

// extract the borrower revocation event
map_event_to_proto!(
    map_borrower_revocations, // function name
    BorrowerRevocationEvent,  // event type
    BorrowerRevocation,       // singular proto type
    BorrowerRevocations,      // plural proto type
    borrower_revocations, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| BorrowerRevocation {
        market_id: bid.market_id.to_string(),
        borrower: format_hex(&bid.borrower),
    }  // closure to map the event to the proto type
);

// extract the market closed event
map_event_to_proto!(
    map_market_closed, // function name
    MarketClosedEvent, // event type
    MarketClosed,      // singular proto type
    MarketsClosed,     // plural proto type
    markets_closed,    // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| MarketClosed {
        market_id: bid.market_id.to_string(),
    }  // closure to map the event to the proto type
);

// extract the lender exit market event
map_event_to_proto!(
    map_lender_exit_market, // function name
    LenderExitMarketEvent,  // event type
    LenderExitMarket,       // singular proto type
    LenderExitMarkets,      // plural proto type
    lender_exit_markets, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| LenderExitMarket {
        market_id: bid.market_id.to_string(),
        lender: format_hex(&bid.lender),
    }  // closure to map the event to the proto type
);

// extract the borrower exit market event
map_event_to_proto!(
    map_borrower_exit_market, // function name
    BorrowerExitMarketEvent,  // event type
    BorrowerExitMarket,       // singular proto type
    BorrowerExitMarkets,      // plural proto type
    borrower_exit_markets, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| BorrowerExitMarket {
        market_id: bid.market_id.to_string(),
        borrower: format_hex(&bid.borrower),
    }  // closure to map the event to the proto type
);

// extract the set market owner event
map_event_to_proto!(
    map_set_market_owner, // function name
    SetMarketOwnerEvent,  // event type
    SetMarketOwner,       // singular proto type
    SetMarketOwners,      // plural proto type
    set_market_owners, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetMarketOwner {
        market_id: bid.market_id.to_string(),
        new_owner: format_hex(&bid.new_owner),
    }  // closure to map the event to the proto type
);

// extract the set fee recipient event
map_event_to_proto!(
    map_set_fee_recipient,      // function name
    SetMarketFeeRecipientEvent, // event type
    SetMarketFeeRecipient,      // singular proto type
    SetMarketFeeRecipients,     // plural proto type
    set_market_fee_recipients, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetMarketFeeRecipient {
        market_id: bid.market_id.to_string(),
        new_recipient: format_hex(&bid.new_recipient),
    }  // closure to map the event to the proto type
);

// extract the set market lender attestation event
map_event_to_proto!(
    map_set_market_lender_attestation, // function name
    SetMarketLenderAttestationEvent,   // event type
    SetMarketLenderAttestation,        // singular proto type
    SetMarketLenderAttestations,       // plural proto type
    set_market_lender_attestations, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetMarketLenderAttestation {
        market_id: bid.market_id.to_string(),
        attestation_required: bid.required,
    }  // closure to map the event to the proto type
);

// extract the set market borrower attestation event
map_event_to_proto!(
    map_set_market_borrower_attestation, // function name
    SetMarketBorrowerAttestationEvent,   // event type
    SetMarketBorrowerAttestation,        // singular proto type
    SetMarketBorrowerAttestations,       // plural proto type
    set_market_borrower_attestations, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetMarketBorrowerAttestation {
        market_id: bid.market_id.to_string(),
        attestation_required: bid.required,
    }  // closure to map the event to the proto type
);

// extract the set payment cycle event
map_event_to_proto!(
    map_set_payment_cycle,     // function name
    SetPaymentCycleEvent,      // event type
    SetPaymentCycle,           // singular proto type
    SetPaymentCycles,          // plural proto type
    set_market_payment_cycles, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetPaymentCycle {
        market_id: bid.market_id.to_string(),
        payment_cycle_type: bid.payment_cycle_type.into(),
        value: bid.value.into(),
    }  // closure to map the event to the proto type
);

// extract the set market payment type event
map_event_to_proto!(
    map_set_payment_type,      // function name
    SetMarketPaymentTypeEvent, // event type
    SetMarketPaymentType,      // singular proto type
    SetMarketPaymentTypes,     // plural proto type
    set_market_payment_types, // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetMarketPaymentType {
        market_id: bid.market_id.to_string(),
        payment_type: bid.payment_type.into(),
    }  // closure to map the event to the proto type
);

// extract the set market uri event
map_event_to_proto!(
    map_set_market_uri, // function name
    SetMarketUriEvent,  // event type
    SetMarketUri,       // singular proto type
    SetMarketUris,      // plural proto type
    set_market_uris,    // plural proto ident (also the name of the field in the plural proto type)
    |(bid, _)| SetMarketUri {
        market_id: bid.market_id.to_string(),
        uri: bid.uri,
    }  // closure to map the event to the proto type
);

#[substreams::handlers::map]
pub fn graph_out(
    map_submitted_bid: SubmittedBids,
    map_accepted_bid: AcceptedBids,
    map_cancelled_bid: CancelledBids,
    map_loan_liquidated: LoanLiquidations,
    map_market_owner_cancelled_bid: MarketOwnerCancelledBids,
    map_loan_repayment: LoanRepayments,
    map_loan_repaid: LoanRepaids,
    map_markets_created: MarketsCreated,
    map_payment_cycle_durations: PaymentCycleDurations,
    map_default_payment_cycle_durations: PaymentDefaultDurations,
    map_bid_expiration_time: BidExpirationTimes,
    map_market_fee: MarketFees,
    map_lender_attestations: LenderAttestations,
    map_borrower_attestations: BorrowerAttestations,
    map_lender_revocations: LenderRevocations,
    map_borrower_revocations: BorrowerRevocations,
    map_market_closed: MarketsClosed,
    map_lender_exit_market: LenderExitMarkets,
    map_borrower_exit_market: BorrowerExitMarkets,
    map_set_market_owner: SetMarketOwners,
    map_set_fee_recipient: SetMarketFeeRecipients,
    map_set_market_lender_attestation: SetMarketLenderAttestations,
    map_set_market_borrower_attestation: SetMarketBorrowerAttestations,
    map_set_payment_cycle: SetPaymentCycles,
    map_set_payment_type: SetMarketPaymentTypes,
    map_set_market_uri: SetMarketUris,
    clock: Clock,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    if clock.number == START_BLOCK {
        bootstrap_protocol(&mut tables);
    }

    // handle the markets created event
    map_markets_created.handle(&mut tables);

    Ok(tables.to_entity_changes())
}
