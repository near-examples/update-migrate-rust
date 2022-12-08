# Contract's Update & State Migration
[![](https://img.shields.io/badge/⋈%20Examples-Intermediate-orange)](https://docs.near.org/tutorials/welcome)
[![](https://img.shields.io/badge/Contract-rust-red)](https://docs.near.org/develop/contracts/anatomy)
[![](https://img.shields.io/badge/Frontend-None-gray)](https://docs.near.org/develop/integrate/frontend)
[![](https://img.shields.io/github/workflow/status/near-examples/update-migrate-rust/Tests/main?color=green&label=Tests)](https://github.com/near-examples/update-migrate-rust/actions/workflows/tests.yml)

Three examples on how to handle updates and [state migration](https://docs.near.org/develop/upgrade/migration):
1. [State Migration](./contracts/basic-updates/): How to implement a `migrate` method to migrate state between contract updates.
2. [State Versioning](./contracts/enum-updates/): How to use readily use versioning on a state, to simplify updating it later.
3. [Self Update](./contracts/self-updates/): How to implement a contract that can update itself.

<br />

## 1. [State Migration](./contracts/basic-updates/)
The examples at [./contracts/basic-updates](./contracts/basic-updates) show how to handle state-breaking changes
between contract updates.

It is composed by 2 contracts:
1. Base: A Guest Book were people can write messages.
2. Update: An update in which we remove a parameter and change the internal structure.

```rust
#[private]
#[init(ignore_state)]
pub fn migrate() -> Self {
  let old_state: OldState = env::state_read().expect("failed");
  let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

  // iterate through the messages of the previous state
  for (idx, posted) in old_state.messages.iter().enumerate() {
    // get the payment using the message index
    let payment = old_state.payments.get(idx as u64).unwrap_or(0);

    // Create a PostedMessage with the new format and push it
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

## 2. [State Versioning](./contracts/enum-updates/)
The example at [./contracts/enum-updates/](./contracts/enum-updates/) shows how to use
[Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) to implement versioning.

Versioning simplifies updating the contract since you only need to add a new new version of the structure.
All versions can coexist, thus you will not need to change previously existing structures. 

The example is composed by 2 contracts:
1. Base: The [guest-book](https://github.com/near-examples/guest-book-rust) contract using versioned `PostedMessages` (`PostedMessagesV1`).
2. Update: An update that adds a new version of `PostedMessages` (`PostedMessagesV2`).

```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedPostedMessage {
  V1(PostedMessageV1),
  V2(PostedMessageV2),
}

impl From<VersionedPostedMessage> for PostedMessageV2 {
  fn from(message: VersionedPostedMessage) -> Self {
    match message {
      VersionedPostedMessage::V2(posted) => posted,
      VersionedPostedMessage::V1(posted) => PostedMessageV2 {
        payment: if posted.premium { POINT_ONE } else { 0 },
        premium: posted.premium,
        sender: posted.sender,
        text: posted.text,
      },
    }
  }
}
```

<br />

## 3. [Self Update](./contracts/self-updates/)
The examples at [./contracts/self-updates](./contracts/self-updates) shows how to implement a contract
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
    0,
    CALL_GAS
  )
  .as_return()
}
```

<br />


# Quickstart

Clone this repository locally or [**open it in gitpod**](https://gitpod.io/#/github.com/near-examples/multiple-cross-contract-calls). Then follow these steps:

### 1. Install Dependencies
```bash
npm install
```

### 2. Test the Contract
Deploy your contract in a sandbox and simulate interactions from users.

```bash
npm test
```

---

# Learn More
1. Learn more on each contract's [README](./contract/README.md).
2. Check [**our documentation**](https://docs.near.org/develop/welcome).
