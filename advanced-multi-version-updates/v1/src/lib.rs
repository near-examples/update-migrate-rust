use near_sdk::{near, BorshStorageKey};

use near_sdk::borsh::BorshSerialize;
use near_sdk::json_types::{U128, U64};
use near_sdk::store::Vector;

use near_sdk::{env, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKey {
    Messages,
    Payments,
}

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
            messages: Vector::new(StorageKey::Messages),
            payments: Vector::new(StorageKey::Payments),
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
        self.messages.push(message);
        self.payments.push(payment);
    }

    pub fn get_messages(
        &self,
        from_index: Option<U128>,
        limit: Option<U64>,
    ) -> Vec<&PostedMessage> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(u64::from(limit.unwrap_or(U64::from(10))) as usize)
            .collect()
    }

    pub fn get_payments(&self, from_index: Option<U128>, limit: Option<U64>) -> Vec<U128> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.payments
            .iter()
            .skip(from as usize)
            .take(u64::from(limit.unwrap_or(U64::from(10))) as usize)
            .map(|x| U128(x.as_yoctonear()))
            .collect()
    }
}
