#[path = "./abi/TellerV2.rs"]
mod teller_v2;

#[path = "./abi/MarketRegistry.rs"]
pub mod market_registry;

mod constants;
mod helpers;
pub mod macros;
mod pb;

use pb::schema::*;
use substreams::pb::substreams::Clock;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
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

use teller_v2::events::SubmittedBid as SubmittedBidEvent;

use constants::{MARKET_REGISTRY, START_BLOCK, TELLER_V2};

// extract the markets created event
map_event_to_proto!(
    map_markets_created, // function name
    MarketCreatedEvent,  // event type
    MarketCreated,       // singular proto type
    MarketsCreated,      // plural proto type
    markets_created,     // plural proto ident (also the name of the field in the plural proto type)
    |market| MarketCreated {
        market_id: market.market_id.to_string(),
        owner: format_hex(&market.owner),
    }  // closure to map the event to the proto type
);

// extract the bids submitted event
map_event_to_proto!(
    map_bids_submitted, // function name
    SubmittedBidEvent,  // event type
    BidSubmitted,       // singular proto type
    BidsSubmitted,      // plural proto type
    bids,               // plural proto ident (also the name of the field in the plural proto type)
    |bid| BidSubmitted {
        bid_id: bid.bid_id.to_string(),
        borrower: format_hex(&bid.borrower),
        receiver: format_hex(&bid.receiver),
        metadata_uri: format_hex(&bid.metadata_uri),
    }  // closure to map the event to the proto type
);

// extract the payment cycle durations event
map_event_to_proto!(
    map_payment_cycle_durations,  // function name
    SetPaymentCycleDurationEvent, // event type
    PaymentCycleDuration,         // singular proto type
    PaymentCycleDurations,        // plural proto type
    payment_cycle_durations, // plural proto ident (also the name of the field in the plural proto type)
    |payment_cycle| PaymentCycleDuration {
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
    |payment_cycle| PaymentDefaultDuration {
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
    |bid| BidExpirationTime {
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
    |bid| MarketFee {
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
    |bid| LenderAttestation {
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
    |bid| BorrowerAttestation {
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
    |bid| LenderRevocation {
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
    |bid| BorrowerRevocation {
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
    |bid| MarketClosed {
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
    |bid| LenderExitMarket {
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
    |bid| BorrowerExitMarket {
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
    |bid| SetMarketOwner {
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
    |bid| SetMarketFeeRecipient {
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
    |bid| SetMarketLenderAttestation {
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
    |bid| SetMarketBorrowerAttestation {
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
    |bid| SetPaymentCycle {
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
    |bid| SetMarketPaymentType {
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
    |bid| SetMarketUri {
        market_id: bid.market_id.to_string(),
        uri: bid.uri,
    }  // closure to map the event to the proto type
);

#[substreams::handlers::map]
pub fn graph_out(
    markets_created: MarketsCreated,
    bids_submitted: BidsSubmitted,
    clock: Clock,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    if clock.number == START_BLOCK {
        bootstrap_protocol(&mut tables);
    }

    markets_created.handle(&mut tables);

    //bids_submitted.handle(&mut tables);

    Ok(tables.to_entity_changes())
}
