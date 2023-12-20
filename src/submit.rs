use near_sdk::Timestamp;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ AccountId, serde::{Serialize, Deserialize}};

pub type SubmissionId = u64;

use crate::category::{CategoryId, Category};
use crate::member::MemberJson;

// Define the Submission structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Submission {
    pub id: SubmissionId,
    pub name: String,
    pub description: String,
    pub image: String,
    pub time: Timestamp,
    pub link: Vec<String>,
    pub categories: Vec<CategoryId>,
    pub members: Vec<AccountId>
}

// Implement the Submission structure
impl Submission {
    pub fn new(submission_id: u64, categories: Vec<u64>, members: Vec<AccountId>, name: String, description: String, image: String, link: Vec<String>, time: Timestamp ) -> Self {
        Submission { id: submission_id, categories, members, name, description, image, link, time }
    }
} 

// Define the Detail Submission Json structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubmissionJson {
    pub id: SubmissionId,
    pub name: String,
    pub description: String,
    pub image: String,
    pub time: Timestamp,
    pub link: Vec<String>,
    pub categories: Vec<Category>,
    pub members: Vec<MemberJson>
}