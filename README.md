# Contract's Update & State Migration
[![](https://img.shields.io/badge/⋈%20Examples-Intermediate-orange)](https://docs.near.org/tutorials/welcome)
[![](https://img.shields.io/badge/Contract-rust-red)](https://docs.near.org/develop/contracts/anatomy)
[![](https://img.shields.io/badge/Frontend-None-gray)](https://docs.near.org/develop/integrate/frontend)
[![](https://img.shields.io/github/workflow/status/near-examples/update-migrate-rust/Tests/main?color=green&label=Tests)](https://github.com/near-examples/update-migrate-rust/actions/workflows/tests.yml)

Three examples on how to handle updates and [state migration](https://docs.near.org/develop/upgrade/migration):
1. [State Migration](./basic-updates/): How to implement a `migrate` method to migrate state between contract updates.
2. [State Versioning](./enum-updates/): How to use readily use versioning on a state, to simplify updating it later.
3. [Self Update](./self-updates/): How to implement a contract that can update itself.

<br />

## 1. [State Migration](./basic-updates/)
The examples at [./basic-updates](./basic-updates) show how to handle state-breaking changes
between contract updates.

It is composed by 2 contracts:
1. Base: A Guest Book where people can write messages.
2. Update: An update in which we remove a parameter and change the internal structure.

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

## 2. [State Versioning](./enum-updates/)
The example at [./enum-updates/](./enum-updates/) shows how to use
[Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) to implement versioning.

Versioning simplifies updating the contract since you only need to add a new new version of the structure.
All versions can coexist, thus you will not need to change previously existing structures. 

The example is composed by 2 contracts:
1. Base: The [guest-book](https://github.com/near-examples/guest-book-rust) contract using versioned `PostedMessages` (`PostedMessagesV1`).
2. Update: An update that adds a new version of `PostedMessages` (`PostedMessagesV2`).

```rust
#[near(serializers=[borsh])]
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

## 3. [Self Update](./self-updates/)
The examples at [./self-updates](./self-updates) shows how to implement a contract
that can update itself.

It is composed by 2 contracts:
1. Base: A Guest Book were people can write messages, implementing a `update_contract` method.
2. Update: An update in which we remove a parameter and change the internal structure.

```rust
pub fn update_contract(&self) -> Promise {
    // Check the caller is authorized to update the code
    assert!(
        env::predecessor_account_id() == self.manager,
        "Only the manager can update the code"
    );

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
            CALL_GAS,
        )
        .as_return()
}
```

<br />


# Quickstart

Clone this repository locally or open it in a codespace using the green `<> Code` button above. Then follow these steps:

### 0. Test the Contract
Deploy your contract in a sandbox and simulate interactions from users.

```bash
cargo test --workspace
```

### 1. Examples' cli-s versions

Commands in each contract's `README` are valid for following versions of programs.

```bash
# NEAR CLI
❯ near --version
4.0.10

# near-cli-rs 
❯ near --version
near-cli-rs 0.8.1

❯ cargo near --version
cargo-near-near 0.6.1
```

`NOTE`: default devcontainer for Codespaces contains only `near-cli-rs` and `cargo-near` commands. 

### 2. Accounts for deploying contracts from these examples can be created by:  

```bash
# NEAR CLI
near create-account <target-account-id> --useFaucet
# near-cli-rs 
near account create-account sponsor-by-faucet-service <target-account-id> autogenerate-new-keypair save-to-keychain network-config testnet create
```

`NOTE`: default devcontainer for Codespaces only supports `save-to-legacy-keychain` option instead of `save-to-keychain` of `near-cli-rs`. 


---

# Learn More
1. Learn more on each contract's `README`.
2. Check [**our documentation**](https://docs.near.org/develop/welcome).
