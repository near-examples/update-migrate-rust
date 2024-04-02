use near_sdk::near;

use near_sdk::collections::Vector;
use near_sdk::json_types::U128;

use near_sdk::{env, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_yoctonear(100_000_000_000_000_000_000_000);

#[near(serializers=[json, borsh])]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(contract_state)]
pub struct GuestBook {
    messages: Vector<PostedMessage>,
    payments: Vector<NearToken>,
}

impl Default for GuestBook {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
            payments: Vector::new(b"p"),
        }
    }
}

#[near]
impl GuestBook {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let payment = env::attached_deposit();
        let premium = payment >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            premium,
            sender,
            text,
        };
        self.messages.push(&message);
        self.payments.push(&payment);
    }

    pub fn get_messages(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<PostedMessage> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
            .collect()
    }

    pub fn get_payments(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<U128> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.payments
            .iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
            .map(|x| U128(x.as_yoctonear()))
            .collect()
    }
}
