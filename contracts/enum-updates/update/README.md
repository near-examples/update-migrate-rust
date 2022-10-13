# Guest Book Contract - Versioned Messages Update

This examples adds a new version to the **versioned** messages of the [enum-base](../base/) contracts.

```rust
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub struct PostedMessageV2 {
    pub payment: u128,
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
                payment: 0,
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
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/enums_update.wasm
```

In contrast with [the base update](../../basic-updates/update), here **no migration is needed**,
because the contract was already prepared to handle versioned messages!

<br />

## 2. Add a Message
```bash
near call <dev-account> add_message '{"text": "another message"}' --amount 0.1 --accountId <account>
```

<br />

## 3. Retrieve the Messages
You will see that the old `V1` messages always have `payment: 0`, while the new ones keep track
of the payment

```bash
near view <dev-account> get_messages
```