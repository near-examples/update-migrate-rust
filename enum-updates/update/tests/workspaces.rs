use near_sdk::NearToken;
use serde_json::json;
use std::fs;

use near_sdk::AccountId;

const FIVE_NEAR: NearToken = NearToken::from_near(5);
const ONE_NEAR: NearToken = NearToken::from_near(1);
const NINE_HUNDREDTH_NEAR: NearToken = NearToken::from_millinear(90);

#[tokio::test]
async fn test_enum_updates_migration() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await.unwrap();

    fs::create_dir_all("../../target/near/enums_base").unwrap();
    let base_contract_wasm = near_workspaces::compile_project("../base").await.unwrap();

    let root = sandbox.root_account().unwrap();
    let guest_book_account = root.create_subaccount("gbook").initial_balance(FIVE_NEAR).transact().await.unwrap().unwrap();

    let contract = guest_book_account
        .deploy(&base_contract_wasm)
        .await
        .unwrap()
        .into_result()
        .unwrap();

    let alice = root.create_subaccount("alice").initial_balance(FIVE_NEAR).transact().await.unwrap().unwrap();

    let guest_book_message_outcome = guest_book_account
        .call(contract.id(), "add_message")
        .args_json(json!({"text": "hello"}))
        .deposit(NINE_HUNDREDTH_NEAR)
        .transact()
        .await
        .unwrap();

    assert!(guest_book_message_outcome.is_success());

    let alice_first_message_outcome = alice
        .call(contract.id(), "add_message")
        .args_json(json!({"text": "bye"}))
        .deposit(ONE_NEAR)
        .transact()
        .await
        .unwrap();

    assert!(alice_first_message_outcome.is_success());

    fs::create_dir_all("../../target/near/enums_update").unwrap();
    let updated_contract_wasm = near_workspaces::compile_project("./").await.unwrap();

    let updated_contract = guest_book_account
        .deploy(&updated_contract_wasm)
        .await
        .unwrap()
        .into_result()
        .unwrap();

    #[derive(near_sdk::serde::Deserialize, Debug, PartialEq, Eq)]
    #[serde(crate = "near_sdk::serde")]
    pub struct PostedMessageV2 {
        pub payment: NearToken,
        pub premium: bool,
        pub sender: AccountId,
        pub text: String,
    }

    let messages_vec: Vec<PostedMessageV2> = updated_contract
        .view("get_messages")
        .args_json(json!({}))
        .await?
        .json()?;

    assert_eq!(
        messages_vec,
        vec![
            PostedMessageV2 {
                payment: NearToken::from_near(0),
                premium: false,
                sender: guest_book_account.id().clone(),
                text: "hello".to_string(),
            },
            PostedMessageV2 {
                payment: NearToken::from_near(0),
                premium: true,
                sender: alice.id().clone(),
                text: "bye".to_string(),
            },
        ]
    );
    let alice_first_message_outcome = alice
        .call(updated_contract.id(), "add_message")
        .args_json(json!({"text": "howdy"}))
        .deposit(ONE_NEAR)
        .transact()
        .await
        .unwrap();

    assert!(alice_first_message_outcome.is_success());

    let messages_vec: Vec<PostedMessageV2> = updated_contract
        .view("get_messages")
        .args_json(json!({}))
        .await?
        .json()?;

    assert_eq!(
        messages_vec,
        vec![
            PostedMessageV2 {
                payment: NearToken::from_near(0),
                premium: false,
                sender: guest_book_account.id().clone(),
                text: "hello".to_string(),
            },
            PostedMessageV2 {
                payment: NearToken::from_near(0),
                premium: true,
                sender: alice.id().clone(),
                text: "bye".to_string(),
            },
            PostedMessageV2 {
                payment: NearToken::from_near(1),
                premium: true,
                sender: alice.id().clone(),
                text: "howdy".to_string(),
            },
        ]
    );
    Ok(())
}
