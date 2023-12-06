use crate::constants::TELLER_V2;
use crate::format_hex;
use crate::pb::schema::{Bid, LoanDetails, Payment, Terms};
use crate::teller_v2::functions::Bids;
use std::str::FromStr;
use substreams::prelude::*;
use substreams::scalar::BigInt;

struct TellerV2(Vec<u8>);

type Address = Vec<u8>;
type Bytes32 = [u8; 32];

type LoanDetailsT = (Address, BigInt, PaymentT, BigInt, BigInt, BigInt, BigInt);

type PaymentT = (BigInt, BigInt);

type TermsT = (BigInt, BigInt, BigInt);

type BidStruct = (
    Address,
    Address,
    Address,
    BigInt,
    Bytes32,
    LoanDetailsT,
    TermsT,
    BigInt,
    BigInt,
);

trait FormatSolType {
    fn format_sol_type(self) -> String;
}

impl FormatSolType for Address {
    fn format_sol_type(self) -> String {
        format_hex(&self)
    }
}

impl FormatSolType for Bytes32 {
    fn format_sol_type(self) -> String {
        format_hex(&self)
    }
}

impl FormatSolType for BigInt {
    fn format_sol_type(self) -> String {
        self.to_string()
    }
}

impl From<PaymentT> for Payment {
    fn from(value: PaymentT) -> Self {
        Self {
            principal: value.0.format_sol_type(),
            interest: value.1.format_sol_type(),
        }
    }
}

impl From<TermsT> for Terms {
    fn from(value: TermsT) -> Self {
        Self {
            payment_cycle_amount: value.0.format_sol_type(),
            payment_cycle: value.1.to_u64() as u32,
            apr: value.2.to_i32() as u32,
        }
    }
}

impl From<LoanDetailsT> for LoanDetails {
    fn from(value: LoanDetailsT) -> Self {
        let (
            lending_token,
            principal,
            total_repaid,
            timestamp,
            accepted_timestamp,
            last_repaid_timestamp,
            loan_duration,
        ) = value;

        Self {
            lending_token: lending_token.format_sol_type(),
            principal: principal.format_sol_type(),
            total_repaid: Some(total_repaid.into()),
            timestamp: timestamp.into(),
            accepted_timestamp: accepted_timestamp.into(),
            last_repaid_timestamp: last_repaid_timestamp.into(),
            loan_duration: loan_duration.into(),
        }
    }
}

impl From<BidStruct> for Bid {
    fn from(value: BidStruct) -> Self {
        let (
            borrower,
            receiver,
            lender,
            marketplace_id,
            metadata_uri,
            loan_details,
            terms,
            state,
            payment_type,
        ) = value;

        Self {
            borrower: borrower.format_sol_type(),
            receiver: receiver.format_sol_type(),
            lender: lender.format_sol_type(),
            marketplace_id: marketplace_id.format_sol_type(),
            metadatauri: metadata_uri.format_sol_type(),
            loan_details: Some(loan_details.into()),
            terms: Some(terms.into()),
            state: state.into(),
            payment_type: payment_type.into(),
        }
    }
}

impl TellerV2 {
    pub fn bids(&self, bid_id: &String) -> Bid {
        let bid_id = BigInt::from_str(bid_id.as_str()).expect(&format!(
            "Couldn't convert the bid_id: {} into BigInt!",
            bid_id
        ));

        let bids = Bids { param0: bid_id };

        let bid: Bid = bids.call(TELLER_V2.to_vec()).unwrap().into();

        bid
    }
}
