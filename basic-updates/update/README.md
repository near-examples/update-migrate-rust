# Guest Book Contract

The [base](../base) contract was modified, removing the `payments` field
and including that information in the `PostedMessage` structure.

```rust
pub struct PostedMessage {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub struct GuestBook {
  messages: Vector<PostedMessage>,
}
```

If we deploy this contract on top of the [base](../base/) one and call any method we will get the error:

```
panicked at 'Cannot deserialize the contract state.: ... }',
``` 

This is because the new contract expects to find `PostedMessages` with 4 fields (`payment`, `premium`, `sender`, `text`)
but the saved messages only have 3 fields (they lack the `payment` field).

In order to fix this problem we need to `migrate` the state, i.e. iterate through the current saved messages
transforming them to the new version.

```rust
#[private]
#[init(ignore_state)]
pub fn migrate() -> Self {
    // retrieve the current state from the contract
    let old_state: OldState = env::state_read().expect("failed");

    // iterate through the state migrating it to the new version
    let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

    for (idx, posted) in old_state.messages.iter().enumerate() {
        let payment = old_state
            .payments
            .get(idx as u64)
            .unwrap_or(NearToken::from_near(0));

        new_messages.push(&PostedMessage {
            payment,
            premium: posted.premium,
            sender: posted.sender,
            text: posted.text,
        })
    }

    // return the new state
    Self {
        messages: new_messages,
    }
}
```

<br />

# Upgrading Base Contract

## 1. Build & Deploy & Migrate State

_In this example we will be using [NEAR CLI](https://github.com/near/near-cli)
to intract with the NEAR blockchain and the smart contract and [near-cli-rs](https://near.cli.rs)
which provides more control over interactions and has interactive menus for subcommands selection_

To build contract install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# from repo root
cd basic-updates/update
cargo near build
```

You can deploy the updated contract by running:

```bash
# `update-migrate-rust-basic-updates-base.testnet` was used as example of <target-account-id>
# NEAR CLI
near deploy <target-account-id> ../../target/near/update/update.wasm
# near-cli-rs 
near contract deploy <target-account-id> use-file ../../target/near/update/update.wasm without-init-call network-config testnet sign-with-keychain send
```

Run this command to see the "Cannot deserialize..." error
```bash
# NEAR CLI
near view <target-account-id> get_messages
# near-cli-rs 
near contract call-function as-read-only <target-account-id> get_messages json-args {} network-config testnet now
```

Ask the contract to migrate the state

```bash
# NEAR CLI
near call <target-account-id> migrate {} --accountId <target-account-id>
# near-cli-rs (may be useful to specify more gas for large state migrations)
near contract call-function as-transaction <target-account-id> migrate json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as <target-account-id> network-config testnet sign-with-keychain send
```

#### Deploying and Migrating
You can actually deploy the contract and migrate the state in one line:

```bash
# NEAR CLI
near deploy <target-account-id> ../../target/near/update/update.wasm --initFunction migrate --initArgs {}
# near-cli-rs (may be useful to specify more gas for large state migrations)
near contract deploy <target-account-id> use-file ../../target/near/update/update.wasm with-init-call migrate json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

<br />

## 2. Retrieve the Stored Messages
`get_messages` will now return messages that include a `payment` field.

```bash
# NEAR CLI
near view <target-account-id> get_messages
```

`get_payments` will raise an error since the method does not exist anymore.

```bash
# NEAR CLI
# raises an error since the method is gone
near view <target-account-id> get_payments
```
