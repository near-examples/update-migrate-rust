# State Migration
[![](https://img.shields.io/badge/⋈%20Examples-Intermediate-orange)](https://docs.near.org/tutorials/welcome)
[![](https://img.shields.io/badge/Contract-rust-red)](https://docs.near.org/develop/contracts/anatomy)
[![](https://img.shields.io/badge/Frontend-None-gray)](https://docs.near.org/develop/integrate/frontend)
[![](https://img.shields.io/badge/Testing-passing-green)](https://docs.near.org/develop/integrate/frontend)


Two examples on how to handle [state migration](https://docs.near.org/develop/upgrade/migration):
1. State Migration: How to implement a `migrate` method to migrate state between contract updates.
2. State Versions: How to use readily use versioning on a state, to simplify updating it later.

<br />

## 1. State Migration
The examples at [./contracts/basic-updates](./contracts/basic-updates) show how to handle state-breaking changes
between contract updates.

It is composed by 3 contracts:
1. Base: The [guest-book example](https://github.com/near-examples/guest-book-rust) contract.
2. First update: A first update in which we add a new parameter to the state.
2. Second update: A second update in which we remove a parameter and change an internal structure.

```rust
#[private]
#[init(ignore_state)]
pub fn migrate() -> Self {
  // Read the current state stored
  let old_state: OldState = env::state_read().expect("failed");

  // Iterate through the state, updating the structures as needed
  let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

  for (idx, posted) in old_state.messages.iter().enumerate() {
    let payment = old_state.payments.get(idx as u64).unwrap_or(0);

    new_messages.push(
      &PostedMessage {
        payment,
        premium: posted.premium,
        sender: posted.sender,
        text: posted.text,
      }
    )
  }

  // Return new state
  Self { messages: new_messages }
}
```

<br />

## 2. State Versioning
If you think that one of your inner structures will change a lot you can plan ahead and implement versioning.

Versioning is simply using [Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) to express
"We are storing different versions of the same structure".

Versioning simplifies updating the contract since you only need to add a new new version of the structure.
All versions can coexist, thus you will not need to change previously existing structures. 

The example is composed by 2 contracts:
1. Base: The [guest-book](https://github.com/near-examples/guest-book-rust) contract using versioned `PostedMessages` (`PostedMessagesV1`).
2. First update: A first update that adds a new version of `PostedMessages` (`PostedMessagesV2`).

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
