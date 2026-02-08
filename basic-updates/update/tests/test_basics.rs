use std::fs;

use near_api::{AccountId, NearToken};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;

const ONE_TENTH_NEAR: NearToken = NearToken::from_millinear(100);
const NINE_HUNDREDTH_NEAR: NearToken = NearToken::from_millinear(90);

#[tokio::test]
async fn test_contract_is_operational() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let sandbox = near_sandbox::Sandbox::start_sandbox().await?;
    let sandbox_network =
        near_api::NetworkConfig::from_rpc_url("sandbox", sandbox.rpc_addr.parse()?);

    fs::create_dir_all("../../target/near/base").unwrap();
    fs::create_dir_all("../../target/near/update").unwrap();

    // Build the base contract wasm file
    let contract_wasm_path = cargo_near_build::build_with_cli(
        cargo_near_build::BuildOpts::builder()
            .manifest_path("../base/Cargo.toml")
            .build(),
    )?;
    let contract_wasm = std::fs::read(contract_wasm_path)?;

    // Build the updated contract wasm file
    let updated_contract_wasm_path = cargo_near_build::build_with_cli(Default::default())?;
    let updated_contract_wasm = std::fs::read(updated_contract_wasm_path)?;

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

    // Deploy the base contract
    near_api::Contract::deploy(contract.account_id().clone())
        .use_code(contract_wasm)
        .without_init_call()
        .with_signer(signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    #[derive(near_sdk::serde::Deserialize, Debug, PartialEq, Eq)]
    #[serde(crate = "near_sdk::serde")]
    pub struct PostedMessage {
        pub premium: bool,
        pub sender: AccountId,
        pub text: String,
    }

    // Test the base contract by adding messages
    let _ = contract
        .call_function("add_message", json!({"text": "hello"}))
        .transaction()
        .deposit(NINE_HUNDREDTH_NEAR)
        .with_signer(contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    let _ = contract
        .call_function("add_message", json!({"text": "bye"}))
        .transaction()
        .deposit(ONE_TENTH_NEAR)
        .with_signer(alice.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    let messages_vec: Vec<PostedMessage> = contract
        .call_function("get_messages", json!({}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;

    assert_eq!(
        messages_vec,
        vec![
            PostedMessage {
                premium: false,
                sender: contract.account_id().clone(),
                text: "hello".to_string(),
            },
            PostedMessage {
                premium: true,
                sender: alice.account_id().clone(),
                text: "bye".to_string(),
            },
        ]
    );
    let payments_vec: Vec<U128> = contract
        .call_function("get_payments", json!({}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;

    assert_eq!(
        payments_vec,
        vec![
            U128(NINE_HUNDREDTH_NEAR.as_yoctonear()),
            U128(ONE_TENTH_NEAR.as_yoctonear())
        ]
    );

    // Deploy the updated contract
    near_api::Contract::deploy(contract.account_id().clone())
        .use_code(updated_contract_wasm)
        .without_init_call()
        .with_signer(signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // Call the migrate function on the updated contract
    let _ = contract
        .call_function("migrate", json!({}))
        .transaction()
        .with_signer(contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // Test the updated contract by getting messages
    let messages_vec: Vec<PostedMessage> = contract
        .call_function("get_messages", json!({}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;

    assert_eq!(
        messages_vec,
        vec![
            PostedMessage {
                premium: false,
                sender: contract.account_id().clone(),
                text: "hello".to_string(),
            },
            PostedMessage {
                premium: true,
                sender: alice.account_id().clone(),
                text: "bye".to_string(),
            },
        ]
    );

    // Test the updated contract by getting payments
    let _ = contract
        .call_function("get_payments", json!({}))
        .transaction()
        .with_signer(contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    Ok(())
}

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
