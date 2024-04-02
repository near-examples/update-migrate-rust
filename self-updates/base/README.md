# Guest Book Contract - Self Update

A [Guest Book Contract](../../basic-updates/base/) that can self-update.

This contract has a `update_contract` that takes as input a wasm file and deploys it on itself.

```rust
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
        NearToken::from_near(0),
        CALL_GAS
    )
    .as_return()
}
```

<br />

# Quickstart

## 1. Build and Deploy the Contract

_In this example we will be using [NEAR CLI](https://github.com/near/near-cli)
to intract with the NEAR blockchain and the smart contract and [near-cli-rs](https://near.cli.rs)
which provides more control over interactions and has interactive menus for subcommands selection_

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# from repo root
cd self-updates/base
cargo near build
```

Build and deploy:

```bash
# `update-migrate-rust-self-updates.testnet` was used as example of <target-account-id>
# `update-migrate-rust-self-updates-manager.testnet` was used as example of <manager-account-id>
cargo near deploy <target-account-id> with-init-call init json-args '{"manager":"<manager-account-id>"}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

<br />

## 2. Lock the Account

Check the account's full-access key:

```bash
# NEAR CLI
near keys <target-account-id>
# result: [{access_key: {"nonce": ..., permission: 'FullAccess'}, "public_key": '<key>'}]

# near-cli-rs 
near account list-keys <target-account-id> network-config testnet now
...
```

Remove the only full-access key, present in the output of the above command, from the <target-account-id>, thus leaving it locked:

```bash
# `ed25519:CE3AtAeHK3VKFPofhb9HLRoDEF2zmW5x9oWCoQU5a5FF` was used as example of <key>
# NEAR CLI
near delete-key <target-account-id> '<key>'

# near-cli-rs 
near account delete-keys <target-account-id> public-keys <key> network-config testnet sign-with-keychain send
```

<br />

## 3. Add a Message

```bash
# NEAR CLI
near call <target-account-id> add_message '{"text": "a message"}' --amount 0.1 --accountId <account>
# near-cli-rs 
near contract call-function as-transaction <target-account-id> add_message json-args '{"text": "a message"}' prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as <account> network-config testnet sign-with-keychain send
```

<br />

## 4. Retrieve the Stored Messages & Payments
`get_messages` and `get_payments` are read-only method (`view` method)

```bash
# NEAR CLI
near view <target-account-id> get_messages
# near-cli-rs 
near contract call-function as-read-only <target-account-id> get_messages json-args {} network-config testnet now
```
  
```bash
# NEAR CLI
near view <target-account-id> get_payments
# near-cli-rs 
near contract call-function as-read-only <target-account-id> get_payments json-args {} network-config testnet now
```

<br />

## 5. Continue in the Update Folder
Navigate to the [update](../update/) folder to continue
