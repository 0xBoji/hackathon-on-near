
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use near_sdk::serde::{Serialize, Deserialize};

use crate::hackathon::{HackathonId, HackathonWithTotalPrize};

// Define the Member structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Member {
    pub id: AccountId,
    pub name: String,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub joined_hackathons: Vec<HackathonId>,
    pub created_hackathons: Vec<HackathonId>
}

// Implement the Member structure
impl Member {
    pub fn new(account_id: &AccountId, name: String, image: Option<String>, bio: Option<String>) -> Self {
        Member {
            id: account_id.clone(),
            name,
            image,
            bio,
            joined_hackathons: Vec::new(),
            created_hackathons: Vec::new(),
        }
    }
}

// Define the Member Json structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MemberJson {
    pub id: AccountId,
    pub name: String,
    pub image: Option<String>,
    pub bio: Option<String>,
}

// Define the Detail Member Json structure - include information of joined and created hackathons
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MemberJsonDetail {
    pub id: AccountId,
    pub name: String,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub joined_hackathons: Vec<HackathonWithTotalPrize>,
    pub created_hackathons: Vec<HackathonWithTotalPrize>
}