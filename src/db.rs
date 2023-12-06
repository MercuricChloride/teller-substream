use crate::{
    pb::schema::{LoanDetails, Terms, TxMeta},
    BidArray,
};
use substreams::hex;
use substreams_entity_change::tables::Tables;
use substreams_helpers_macros::helpers_from_graphql;

use crate::pb::schema::Bid;

pub trait DbChanges {
    fn db_changes(&self, t: &mut Tables);
}

impl DbChanges for Bid {
    fn db_changes(&self, t: &mut Tables) {}
}

pub trait AsAddress {
    fn as_address(&self) -> Vec<u8>;
}

impl AsAddress for String {
    fn as_address(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

helpers_from_graphql!(
    "

type Bid @entity {
  id: ID!
  bidId: BigInt!
  createdAt: BigInt!
  expiresAt: BigInt!
  updatedAt: BigInt
  transactionHash: String!
  borrowerAddress: Bytes!
  borrower: Borrower!

  receiverAddress: Bytes!

  liquidatorAddress: Bytes

  lenderAddress: Bytes
  lender: Lender

  lendingToken: Token!
  lendingTokenAddress: Bytes!

  marketplaceId: BigInt!
  metadataURI: String!
  status: String!
  principal: BigInt!
  acceptedTimestamp: BigInt!
  lastRepaidTimestamp: BigInt!
  loanDuration: BigInt!
  paymentCycle: BigInt!
  paymentCycleAmount: BigInt!
  apr: BigInt!
  endDate: BigInt
  nextDueDate: BigInt
  marketplace: MarketPlace!
  commitment: Commitment
  commitmentId: String

  totalRepaidPrincipal: BigInt!
  totalRepaidInterest: BigInt!
  _lastTotalRepaidAmount: BigInt!
  _lastTotalRepaidInterestAmount: BigInt!
  paymentDefaultDuration: BigInt!
  collateralEscrow: Bytes
  collateral: [BidCollateral!]
}
"
);

pub fn handle_submitted_bids(bids: BidArray, t: &mut Tables) {
    use BidHelpers::create;

    for bid in bids.elements {
        let Bid {
            borrower,
            receiver,
            lender,
            marketplace_id,
            metadatauri,
            loan_details,
            terms,
            state,
            payment_type,
            bid_id,
            tx_meta,
        } = &bid;

        let tx_meta = tx_meta
            .as_ref()
            .expect("tx_meta empty, this should never happen!");

        let LoanDetails {
            lending_token,
            principal,
            total_repaid,
            timestamp,
            accepted_timestamp,
            last_repaid_timestamp,
            loan_duration,
        } = loan_details
            .as_ref()
            .expect("Loan Details Empty, is this supposed to happen?");

        let total_repaid = total_repaid
            .as_ref()
            .expect("Loan Details Payment empty, should this happen?");

        let Terms {
            payment_cycle_amount,
            payment_cycle,
            apr,
        } = terms.as_ref().expect("Terms empty, should this happen?");

        let (payment_cycle_amount, payment_cycle, apr) = (
            payment_cycle_amount.to_string(),
            payment_cycle.to_string(),
            apr.to_string(),
        );

        let created_at = tx_meta.timestamp.to_string();
        let expires_at = (tx_meta.timestamp + *loan_duration as u64).to_string();
        let transaction_hash = &tx_meta.hash;

        create(
            t,
            bid_id,
            bid_id,
            &created_at,
            &expires_at,
            transaction_hash,
            &borrower.as_address(),
            borrower,
            &receiver.as_address(),
            &lending_token,
            &lending_token.as_address(),
            marketplace_id,
            metadatauri,
            &state.to_string(),
            principal,
            &accepted_timestamp.to_string(),
            &last_repaid_timestamp.to_string(),
            &loan_duration.to_string(),
            &payment_cycle,
            &payment_cycle_amount,
            &apr,
            marketplace_id,
            &total_repaid.principal,
            &total_repaid.interest,
            &total_repaid.principal,
            &total_repaid.interest,
            &expires_at, // TODO Check if the payment default duration is the expiration time?
        );
    }
}
