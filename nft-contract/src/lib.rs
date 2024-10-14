use near_sdk::store::Vector;
use near_sdk::{env, near, AccountId, BorshStorageKey};
use near_sdk::json_types::U64;

#[near]
#[derive(BorshStorageKey)]
pub enum Prefix {
    Vector,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct NftData {
    id: U64,
    owner: AccountId,
    uri: String
}

#[near(contract_state)]
pub struct Contract {
    owner: AccountId,
    item_counter: U64,
    nfts: Vector<NftData>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::signer_account_id(),
            item_counter: U64(0),
            nfts: Vector::new(Prefix::Vector),
        }
    }
}

#[near]
impl Contract {
    pub fn mint(&mut self, uri: String) -> U64 {
        let sender = env::signer_account_id();

        let new_counter = self.item_counter.0 + 1;
        self.item_counter = U64(new_counter);  

        let nft = NftData {
            id: self.item_counter,
            owner: sender,
            uri
        };

        self.nfts.push(nft.clone());
        self.item_counter
    }

    pub fn get_total_count(&self) -> U64 {
        self.item_counter
    }

    pub fn get_nft(&self, index:U64) -> Option<NftData> {
        self.nfts.get(index.0 as u32).cloned()
    }

    pub fn get_owner_of_contract(&self) -> AccountId {
        self.owner.clone()
    }
}