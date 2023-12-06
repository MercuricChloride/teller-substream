//pub mod abi;
mod constants;
//mod helpers;
pub mod contracts;
pub mod db;
pub mod macros;
mod pb;

use contracts::TellerV2;
use db::handle_submitted_bids;
//use abi::teller_v2::functions::{SubmitBid1, SubmitBid2};
//use helpers::*;
use pb::schema::*;
use prost::Message;
use prost_types::Any;
use substreams::log::println;
use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::Hex;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::TransactionTrace;
use substreams_ethereum::Function;
use substreams_ethereum::{pb::eth, Event};

use substreams_ethereum::use_contract;

use constants::{MARKET_REGISTRY, START_BLOCK, TELLER_V2, TRANSACTION_TYPE_URL};
use substreams_helpers_traits::{FromBlock, Map, Store, StringW, TimeStore};
fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}

use_contract!(teller_v2, "abis/TellerV2.json");

use teller_v2::events;
use teller_v2::functions::{SubmitBid1, SubmitBid2};

//use entities::schema::*;

//pub mod events {
//pub use crate::abi::market_registry::events::*;
//pub use crate::abi::teller_v2::events::*;
//}

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
            SubmitBid::V1(transaction) => Vec::new(),
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

// //pub fn bootstrap_protocol(tables: &mut Tables) {
// //     let zero = String::from("0");

// //     protocol::create(
// //         tables,
// //         &"v2".to_string(),
// //         &Vec::new(),
// //         &Vec::new(),
// //         &zero,
// //         &zero,
// //     );
// // }

// // impl FromEvent<MarketCreatedEvent> for MarketCreated {
// //     fn from_event(event: MarketCreatedEvent) -> Option<Self>
// //     where
// //         Self: Sized,
// //     {
// //         let (a, b) = event.into();
// //         let (a, b): (StringW, StringW) = (a.into(), b.into());
// //         let (a, b) = (a.into(), b.into());
// //         Some((a, b).into())
// //     }
// // }

// // impl From<MarketCreatedEvent> for MarketCreated {
// //     fn from(value: MarketCreatedEvent) -> Self {
// //         let (a, b) = value.into();
// //         let (a, b): (StringW, StringW) = (a.into(), b.into());
// //         let (a, b) = (a.into(), b.into());
// //         (a, b).into()
// //     }
// // }

// //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// //TELLER V2 EVENTS
// //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// // impl FromEvent<events::Market> for MarketCreated {
// //     fn from_event(event: MarketCreatedEvent) -> Option<Self>
// //     where
// //         Self: Sized,
// //     {
// //         let (a, b) = event.into();
// //         let (a, b): (StringW, StringW) = (a.into(), b.into());
// //         let (a, b) = (a.into(), b.into());
// //         Some((a, b).into())
// //}
// //}

// //impl Map<MarketCreatedEvent, MarketCreated> for MarketsCreated {}

// //#[substreams::handlers::map]
// //fn map_markets(block: eth::v2::Block) -> Result<MarketsCreated, substreams::errors::Error> {
// //MarketsCreated::map(block)
// //}

fn create_tx_meta(log: LogView<'_>, block: &eth::v2::Block) -> TxMeta {
    TxMeta {
        hash: format_hex(&log.receipt.transaction.hash),
        block_number: block.number,
        timestamp: block.timestamp_seconds(),
    }
}

#[substreams::handlers::store]
fn store_submitted_bids(block: eth::v2::Block, s: StoreSetProto<Bid>) {
    block
        .events::<teller_v2::events::SubmittedBid>(&[&constants::TELLER_V2])
        .map(|(e, log)| (e.bid_id.to_string(), log))
        .for_each(|(id, log)| {
            let tx_meta = create_tx_meta(log, &block);
            let bid = TellerV2::bids(&id, tx_meta);
            if let Some(bid) = bid {
                s.set(0, id, &bid)
            }
        });
}

#[substreams::handlers::map]
fn submitted_bids(block: eth::v2::Block) -> BidArray {
    let bids = block
        .events::<teller_v2::events::SubmittedBid>(&[&constants::TELLER_V2])
        .map(|(e, log)| (e.bid_id.to_string(), log))
        .filter_map(|(id, log)| {
            let tx_meta = create_tx_meta(log, &block);
            TellerV2::bids(&id, tx_meta)
        })
        .collect::<Vec<_>>();

    BidArray { elements: bids }
}

