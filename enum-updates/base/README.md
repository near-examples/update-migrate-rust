# Guest Book Contract - Versioned Messages

The smart contract stores **versioned** messages. This simplifies further updates of the contract.

```rust
// Versioned Messages
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub enum VersionedPostedMessage {
    V1(PostedMessageV1),
}

// Smart Contract
pub struct GuestBook {
    messages: Vector<VersionedPostedMessage>,
}

pub fn add_message(&mut self, text: String) {
    let payment = env::attached_deposit();
    let sender = env::predecessor_account_id();
    let premium = payment > POINT_ONE;
    let message = VersionedPostedMessage::V1(PostedMessageV1 {
        sender,
        premium,
        text,
    });
    self.messages.push(&message);
}
```

<br />

# Quickstart

## 1. Build and Deploy the Contract
Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# from repo root
cd enum-updates/base
cargo near build
```

Build and deploy: 

```bash
# `update-migrate-rust-enum-updates.testnet` was used as example of <target-account-id>
cargo near deploy <target-account-id> without-init-call network-config testnet sign-with-keychain send
```

## 2. How to interact?

_In this example we will be using [NEAR CLI](https://github.com/near/near-cli)
to intract with the NEAR blockchain and the smart contract and [near-cli-rs](https://near.cli.rs)
which provides more control over interactions and has interactive menus for subcommands selection_

### 1. Add a Message

```bash
# NEAR CLI
near call <target-account-id> add_message '{"text": "a message"}' --amount 0.1 --accountId <account>
# near-cli-rs 
near contract call-function as-transaction <target-account-id> add_message json-args '{"text": "a message"}' prepaid-gas '100.0 Tgas' attached-deposit '0.1 NEAR' sign-as <account> network-config testnet sign-with-keychain send
```

<br />

### 2. Retrieve the Messages
```bash
# NEAR CLI
near view <target-account-id> get_messages
# near-cli-rs 
near contract call-function as-read-only <target-account-id> get_messages json-args {} network-config testnet now
```

<br />

### 3. Continue in the Update Folder
Navigate to the [update](../update/) folder to continue
