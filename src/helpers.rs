//use crate::ADDRESS;
use substreams::Hex;
use substreams_entity_change::tables;
use substreams_entity_change::tables::Tables;

use crate::pb::schema::*;

pub trait PopulateTables {
    fn handle(&self, tables: &mut Tables);
}

pub fn bootstrap_protocol(tables: &mut Tables) {
    tables
        .create_row("Protocol", "v2")
        .set("activeCommitments", Vec::<String>::new())
        .set("activeRewards", Vec::<String>::new())
        .set_bigint("_durationTotal", &"0".to_string())
        .set_bigint("durationAverage", &"0".to_string());
}

impl PopulateTables for MarketsCreated {
    fn handle(&self, tables: &mut Tables) {
        for market_created in self.markets_created.iter() {
            tables
                .create_row("MarketPlace", &market_created.market_id)
                .set("owner", &market_created.owner)
                .set("isMarketOpen", true);
        }
    }
}

// impl PopulateTables for BidsSubmitted {
//     fn handle(&self, tables: &mut Tables) {
//         let zero = "0".to_string();
//         for bid_submitted in self.bids.iter() {
//             // create the bid with all the values we can fill
//             let bid = tables
//                 .update_row("Bid", &bid_submitted.bid_id)
//                 .set_bigint("bidId", &bid_submitted.bid_id)
//                 .set("borrowedAddress", &bid_submitted.borrower)
//                 .set("borrower", &bid_submitted.borrower)
//                 .set("receiverAddress", &bid_submitted.receiver)
//                 .set("lender", &bid_submitted.lender)
//                 .set("lenderAddress", &bid_submitted.lender)
//                 .set("lendingToken", &bid_submitted.lender)
//                 .set("lendingTokenAddress", &bid_submitted.lender)
//                 .set("metadataUri", &bid_submitted.metadata_uri)
//                 .set("status", "SUBMITTED") // TODO this should be an enum of the proper states
//                 .set("createdAt", &bid_submitted.created_at) //TODO Update these with the correct values
//                 .set("expiresAt", &bid_submitted.expires_at) //TODO Update these with the correct values
//                 .set("updatedAt", &bid_submitted.updated_at) //TODO Update these with the correct values
//                 .set("transactionHash", &bid_submitted.transaction_hash) //TODO Update these with the correct values
//                 .set("isBidActive", true);

//             // set the other bid values to defaults, ie 0, these will be updated later through other events
//             bid.set_bigint("totalRepaidPrincipal", &zero)
//                 .set_bigint("totalRepaidInterest", &zero)
//                 .set_bigint("_lastTotalRepaidAmount", &zero)
//                 .set_bigint("_lastTotalRepaidInterestAmount", &zero)
//                 .set_bigint("paymentDefaultDuration", &zero)
//                 .set("collateralEscrow", "Some Address")
//                 .set("collateral", Vec::<String>::new());

//             // TODO Create the accounts

//             // TODO Create the Lending Tokens
//         }
//     }
// }

// pub fn transfers_to_table_changes(tables: &mut Tables, transfers: &Transfers) {
//     for transfer in transfers.transfers.iter() {
//         // handle the transfer
//         let key = &transfer.id;
//         let row = tables.update_row("Transfer", key);
//         row.set("from", &transfer.from);
//         row.set("to", &transfer.to);
//         row.set("operator", &transfer.operator);
//         row.set("tokenIds", &transfer.token_ids);
//         if &transfer.token_ids.len() == &1 {
//             row.set("transferType", "SINGLE");
//         } else {
//             row.set("transferType", "BATCH");
//         }

//         // handle the accounts
//         tables.create_row("Account", &transfer.from);
//         tables.create_row("Account", &transfer.to);

//         // handle updating the token owner
//         for token in transfer.token_ids.iter() {
//             tables
//                 .update_row("Token", token)
//                 .set("collection", ADDRESS.to_string())
//                 .set("owner", &transfer.to);
//         }
//     }
// }

// pub fn approvals_to_table_changes(tables: &mut Tables, approvals: &Approvals) {
//     for approval in approvals.approvals.iter() {
//         // handle the approval
//         let key = &approval.id;
//         let row = tables.update_row("ApprovalForAll", key);
//         row.set("owner", &approval.owner);
//         row.set("operator", &approval.operator);
//         row.set("approved", &approval.approved);

//         // handle creation of accounts
//         tables.create_row("Account", &approval.owner);
//         tables.create_row("Account", &approval.operator);
//     }
// }

// pub fn uris_to_table_changes(tables: &mut Tables, uris: &Uris) {
//     for uri in uris.uris.iter() {
//         // handle the uri
//         let row = tables.update_row("Token", &uri.token_id);
//         row.set("uri", &uri.value);
//     }
// }

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}
