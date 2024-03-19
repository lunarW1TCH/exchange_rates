use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct UnsupportedCodeError(pub &'static str);

impl Error for UnsupportedCodeError {}

impl Display for UnsupportedCodeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

pub const FROM_CODE_ERROR: UnsupportedCodeError =
    UnsupportedCodeError("Provided `--from` currency code is not supported.");
    
pub const TO_CODE_ERROR: UnsupportedCodeError =
    UnsupportedCodeError("Provided `--to` currency code is not supported.");
