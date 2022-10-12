use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, Balance};

use versioned_msg::{PostedMessageV2, VersionedPostedMessage};
mod versioned_msg;

const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuestBook {
    messages: Vector<VersionedPostedMessage>,
}

impl Default for GuestBook {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

#[near_bindgen]
impl GuestBook {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let payment = env::attached_deposit();
        let sender = env::predecessor_account_id();
        let premium = payment >= POINT_ONE;
        let message = VersionedPostedMessage::V2(PostedMessageV2 {
            payment,
            sender,
            premium,
            text,
        });
        self.messages.push(&message);
    }

    pub fn get_messages(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<PostedMessageV2> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
            .map(|message| message.into())
            .collect()
    }
}