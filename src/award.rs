use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Serialize, Deserialize};

use crate::submission::{SubmissionId, SubmissionJson};

pub type AwardId = u64;

// Define the Award structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Award {
    pub id: AwardId,
    pub name: String,
    pub price: U128,
    pub winner: Option<SubmissionId>,
    pub is_awarded: bool
}

// Implement the Award structure
impl Award {
    pub fn new(id: AwardId,name: String, price: U128) -> Self {
        Award { id: id, name, price, winner: None, is_awarded: false }
    }
}

#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AwardJson {
    pub id: AwardId,
    pub name: String,
    pub price: U128,
    pub winner: Option<SubmissionJson>,
    pub is_awarded: bool
}