#[substreams::handlers::map]
fn map_submitted_bid(
    block: eth::v2::Block,
) -> Result<SubmittedBidArray, substreams::errors::Error> {
    let submitted_bids: Vec<SubmittedBid> = block
        .events::<events::SubmittedBid>(&[&constants::TELLER_V2])
        .filter_map(|(event, log)| {
            let mut function_call = None;
            for call in log.receipt.transaction.calls() {
                if let Some(function) = SubmitBid1::match_and_decode(call) {
                    function_call = Some(SubmitBid::V1(function));
                } else if let Some(function) = SubmitBid2::match_and_decode(call) {
                    function_call = Some(SubmitBid::V2(function));
                }
            }

            if let Some(function_call) = function_call {
                return Some((event, function_call, log));
            }
            None
        })
        .map(|(bid, function_call, transaction)| SubmittedBid {
            bid_id: bid.bid_id.into(),
            borrower: format_hex(&bid.borrower),
            receiver: format_hex(&bid.receiver),
            metadata_uri: format_hex(&bid.metadata_uri),
            lending_token: function_call.lending_token().into(),
            marketplace_id: function_call.marketplace_id(),
            principal: function_call.principal(),
            duration: function_call.duration(),
            apr: function_call.apr(),
            collateral: function_call.collateral_info(),
            // transaction: Some(Any {
            //     type_url: TRANSACTION_TYPE_URL.to_string(),
            //     value: transaction.log.encode_to_vec(),
            // })
        })
        .collect();

    Ok(SubmittedBidArray { submitted_bids })
}

// //extract the accepted bid event
// map_event_to_proto!(
//     map_accepted_bid,    // function name
//     events::AcceptedBid, // event type
//     AcceptedBid,         // singular proto type
//     AcceptedBids,        // plural proto type
//     accepted_bids,       // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| AcceptedBid {
//         bid_id: bid.bid_id.to_string(),
//         lender: format_hex(&bid.lender),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the cancelled bid event
// map_event_to_proto!(
//     map_cancelled_bid, // function name
//     CancelledBidEvent, // event type
//     CancelledBid,      // singular proto type
//     CancelledBids,     // plural proto type
//     cancelled_bids,    // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| CancelledBid {
//         bid_id: bid.bid_id.to_string(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the market owner cancelled bid event
// map_event_to_proto!(
//     map_market_owner_cancelled_bid, // function name
//     MarketOwnerCancelledBidEvent,   // event type
//     MarketOwnerCancelledBid,        // singular proto type
//     MarketOwnerCancelledBids,       // plural proto type
//     market_owner_cancelled_bids, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| MarketOwnerCancelledBid {
//         bid_id: bid.bid_id.to_string(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the loan repayment event
// map_event_to_proto!(
//     map_loan_repayment, // function name
//     LoanRepaymentEvent, // event type
//     LoanRepayment,      // singular proto type
//     LoanRepayments,     // plural proto type
//     loan_repayments,    // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| LoanRepayment {
//         bid_id: bid.bid_id.to_string(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the loan repaid event
// map_event_to_proto!(
//     map_loan_repaid, // function name
//     LoanRepaidEvent, // event type
//     LoanRepaid,      // singular proto type
//     LoanRepaids,     // plural proto type
//     loan_repaid,     // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| LoanRepaid {
//         bid_id: bid.bid_id.to_string(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the loan liquidated event
// map_event_to_proto!(
//     map_loan_liquidated, // function name
//     LoanLiquidatedEvent, // event type
//     LoanLiquidated,      // singular proto type
//     LoanLiquidations,    // plural proto type
//     loan_liquidations,   // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| LoanLiquidated {
//         bid_id: bid.bid_id.to_string(),
//         liquidator: format_hex(&bid.liquidator),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// // MARKET REGISTRY EVENTS
// // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// // extract the markets created event
// // map_event_to_proto!(
// //     map_markets_created, // function name
// //     MarketCreatedEvent,  // event type
// //     MarketCreated,       // singular proto type
// //     MarketsCreated,      // plural proto type
// //     markets_created,     // plural proto ident (also the name of the field in the plural proto type)
// //     |(market, transaction)| MarketCreated {
// //         market_id: market.market_id.to_string(),
// //         owner: format_hex(&market.owner),
// //         //     type_url: TRANSACTION_TYPE_URL.to_string(),
// //         //     value: transaction.encode_to_vec(),
// //         // })
// //     }  // closure to map the event to the proto type
// // );

