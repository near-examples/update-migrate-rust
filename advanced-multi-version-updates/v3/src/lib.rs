pub mod migrations;

use near_sdk::{near, BorshStorageKey, PanicOnDefault};

use near_sdk::borsh::BorshSerialize;
use near_sdk::json_types::{U128, U64};
use near_sdk::store::Vector;

use near_sdk::{env, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKey {
    Messages,
}

#[near(serializers=[json, borsh])]
pub struct PostedMessage {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct GuestBook {
    messages: Vector<PostedMessage>,
    owner: AccountId,
}

#[near]
impl GuestBook {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        // New contracts will use the latest state version
        migrations::state_version_write(&migrations::StateVersion::V3);

        Self {
            messages: Vector::new(StorageKey::Messages),
            owner,
        }
    }

    #[payable]
    pub fn add_message(&mut self, text: String) {
        let payment = env::attached_deposit();
        let premium = payment >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            payment,
            premium,
            sender,
            text,
        };
        self.messages.push(message);
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

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
}
