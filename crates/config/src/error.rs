use std::num::ParseIntError;

use hex::FromHexError;
use thiserror::Error;
/// Error returned by config
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Failed to parse to Address or FixedBytes<32>
    #[error("FromHexError :{0}")]
    HexParse(#[from] FromHexError),
    /// Parse Error
    #[error("Parse Error :{0}")]
    ParseError(#[from] ruint::ParseError),
    /// ParseIntError
    #[error("Parse Int Error :{0}")]
    ParseIntError(#[from] ParseIntError),
}
