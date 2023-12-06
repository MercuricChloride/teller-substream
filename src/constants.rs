// if we aren't building for polygon, we are building for mainnet
#[cfg(not(feature = "polygon"))]

pub const TELLER_V2: [u8; 20] = substreams::hex!("00182fdb0b880ee24d428e3cc39383717677c37e");

#[cfg(feature = "polygon")]

pub const TELLER_V2: &str = "0xD3D79A066F2cD471841C047D372F218252Dbf8Ed";

#[cfg(not(feature = "polygon"))]

pub const MARKET_REGISTRY: &str = "0x5e30357d5136Bc4BfaDBA1ab341D0da09Fe7a9F1";

#[cfg(feature = "polygon")]

pub const MARKET_REGISTRY: &str = "0xeF0f89baC623eD7C875bC2F23b5403DcF90ba8Bd";

#[cfg(not(feature = "polygon"))]

pub const START_BLOCK: u64 = 15094704;

#[cfg(feature = "polygon")]

pub const START_BLOCK: u64 = 26017630;

use substreams_entity_change::pb::entity::{value::Typed, Value};

pub const TRANSACTION_TYPE_URL: &str = "sf.ethereum.type.v2.TransactionTrace";

pub enum GraphQlType {
    String,
    Int,
    Boolean,
    BigInt,
    Bytes,
    Address,
}

impl GraphQlType {
    pub fn default_value(&self) -> Value {
        match self {
            GraphQlType::BigInt => Value {
                typed: Some(Typed::Bigint("".to_string())),
            },
            GraphQlType::Int => Value {
                typed: Some(Typed::Int32(0)),
            },
            GraphQlType::Boolean => Value {
                typed: Some(Typed::Bool(false)),
            },
            GraphQlType::String => Value {
                typed: Some(Typed::String("".to_string())),
            },
            _ => Value {
                typed: Some(Typed::String("".to_string())),
            },
        }
    }
}

pub mod tables {

    use strum::EnumIter;

    use super::GraphQlType;

    #[derive(EnumIter)]
    /// An enum that models the tables in the substream

    pub enum MarketPlaceTable {
        Table,
        MarketId,
        Owner,
        FeeRecipient,
        MetadataUri,
        IsOpen,
        PaymentDefaultDuration,
        PaymentCycleDuration,
        PaymentCycleType,
        PaymentType,
        BidExpirationTime,
        BorrowerAttestationRequired,
        LenderAttestationRequired,
        MarketFeePercent,
        DurationTotal,
        DurationAverage,
        TotalNumberOfLenders,
    }

    impl MarketPlaceTable {
        pub fn key(&self) -> &str {
            match self {
                MarketPlaceTable::Table => "MarketPlace",
                MarketPlaceTable::MarketId => "marketplaceId",
                MarketPlaceTable::Owner => "owner",
                MarketPlaceTable::FeeRecipient => "feeRecipient",
                MarketPlaceTable::MetadataUri => "metadataURI",
                MarketPlaceTable::IsOpen => "isMarketOpen",
                MarketPlaceTable::PaymentDefaultDuration => "paymentDefaultDuration",
                MarketPlaceTable::PaymentCycleDuration => "paymentCycleDuration",
                MarketPlaceTable::PaymentCycleType => "paymentCycleType",
                MarketPlaceTable::PaymentType => "paymentType",
                MarketPlaceTable::BidExpirationTime => "bidExpirationTime",
                MarketPlaceTable::BorrowerAttestationRequired => "borrowerAttestationRequired",
                MarketPlaceTable::LenderAttestationRequired => "lenderAttestationRequired",
                MarketPlaceTable::MarketFeePercent => "marketplaceFeePercent",
                MarketPlaceTable::DurationTotal => "_durationTotal",
                MarketPlaceTable::DurationAverage => "durationAverage",
                MarketPlaceTable::TotalNumberOfLenders => "totalNumberOfLenders",
            }
        }

        pub fn required(&self) -> bool {
            match self {
                // true
                MarketPlaceTable::Table
                | MarketPlaceTable::IsOpen
                | MarketPlaceTable::PaymentDefaultDuration
                | MarketPlaceTable::PaymentCycleDuration
                | MarketPlaceTable::PaymentCycleType
                | MarketPlaceTable::PaymentType
                | MarketPlaceTable::BidExpirationTime
                | MarketPlaceTable::BorrowerAttestationRequired
                | MarketPlaceTable::LenderAttestationRequired
                | MarketPlaceTable::MarketFeePercent
                | MarketPlaceTable::DurationTotal
                | MarketPlaceTable::DurationAverage
                | MarketPlaceTable::TotalNumberOfLenders
                | MarketPlaceTable::MarketId => true,

                // false
                MarketPlaceTable::Owner
                | MarketPlaceTable::MetadataUri
                | MarketPlaceTable::FeeRecipient => false,
            }
        }

        pub fn get_type(&self) -> GraphQlType {
            match self {
                MarketPlaceTable::MarketId
                | MarketPlaceTable::MetadataUri
                | MarketPlaceTable::PaymentCycleType
                | MarketPlaceTable::PaymentType
                | MarketPlaceTable::Table => GraphQlType::String,

                MarketPlaceTable::FeeRecipient | MarketPlaceTable::Owner => GraphQlType::Bytes,

                MarketPlaceTable::BorrowerAttestationRequired
                | MarketPlaceTable::LenderAttestationRequired
                | MarketPlaceTable::IsOpen => GraphQlType::Boolean,

                MarketPlaceTable::PaymentDefaultDuration
                | MarketPlaceTable::BidExpirationTime
                | MarketPlaceTable::MarketFeePercent
                | MarketPlaceTable::DurationTotal
                | MarketPlaceTable::DurationAverage
                | MarketPlaceTable::TotalNumberOfLenders
                | MarketPlaceTable::PaymentCycleDuration => GraphQlType::BigInt,
            }
        }
    }
}
