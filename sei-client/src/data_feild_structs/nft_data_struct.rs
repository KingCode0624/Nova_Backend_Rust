use serde::{Deserialize, Serialize};
use crate::data_rp_structs::{nft_collect_contract_rp_struct::NftCollectionInfo, tx_rp_struct::FeeAmount};


//  nft info
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct NftInfo{
    pub token_id:String,
    pub name:String, // CollectionInfo name + # + id 
    pub key:String, // collection + - +id
    pub image:String,
    pub royalty_percentage:u64,
    pub attributes:Vec<NftAttribute>,
}
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct NftAttribute{
    pub trait_type:String,
    pub value:String,
}

// 用户持有的 nft collect
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct NftCollectHold{
    pub collect_address:String,
    pub collect_info:NftCollectionInfo,
    pub nfts_hold:Vec<NftInfo>
}

// transaction type
#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub enum NftTransaction {
    Mint(Mint),
    BatchBids(BatchBids),
    OnlyCreateAuction(OnlyCreateAuction),
    Transfer(Transfer),
    FixedSell(FixedSell),
    PurchaseCart(PurchaseCart),
    AcceptBid(AcceptBid),
    CreateAuction(CreateAuction),
    CancelAuction(CancelAuction),
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct OnlyCreateAuction{
    pub collection:String,
    pub nft_id:String,
    pub auction_price:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct Transfer{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct Mint{
    pub collection:String,
    pub recipient:String,
    pub nft_id:String,
    pub price:Option<String>,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct FixedSell{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub sale_price:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct BatchBids{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub sale_price:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct PurchaseCart{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub buyer:String,
    pub seller:String,
    pub sale_price:String,
    pub marketplace_fee:String,
    pub royalties:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct AcceptBid{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub bidder:String,
    pub seller : String,
    pub sale_price:String,
    pub marketplace_fee:String,
    pub royalties:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct CreateAuction{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub auction_price:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}

#[derive(Serialize, Deserialize,Clone,Debug,PartialEq, Eq)]
pub struct CancelAuction{
    pub collection:String,
    pub sender:String,
    pub recipient:String,
    pub nft_id:String,
    pub auction_price:String,
    pub transaction_sender:Option<String>,
    pub fee:Vec<FeeAmount>,
    pub ts:String,
    pub tx:String,
}