// // extract the payment cycle durations event
// map_event_to_proto!(
//     map_payment_cycle_durations,  // function name
//     SetPaymentCycleDurationEvent, // event type
//     PaymentCycleDuration,         // singular proto type
//     PaymentCycleDurations,        // plural proto type
//     payment_cycle_durations, // plural proto ident (also the name of the field in the plural proto type)
//     |(payment_cycle, transaction)| PaymentCycleDuration {
//         market_id: payment_cycle.market_id.to_string(),
//         duration: payment_cycle.duration.into(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the default payment cycle durations event
// map_event_to_proto!(
//     map_default_payment_cycle_durations, // function name
//     SetPaymentCycleDurationEvent,        // event type
//     PaymentDefaultDuration,              // singular proto type
//     PaymentDefaultDurations,             // plural proto type
//     payment_default_durations, // plural proto ident (also the name of the field in the plural proto type)
//     |(payment_cycle, transaction)| PaymentDefaultDuration {
//         market_id: payment_cycle.market_id.to_string(),
//         duration: payment_cycle.duration.into(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the bid expiration time event
// map_event_to_proto!(
//     map_bid_expiration_time, // function name
//     SetBidExpirationTime,    // event type
//     BidExpirationTime,       // singular proto type
//     BidExpirationTimes,      // plural proto type
//     bid_expiration_times, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| BidExpirationTime {
//         market_id: bid.market_id.to_string(),
//         duration: bid.duration.into(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the market fee event
// map_event_to_proto!(
//     map_market_fee,    // function name
//     SetMarketFeeEvent, // event type
//     MarketFee,         // singular proto type
//     MarketFees,        // plural proto type
//     market_fees,       // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| MarketFee {
//         market_id: bid.market_id.to_string(),
//         fee_pct: bid.fee_pct.into(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the lender attestations event
// map_event_to_proto!(
//     map_lender_attestations, // function name
//     LenderAttestationEvent,  // event type
//     LenderAttestation,       // singular proto type
//     LenderAttestations,      // plural proto type
//     lender_attestations, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| LenderAttestation {
//         market_id: bid.market_id.to_string(),
//         lender: format_hex(&bid.lender),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the borrower attestation event
// map_event_to_proto!(
//     map_borrower_attestations, // function name
//     BorrowerAttestationEvent,  // event type
//     BorrowerAttestation,       // singular proto type
//     BorrowerAttestations,      // plural proto type
//     borrower_attestations, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| BorrowerAttestation {
//         market_id: bid.market_id.to_string(),
//         borrower: format_hex(&bid.borrower),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the lender revocation event
// map_event_to_proto!(
//     map_lender_revocations, // function name
//     LenderRevocationEvent,  // event type
//     LenderRevocation,       // singular proto type
//     LenderRevocations,      // plural proto type
//     lender_revocations, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| LenderRevocation {
//         market_id: bid.market_id.to_string(),
//         lender: format_hex(&bid.lender),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the borrower revocation event
// map_event_to_proto!(
//     map_borrower_revocations, // function name
//     BorrowerRevocationEvent,  // event type
//     BorrowerRevocation,       // singular proto type
//     BorrowerRevocations,      // plural proto type
//     borrower_revocations, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| BorrowerRevocation {
//         market_id: bid.market_id.to_string(),
//         borrower: format_hex(&bid.borrower),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the market closed event
// map_event_to_proto!(
//     map_market_closed, // function name
//     MarketClosedEvent, // event type
//     MarketClosed,      // singular proto type
//     MarketsClosed,     // plural proto type
//     markets_closed,    // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| MarketClosed {
//         market_id: bid.market_id.to_string(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the lender exit market event
// map_event_to_proto!(
//     map_lender_exit_market, // function name
//     LenderExitMarketEvent,  // event type
//     LenderExitMarket,       // singular proto type
//     LenderExitMarkets,      // plural proto type
//     lender_exit_markets, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| LenderExitMarket {
//         market_id: bid.market_id.to_string(),
//         lender: format_hex(&bid.lender),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the borrower exit market event
// map_event_to_proto!(
//     map_borrower_exit_market, // function name
//     BorrowerExitMarketEvent,  // event type
//     BorrowerExitMarket,       // singular proto type
//     BorrowerExitMarkets,      // plural proto type
//     borrower_exit_markets, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| BorrowerExitMarket {
//         market_id: bid.market_id.to_string(),
//         borrower: format_hex(&bid.borrower),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set market owner event
// map_event_to_proto!(
//     map_set_market_owner, // function name
//     SetMarketOwnerEvent,  // event type
//     SetMarketOwner,       // singular proto type
//     SetMarketOwners,      // plural proto type
//     set_market_owners, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetMarketOwner {
//         market_id: bid.market_id.to_string(),
//         new_owner: format_hex(&bid.new_owner),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set fee recipient event
// map_event_to_proto!(
//     map_set_fee_recipient,      // function name
//     SetMarketFeeRecipientEvent, // event type
//     SetMarketFeeRecipient,      // singular proto type
//     SetMarketFeeRecipients,     // plural proto type
//     set_market_fee_recipients, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetMarketFeeRecipient {
//         market_id: bid.market_id.to_string(),
//         new_recipient: format_hex(&bid.new_recipient),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set market lender attestation event
// map_event_to_proto!(
//     map_set_market_lender_attestation, // function name
//     SetMarketLenderAttestationEvent,   // event type
//     SetMarketLenderAttestation,        // singular proto type
//     SetMarketLenderAttestations,       // plural proto type
//     set_market_lender_attestations, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetMarketLenderAttestation {
//         market_id: bid.market_id.to_string(),
//         attestation_required: bid.required,
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set market borrower attestation event
// map_event_to_proto!(
//     map_set_market_borrower_attestation, // function name
//     SetMarketBorrowerAttestationEvent,   // event type
//     SetMarketBorrowerAttestation,        // singular proto type
//     SetMarketBorrowerAttestations,       // plural proto type
//     set_market_borrower_attestations, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetMarketBorrowerAttestation {
//         market_id: bid.market_id.to_string(),
//         attestation_required: bid.required,
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set payment cycle event
// map_event_to_proto!(
//     map_set_payment_cycle,     // function name
//     SetPaymentCycleEvent,      // event type
//     SetPaymentCycle,           // singular proto type
//     SetPaymentCycles,          // plural proto type
//     set_market_payment_cycles, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetPaymentCycle {
//         market_id: bid.market_id.to_string(),
//         payment_cycle_type: bid.payment_cycle_type.into(),
//         value: bid.value.into(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set market payment type event
// map_event_to_proto!(
//     map_set_payment_type,      // function name
//     SetMarketPaymentTypeEvent, // event type
//     SetMarketPaymentType,      // singular proto type
//     SetMarketPaymentTypes,     // plural proto type
//     set_market_payment_types, // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetMarketPaymentType {
//         market_id: bid.market_id.to_string(),
//         payment_type: bid.payment_type.into(),
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

