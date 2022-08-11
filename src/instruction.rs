//use crate::error

use solana_program::{
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use std::convert::TryInto;
use std::mem::size_of;


//

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub enum AuctionInstruction{
    IitializeAuction{
        mint:Pubkey,
        price: i64,
        end_at: UnixTimestamp,
        authority:Pubkey,
    },
 InitializeAccount,   
}



pub enum AuctionInstruction{
    Exhibit{
        intial_price:u64,
        seconds: u64
    },
    Bid{
        price:u64,
    },
    Cancel {},
    Close{},
}

imp AuctionInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError{
        let (instruction_type,rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match instruction_type{
            0 => Self::Exhibit{
                intial_price : Self::unpack(rest,0)?,
                seconds : Self::unpack64(rest,8)?,
        },
        1 => Self::Bid{
            price::Self::unpack(rest,0)?,
        },
        2 => Self::Cancel{},
        3 => Self::Close{},
        _ => return Err(InvalidInstruction.into()),
            
        })
    }

    fn unpack64(input: &[u8], start:usize) -> Result<u64,ProgramError>{
        let v= input
            .get(start..start+8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
            Ok(v)
    }
}






















#[derive(BorshDeserialise,BorshSerialise,Debug,Clone)]
pub enum BidInstruction{
    CreateBid{
        token_mint: Pubkey,
        ended_at: Option<UnixTimestamp>,
        minimum_price: u64,
        minimum_increment: u64,
        bid_state: Created,
    },
    Bid{
        name: Pubkey,
        bid_id:u8,
    },

}

imp BidInstruction{
    pub fn unpack(input: &[u8])-> Result<Self,ProgramError>{
        let(tag,rest) = input
        .split_first()
        .ok_or(ProgramError::from(BidError::InvalidInstruction))?;
        match tag {
            0=> BidInstruction::create_bid(rest),
            1=> BidInstruction::Bidder_bid(rest),
            _=> Err(ProgramError::InvalidInstructionData),
        }
    }
}
// fn create_bid(input: &[u8]) -> Result<Self, ProgramError{
//     let mut start_index: usize = 0;

//     let 
// }