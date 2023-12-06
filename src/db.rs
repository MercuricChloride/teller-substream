use crate::{
    pb::schema::{LoanDetails, TxMeta},
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

pub fn handle_submitted_bids(bids: BidArray, tx_meta: TxMeta, t: &mut Tables) {
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

        // create(
        //     t,
        //     bid_id,
        //     bid_id,
        //     created_at,
        //     expires_at,
        //     transaction_hash,
        //     borrower.as_address(),
        //     borrower,
        //     receiver.as_address(),
        //     &lending_token,
        //     lending_token.as_address(),
        //     marketplace_id,
        //     metadata_u_r_i,
        //     status,
        //     principal,
        //     &accepted_timestamp.to_string(),
        //     &last_repaid_timestamp.to_string(),
        //     &loan_duration.to_string(),
        //     payment_cycle,
        //     payment_cycle_amount,
        //     apr,
        //     marketplace,
        //     total_repaid_principal,
        //     total_repaid_interest,
        //     last_total_repaid_amount,
        //     last_total_repaid_interest_amount,
        //     payment_default_duration,
        // )
    }
}