// // extract the set market uri event
// map_event_to_proto!(
//     map_set_market_uri, // function name
//     SetMarketUriEvent,  // event type
//     SetMarketUri,       // singular proto type
//     SetMarketUris,      // plural proto type
//     set_market_uris,    // plural proto ident (also the name of the field in the plural proto type)
//     |(bid, transaction)| SetMarketUri {
//         market_id: bid.market_id.to_string(),
//         uri: bid.uri,
//         transaction: Some(Any {
//             type_url: TRANSACTION_TYPE_URL.to_string(),
//             value: transaction.encode_to_vec(),
//         })
//     }  // closure to map the event to the proto type
// );

#[substreams::handlers::map]
pub fn graph_out(
    submitted_bids: BidArray,
    clock: Clock,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    //if clock.number == START_BLOCK {
    //  bootstrap_protocol(&mut tables);
    //}
    handle_submitted_bids(submitted_bids, &mut tables);

    // create the markets
    //markets::create_market(&mut tables, &map_markets_created);

    // create the bids
    //bids::create_bids(&mut tables, &map_submitted_bid, &clock);

    //bids::update_accepted_bids(&mut tables, &map_accepted_bid);

    //bids::update_cancelled_bids(&mut tables, &map_cancelled_bid);

    Ok(tables.to_entity_changes())
}
