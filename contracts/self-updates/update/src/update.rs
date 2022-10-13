use near_sdk::{Promise, Gas};

use crate::*;

const NO_ARGS: Vec<u8> = vec![];
const CALL_GAS: Gas = Gas(200_000_000_000_000); // 200 TGAS

#[near_bindgen]
impl GuestBook{
    pub fn update_contract(&self) -> Promise {
        // Check the caller is authorized to update the code
        assert!(env::predecessor_account_id() == self.manager, "Only the manager can update the code");

        // Receive the code directly from the input to avoid the
        // GAS overhead of deserializing parameters
        let code = env::input().expect("Error: No input").to_vec();

        // Deploy the contract on self
        Promise::new(env::current_account_id())
        .deploy_contract(code)
        .function_call(
            "migrate".to_string(),
            NO_ARGS,
            0,
            CALL_GAS
        )
        .as_return()
    }
}