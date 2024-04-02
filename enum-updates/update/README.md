# Guest Book Contract - Versioned Messages Update

This examples adds a new version to the **versioned** messages of the [enum-base](../base/) contracts.

```rust
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub struct PostedMessageV2 {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub enum VersionedPostedMessage {
    V1(PostedMessageV1),
    V2(PostedMessageV2),
}

impl From<VersionedPostedMessage> for PostedMessageV2 {
    fn from(message: VersionedPostedMessage) -> Self {
        match message {
            VersionedPostedMessage::V2(posted) => posted,
            VersionedPostedMessage::V1(posted) => PostedMessageV2 {
                payment: NearToken::from_near(0),
                premium: posted.premium,
                sender: posted.sender,
                text: posted.text,
            },
        }
    }
}
```

<br />

# Quickstart

## 1. Deploy the Contract

_In this example we will be using [NEAR CLI](https://github.com/near/near-cli)
to intract with the NEAR blockchain and the smart contract and [near-cli-rs](https://near.cli.rs)
which provides more control over interactions and has interactive menus for subcommands selection_

To build contract install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# from repo root
cd enum-updates/update
cargo near build
```

You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
# `update-migrate-rust-enum-updates.testnet` was used as example of <target-account-id>
# NEAR CLI
near deploy <target-account-id> ../../target/near/enums_update/enums_update.wasm
# near-cli-rs 
near contract deploy <target-account-id> use-file ../../target/near/enums_update/enums_update.wasm without-init-call network-config testnet sign-with-keychain send
```

In contrast with [the base update](../../basic-updates/update), here **no migration is needed**,
because the contract was already prepared to handle versioned messages!

<br />

## 2. Add a Message
```bash
# NEAR CLI
near call <target-account-id> add_message '{"text": "a message"}' --amount 0.1 --accountId <account>
# near-cli-rs 
near contract call-function as-transaction <target-account-id> add_message json-args '{"text": "a message"}' prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as <account> network-config testnet sign-with-keychain send
```

<br />

## 3. Retrieve the Messages
You will see that the old `V1` messages always have `payment: 0`, while the new ones keep track
of the payment

```bash
# NEAR CLI
near view <target-account-id> get_messages
```
