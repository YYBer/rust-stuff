use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]

pub enum OracleErrorCode {
    #[msg("Not a valid account")]
    InvalidAccount,
}