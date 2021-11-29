use num_derive::FromPrimitive;
use anchor_lang::solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PortAdaptorError {
    #[error("Only has borrows, no collaterals")]
    Insolvency,

    #[error("CollateralIndexOutOfBound")]
    CollateralIndexOutOfBound,
    #[error("BorrowIndexOutOfBound")]
    BorrowIndexOutOfBound

}

impl From<PortAdaptorError> for ProgramError {
    fn from(e: PortAdaptorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PortAdaptorError {
    fn type_of() -> &'static str {
        "PortAdaptor Error"
    }
}
