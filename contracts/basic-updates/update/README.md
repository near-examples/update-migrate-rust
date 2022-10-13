# Guest Book Contract

The [base](../base) contract was modified, removing the `payments` field
and including that information in the `PostedMessage` structure.

```rust
pub struct PostedMessage {
  pub payment: u128, 
  pub premium: bool, 
  pub sender: AccountId,
  pub text: String
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
  let old_state: OldState = env::state_read().expect("failed");
  let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

  // iterate through the messages of the previous state
  for (idx, posted) in old_state.messages.iter().enumerate() {
    // get the payment using the message index
    let payment = old_state.payments.get(idx as u64).unwrap_or(0);

    // Create a PostedMessage with the new format and push it
    new_messages.push(
      &PostedMessage {
        payment,
        premium: posted.premium,
        sender: posted.sender,
        text: posted.text,
      }
    )
  }

  // return the new state
  Self { messages: new_messages }
}
```

<br />

# Upgrading Base Contract

## 1. Deploy & Migrate State
You can deploy the updated contract by running:

```bash
# deploy the updated contract
near deploy <dev-account> --wasmFile target/wasm32-unknown-unknown/release/update.wasm

# run this command to see the "Cannot deserialize..." error
near view <dev-account> get_messages

# Ask the contract to migrate the state
near call <dev-account> migrate {} --accountId <dev-account>
```

#### Deploying and Migrating
You can actually deploy the contract and migrate the state in one line:
```bash
near deploy <dev-account> --wasmFile target/wasm32-unknown-unknown/release/update.wasm --initFunction migrate --initArgs {}
```

<br />

## 2. Retrieve the Stored Messages
`get_messages` will now return messages that include a `payment` field.

```bash
near view <dev-account> get_messages
```

`get_payments` will raise an error since the method does not exist anymore.

```bash
# raises an error since the method is gone
near view <dev-account> get_payments
```