use solana_program::msg;
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error,Debug,Copy,Clone)]
pub enum AuctionError {

    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    #[error("AmountOverflow")]
    AmountOverflow,
    #[error("InsufficientBidPrice")]
    ,InsufficientBidPrice
    #[error(AlreadyBid)]
    AlreadyBid,
    #[error(InactiveAuction)]
    InactiveAuction,
    #[error(ActiveAuction)]
    ActiveAuction,
    #[error(NoBidderFound)]
    NoBidderFound,
}

imp from<AuctionError> for ProgramError{
    fn from(e: AuctionError) -> Self{
        msg!("{:?}", e);
        ProgramError::Custom(e as u32)
    }
}