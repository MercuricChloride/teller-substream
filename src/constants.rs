// if we aren't building for polygon, we are building for mainnet
#[cfg(not(feature = "polygon"))]
pub const TELLER_V2: &str = "0x00182fdb0b880ee24d428e3cc39383717677c37e";

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
