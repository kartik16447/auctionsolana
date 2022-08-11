// use anchor_lang::prelude::*;

// declare_id!("FafFr3nZ3hZKwrNGwFdJxNwxMCQo7cD2ZScc4B8sBNnM");
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature="no-entrypoint"))]
mod entrypoint;

pub use solana_program;

solana_program::declare_id!("t")


















#[program]
pub mod auct_practice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let a=&mut ctx.accounts.auction_account;
        a.authority= *ctx.accounts.authority.key;
        Ok(())

        pub fn Bid(ctx:Context<Bid>,base_price:,fess:u64,start_time:u64,end_time:u64)-> ProgramResult{

            let bid= from_utf8(&new_bid)

        }
}

#[account]
pub struct AuctionAccount{
    pub authority: Pubkey,
    pub bid: Bid,
}
pub struct Bid
{
    pub id: u8,
    pub bid_state: BidState,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub last_bid: Option<UnixTimestamp>,
    pub ended_at: Option<UnixTimestamp>,
    pub end_bid_at: Option<UnixTimestamp>,
    pub minimum_price: u64,
    pub minimum_increment: u64,
    pub bump : u8,
}

pub Bider{
    pub name: Pubkey,
    pub bid_id:u8,
    pub balance: u64,
    pub bump: u8,
}
pub Makebid{
    pub name: Bider,
    pub amount: u64,
}


#[repr(C)]
#[derive(Clone, BorshSerialise, BorshDeserialise, PartialEq, Debug)]
pub enum BidState{
    Created,Started,
    Ended { winner:Pubkey},
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=authority,space= 32+8+32+8+8+8)]

    pub auction_account: Account<'info,AuctionAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info,System>
}
// 
#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut,has_one = authority)]
    pub bid_account: Account<'info,Bid>, 
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MakeBid<'info>{
    #[account(mut,has_one = authority)]
    pub make_bid: Account<'info,MakeBid>,
    pub authority: Signer<'info>,

}

impl Bid{
    pub fn 
}





// #[derive(Accounts)]
// pub struct Initialize {}

// pub mod auctioneer;
// pub mod bid;
// pub mod cancel;
// pub mod constants;
// pub mod deposit;
// pub mod errors;
// pub mod pda;
// pub mod receipt;
// pub mod sell;
// pub mod utils;
// pub mod withdraw;
// pub mod execute_sale;
// pub mod state;