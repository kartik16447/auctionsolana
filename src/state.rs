use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack,Sealed},
    pubkey::Pubkey,
}

use arrayref::{array_ref,array_ref,array_refs, mut_array_refs};
use solana_program::program_pack::IsInitialized;
use anchor_lang::{prelude::*,AnchorDeserialize,AccountSerialize};

use crate::constants::*;

pub struct Auction{
    // if it is intialized or not
    pub is_initialized: bool,
    // public key of the owner of auction
    pub exhibitor_pubkey: Pubkey,
    // public key of the nft to be auctioned
    pub exhibiting_nft_temp_pubkey: Pubkey,

    pub exhibitor_ft_receiving_pubkey: Pubkey,

    // current price
    pub price: u64,

    pub end_at:i64,

    pub highest_bidder_pubkey:Pubkey,

    pub highest_bidder_ft_temp_pubkey: Pubkey,

    pub highest_bidder_ft_returning_pubkey: Pubkey,


}

impl Sealed for Auction{}
impl IsInitialized for Auction{
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for Auction{
    const LEN: usize =  209;
    fn unpack_from_slice(src:&[u8]) -> Result<Self,ProgramError> {
        let src = array_ref![src,0,209];
        let(
            is_initialized,
             exhibitor_pubkey,
              exhibiting_nft_temp_pubkey,
              exhibitor_ft_receiving_pubkey,
              price,
              end_at,
              highest_bidder_pubkey,
              highest_bidder_ft_temp_pubkey,
              highest_bidder_ft_returning_pubkey
            )=array_refs![src,1,36,36,36,8,8,36,36,36];
            let is_initialized = match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),   
            };
            Ok(Auction{
                is_initialized,
                 exhibitor_pubkey:Pubkey::new_from_array(*exhibitor_pubkey),
                 exhibiting_nft_temp_pubkey:Pubkey::new_from_array(*exhibiting_nft_temp_pubkey),
                 exhibitor_ft_receiving_pubkey:Pubkey::new_from_array(*exhibitor_ft_receiving_pubkey),
                 price:u64::from_le_bytes(*price),
                 end_at:i64::from_le_bytes(*end_at),
                 highest_bidder_pubkey: Pubkey::new_from_array(*highest_bidder_pubkey),
                 highest_bidder_ft_temp_pubkey: Pubkey::new_from_array(*highest_bidder_ft_temp_pubkey),
                 highest_bidder_ft_returning_pubkey: Pubkey::new_from_array(*highest_bidder_ft_returning_pubkey),
            })
    }


    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst,0,209];
        let(
            is_initialized_dst,
             exhibitor_pubkey_dst,
              exhibiting_nft_temp_pubkey_dst,
              exhibitor_ft_receiving_pubkey_dst,
              price_dst,
              end_at_dst,
              highest_bidder_pubkey_dst,
              highest_bidder_ft_temp_pubkey_dst,
              highest_bidder_ft_returning_pubkey_dst,
            )=mut_array_refs![dst,1,36,36,36,8,8,36,36,36];

            let &Auction {
                is_initialized,
                ref exhibitor_pubkey,
                ref exhibiting_nft_temp_pubkey,
                ref exhibitor_ft_receiving_pubkey,
                price,
                end_at,
                highest_bidder_pubkey,
                highest_bidder_ft_temp_pubkey,
                highest_bidder_ft_returning_pubkey,
            }= self;
             is_initialized_dst[0] = *is_initialized as u8;
             exhibitor_pubkey_dst.copy_from_slice(exhibitor_pubkey.as_ref());
             exhibiting_nft_temp_pubkey_dst.copy_from_slice(exhibiting_nft_temp_pubkey.as_ref());
             exhibitor_ft_receiving_pubkey_dst.copy_from_slice(exhibitor_ft_receiving_pubkey.as_ref());

             *price= price.to_le_bytes();
             *end_at= end_at.to_le_bytes();
                
             highest_bidder_pubkey_dst.copy_from_slice(highest_bidder_pubkey.as_ref());
             highest_bidder_ft_temp_pubkey_dst.copy_from_slice(highest_bidder_ft_temp_pubkey.as_ref());
             highest_bidder_ft_returning_pubkey_dst.copy_from_slice(highest_bidder_ft_returning_pubkey.as_ref());
            }
    }

