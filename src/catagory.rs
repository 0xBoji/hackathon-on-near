use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};

use crate::award::{AwardId, AwardJson};

pub type CategoryId = u64;

// Define the Category structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub awards: Vec<AwardId>
    // pub prizes_list: Vec<PrizeId>
}

// Implement the Category structure
impl Category {
    pub fn new(category_id: CategoryId, name: String) -> Self {
        Category { 
            id: category_id, 
            name,
            awards: Vec::new()
            // prizes_list: Vec::new()
        }
    }
}

// Define the Category Json structure (response)
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoryJson {
    pub id: CategoryId,
    pub name: String,
    pub awards: Vec<AwardJson>
}