use std::fs;

use near_sdk::json_types::U128;
use near_sdk::{AccountId, Gas};
use near_workspaces::types::NearToken;
use near_workspaces::Account;
use near_workspaces::Contract;
use rstest::{fixture, rstest};
use serde_json::json;

const FIVE_NEAR: NearToken = NearToken::from_near(5);
const ONE_TENTH_NEAR: NearToken = NearToken::from_millinear(100);
const NINE_HUNDREDTH_NEAR: NearToken = NearToken::from_millinear(90);

struct Common {
    contract: Contract,
    alice: Account,
    bob: Account,
    guest_book: Account,
}

#[fixture]
async fn base_contract() -> Common {
    let sandbox = near_workspaces::sandbox().await.unwrap();

    fs::create_dir_all("../../target/near/self_base").unwrap();
    let contract_wasm = near_workspaces::compile_project("../base").await.unwrap();

    let root = sandbox.root_account().unwrap();
    let alice = root.create_subaccount("alice").initial_balance(FIVE_NEAR).transact().await.unwrap().unwrap();
    let guest_book_account = root.create_subaccount("gbook").initial_balance(FIVE_NEAR).transact().await.unwrap().unwrap();

    let contract = guest_book_account
        .deploy(&contract_wasm)
        .await
        .unwrap()
        .into_result()
        .unwrap();

    let guest_book_init_outcome = guest_book_account
        .call(contract.id(), "init")
        .args_json(json!({"manager": alice.id().to_string() }))
        .transact()
        .await
        .unwrap();

    assert!(guest_book_init_outcome.is_success());

    let bob = root.create_subaccount("bob").initial_balance(FIVE_NEAR).transact().await.unwrap().unwrap();

    let bob_first_message_outcome = bob
        .call(contract.id(), "add_message")
        .args_json(json!({"text": "hello"}))
        .deposit(NINE_HUNDREDTH_NEAR)
        .transact()
        .await
        .unwrap();

    assert!(bob_first_message_outcome.is_success());

    let alice_first_message_outcome = alice
        .call(contract.id(), "add_message")
        .args_json(json!({"text": "bye"}))
        .deposit(ONE_TENTH_NEAR)
        .transact()
        .await
        .unwrap();

    assert!(alice_first_message_outcome.is_success());

    Common {
        contract,
        alice,
        bob,
        guest_book: guest_book_account,
    }
}

#[rstest]
#[tokio::test]
async fn test_self_updates_base_contract_returns(
    #[future] base_contract: Common,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_contract = base_contract.await;

    #[derive(near_sdk::serde::Deserialize, Debug, PartialEq, Eq)]
    #[serde(crate = "near_sdk::serde")]
    pub struct PostedMessage {
        pub premium: bool,
        pub sender: AccountId,
        pub text: String,
    }
    let messages_vec: Vec<PostedMessage> = base_contract
        .contract
        .view("get_messages")
        .args_json(json!({}))
        .await?
        .json()?;

    assert_eq!(
        messages_vec,
        vec![
            PostedMessage {
                premium: false,
                sender: base_contract.bob.id().clone(),
                text: "hello".to_string(),
            },
            PostedMessage {
                premium: true,
                sender: base_contract.alice.id().clone(),
                text: "bye".to_string(),
            },
        ]
    );

    let payments_vec: Vec<U128> = base_contract
        .contract
        .view("get_payments")
        .args_json(json!({}))
        .await?
        .json()?;

    assert_eq!(
        payments_vec,
        vec![
            U128(NINE_HUNDREDTH_NEAR.as_yoctonear()),
            U128(ONE_TENTH_NEAR.as_yoctonear())
        ]
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_self_updates_update_by_manager(
    #[future] base_contract: Common,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_contract = base_contract.await;

    fs::create_dir_all("../../target/near/self_update").unwrap();
    let updated_contract_wasm = near_workspaces::compile_project("./").await.unwrap();

    let manager_update_call_outcome = base_contract
        .alice
        .call(base_contract.guest_book.id(), "update_contract")
        .args(updated_contract_wasm)
        .gas(Gas::from_tgas(300))
        .transact()
        .await
        .unwrap();

    assert!(manager_update_call_outcome.is_success());

    let contract = base_contract.contract;

    #[derive(near_sdk::serde::Deserialize, Debug, PartialEq, Eq)]
    #[serde(crate = "near_sdk::serde")]
    pub struct PostedMessage {
        pub payment: NearToken,
        pub premium: bool,
        pub sender: AccountId,
        pub text: String,
    }
    let messages_vec: Vec<PostedMessage> = contract
        .view("get_messages")
        .args_json(json!({}))
        .await?
        .json()?;
    assert_eq!(
        messages_vec,
        vec![
            PostedMessage {
                payment: NINE_HUNDREDTH_NEAR,
                premium: false,
                sender: base_contract.bob.id().clone(),
                text: "hello".to_string(),
            },
            PostedMessage {
                payment: ONE_TENTH_NEAR,
                premium: true,
                sender: base_contract.alice.id().clone(),
                text: "bye".to_string(),
            },
        ]
    );
    let get_payments_result = contract.view("get_payments").args_json(json!({})).await;

    assert!(get_payments_result.is_err());
    Ok(())
}
