pub const PREFIX: &str ="auction_house";
pub const FEE_PAYER: &str ="fee_payer";
pub const TREASURY: &str="treasury";
// pub const TREASURY: &str="treasury";
pub const SIGNER: &str="signer";
pub const PURCHASE_RECEIPT_PREFIX: &str="purchase_receipt";
pub const BID_RECEIPT_PREFIX: &str="bid_receipt";
pub const AUCTIONEER: &str="auctioneer";
pub const LISTING_RECIEPT_PREFIX: &str="listing_receipt";
pub const TRADE_STATE_SIZE: usize = 1;
pub const MAX_NUM_SCOPES: usize = 7;
pub const AUCTIONEER_SIZE: usize = 8+32+32+MAX_NUM_SCOPES+64;

pub const AUCTION_HOUSE_SIZE: usize = 8+32+32+32+32+32+32+32+1+1+1+2+1+1+8+1+8+203;
