use std::str::FromStr;

use crate::*;
use near_sdk::{
    borsh::{to_vec, BorshDeserialize},
    near, PanicOnDefault, Promise,
};

#[near]
#[derive(Debug)]
pub(crate) enum StateVersion {
    V1,
    V2,
    V3,
}

#[near]
#[derive(PanicOnDefault)]
struct GuestBookV1 {
    messages: Vector<PostedMessageV1>,
    payments: Vector<NearToken>,
}

// This structure is implemented from V1
#[near]
#[derive(Clone)]
struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

// From V1 to V2
impl GuestBook {
    fn unsafe_add_owner() {
        let GuestBookV1 { messages, payments } = env::state_read().unwrap();
        let owner = AccountId::from_str("bob.near").unwrap();

        env::state_write(&GuestBookV2 {
            messages,
            payments,
            owner,
        });
    }
}

#[near]
#[derive(PanicOnDefault)]
struct GuestBookV2 {
    messages: Vector<PostedMessageV1>,
    payments: Vector<NearToken>,
    owner: AccountId,
}

// From V2 to V3
impl GuestBook {
    fn unsafe_add_payment_to_message() {
        let GuestBookV2 {
            messages: old_messages,
            mut payments,
            owner,
        } = env::state_read().unwrap();

        let default_payment = NearToken::from_yoctonear(0);

        // New messages must be written to storage
        let mut messages = Vector::new(StorageKey::Messages);

        for (idx, old_message) in old_messages.iter().enumerate() {
            let payment = payments.get(idx as u32).unwrap_or(&default_payment);

            messages.push(PostedMessageV3 {
                premium: old_message.premium.clone(),
                sender: old_message.sender.clone(),
                text: old_message.text.clone(),
                payment: payment.clone(),
            });
        }

        // Payments must be removed from storage
        payments.clear();

        env::state_write(&GuestBookV3 { messages, owner });
    }
}

#[near]
#[derive(PanicOnDefault)]
struct GuestBookV3 {
    messages: Vector<PostedMessageV3>,
    owner: AccountId,
}

// New field was introduced as part of V3
#[near]
#[derive(Clone)]
struct PostedMessageV3 {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

// Implement publicly available functions of the contract for self-upgrade and migration
#[near]
impl GuestBook {
    pub fn unsafe_self_upgrade() {
        near_sdk::assert_self();

        let contract = env::input().expect("No contract code is attached in input");
        Promise::new(env::current_account_id())
            .deploy_contract(contract)
            .then(Promise::new(env::current_account_id()).function_call(
                "unsafe_migrate".to_string(),
                Vec::new(),
                NearToken::from_near(0),
                env::prepaid_gas().saturating_sub(near_sdk::Gas::from_tgas(100)),
            ))
            .as_return();
    }

    fn migration_done() {
        near_sdk::log!("Migration done.");
        env::value_return(b"\"done\"");
    }

    fn needs_migration() {
        env::value_return(b"\"needs-migration\"");
    }

    pub fn unsafe_migrate() {
        near_sdk::assert_self();
        let current_version = state_version_read();
        near_sdk::log!("Migrating from version: {:?}", current_version);
        match current_version {
            StateVersion::V1 => {
                GuestBook::unsafe_add_owner();
                state_version_write(&StateVersion::V2);
            }
            StateVersion::V2 => {
                GuestBook::unsafe_add_payment_to_message();
                state_version_write(&StateVersion::V3);
            }
            _ => {
                return GuestBook::migration_done();
            }
        }
        GuestBook::needs_migration();
    }
}

const VERSION_KEY: &[u8] = b"VERSION";

fn state_version_read() -> StateVersion {
    env::storage_read(VERSION_KEY)
        .map(|data| {
            StateVersion::try_from_slice(&data).expect("Cannot deserialize the contract state.")
        })
        .unwrap_or(StateVersion::V1) // StateVersion is introduced in V2 State.
}

pub(crate) fn state_version_write(version: &StateVersion) {
    let data = to_vec(&version).expect("Cannot serialize the contract state.");
    env::storage_write(VERSION_KEY, &data);
    near_sdk::log!("Migrated to version: {:?}", version);
}
