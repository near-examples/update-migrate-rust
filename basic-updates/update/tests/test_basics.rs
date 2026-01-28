use rstest::{fixture, rstest};
use std::fs;

use serde_json::json;

use near_sdk::{json_types::U128, AccountId};

const FIVE_NEAR: NearToken = NearToken::from_near(5);
const ONE_TENTH_NEAR: NearToken = NearToken::from_millinear(100);
const NINE_HUNDREDTH_NEAR: NearToken = NearToken::from_millinear(90);

struct Common {
    contract: Contract,
    alice: Account,
    guest_book: Account,
}

#[tokio::test]
async fn base_contract() -> Common {
    // Initialize the sandbox
    let sandbox = near_sandbox::Sandbox::start_sandbox().await?;
    let sandbox_network =
        near_api::NetworkConfig::from_rpc_url("sandbox", sandbox.rpc_addr.parse()?);

    fs::create_dir_all("../../target/near/base").unwrap();

    // Build the contract wasm file
    let contract_wasm_path = cargo_near_build::build_with_cli(Default::default())?;
    // let contract_wasm_path = cargo_near_build::build_with_cli(
    //     cargo_near_build::BuildOpts::builder()
    //         .manifest_path("../base/Cargo.toml")
    //         .build(),
    // )?;
    let contract_wasm = std::fs::read(contract_wasm_path)?;

    // Create accounts
    let alice = create_subaccount(&sandbox, "alice.sandbox").await?;
    let contract = create_subaccount(&sandbox, "gbook.sandbox")
        .await?
        .as_contract();

    // Initialize signer for the contract deployment
    let signer = near_api::Signer::from_secret_key(
        near_sandbox::config::DEFAULT_GENESIS_ACCOUNT_PRIVATE_KEY
            .parse()
            .unwrap(),
    )?;

    // Deploy the contract
    near_api::Contract::deploy(contract.account_id().clone())
        .use_code(nft_wasm)
        .without_init_call()?
        .with_signer(signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    let guest_book_message_outcome = contract
        .call_function("add_message", json!({"text": "hello"}))
        .transaction()
        .deposit(NINE_HUNDREDTH_NEAR)
        .with_signer(contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    let alice_first_message_outcome = contract
        .call_function("add_message", json!({"text": "bye"}))
        .transaction()
        .deposit(ONE_TENTH_NEAR)
        .with_signer(alice.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    Common {
        contract,
        alice,
        guest_book: contract,
    }
}

// #[rstest]
// #[tokio::test]
// async fn test_basic_updates_base_contract_returns(
//     #[future] base_contract: Common,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let base_contract = base_contract.await;

//     #[derive(near_sdk::serde::Deserialize, Debug, PartialEq, Eq)]
//     #[serde(crate = "near_sdk::serde")]
//     pub struct PostedMessage {
//         pub premium: bool,
//         pub sender: AccountId,
//         pub text: String,
//     }
//     let messages_vec: Vec<PostedMessage> = base_contract
//         .contract
//         .call_function("get_messages", json!({}))
//         .read_only()
//         .fetch_from(&sandbox_network)
//         .await?
//         .data;

//     assert_eq!(
//         messages_vec,
//         vec![
//             PostedMessage {
//                 premium: false,
//                 sender: base_contract.guest_book.account_id().clone(),
//                 text: "hello".to_string(),
//             },
//             PostedMessage {
//                 premium: true,
//                 sender: base_contract.alice.account_id().clone(),
//                 text: "bye".to_string(),
//             },
//         ]
//     );
//     let payments_vec: Vec<U128> = base_contract
//         .contract
//         .call_function("get_payments", json!({}))
//         .read_only()
//         .fetch_from(&sandbox_network)
//         .await?
//         .data;

//     assert_eq!(
//         payments_vec,
//         vec![
//             U128(NINE_HUNDREDTH_NEAR.as_yoctonear()),
//             U128(ONE_TENTH_NEAR.as_yoctonear())
//         ]
//     );

//     Ok(())
// }

// #[rstest]
// #[tokio::test]
// async fn test_basic_updates_migration(
//     #[future] base_contract: Common,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let base_contract = base_contract.await;

//     fs::create_dir_all("../../target/near/update").unwrap();
//     let updated_contract_wasm = near_workspaces::compile_project("./").await.unwrap();

//     let migrated_contract = base_contract
//         .guest_book
//         .deploy(&updated_contract_wasm)
//         .await
//         .unwrap()
//         .into_result()
//         .unwrap();

//     let migrate_call_outcome = base_contract
//         .guest_book
//         .call(migrated_contract.id(), "migrate")
//         .args_json(json!({}))
//         .transact()
//         .await
//         .unwrap();

//     assert!(migrate_call_outcome.is_success());

//     #[derive(near_sdk::serde::Deserialize, Debug, PartialEq, Eq)]
//     #[serde(crate = "near_sdk::serde")]
//     pub struct PostedMessage {
//         pub payment: NearToken,
//         pub premium: bool,
//         pub sender: AccountId,
//         pub text: String,
//     }
//     let messages_vec: Vec<PostedMessage> = migrated_contract
//         .view("get_messages")
//         .args_json(json!({}))
//         .await?
//         .json()?;
//     assert_eq!(
//         messages_vec,
//         vec![
//             PostedMessage {
//                 payment: NINE_HUNDREDTH_NEAR,
//                 premium: false,
//                 sender: base_contract.guest_book.id().clone(),
//                 text: "hello".to_string(),
//             },
//             PostedMessage {
//                 payment: ONE_TENTH_NEAR,
//                 premium: true,
//                 sender: base_contract.alice.id().clone(),
//                 text: "bye".to_string(),
//             },
//         ]
//     );
//     let get_payments_result = migrated_contract
//         .view("get_payments")
//         .args_json(json!({}))
//         .await;

//     assert!(get_payments_result.is_err());
//     Ok(())
// }

async fn create_subaccount(
    sandbox: &near_sandbox::Sandbox,
    name: &str,
) -> testresult::TestResult<near_api::Account> {
    let account_id: AccountId = name.parse().unwrap();
    sandbox
        .create_account(account_id.clone())
        .initial_balance(NearToken::from_near(10))
        .send()
        .await?;
    Ok(near_api::Account(account_id))
}
