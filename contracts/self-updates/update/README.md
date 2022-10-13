# Guest Book Contract - Self Update

The [base](../base) contract was modified, removing the `payments` field and including that information
in the `PostedMessage` structure.

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

## 1. Asking the Contract to Update Itself

The [base contract](../base/) implements a `update_contract` method that only the `manager` can call. That method takes
a compiled wasm as input and then:
1. Deploys in on itself.
2. Calls the `migrate` method on itself.

Lets call `update_contract` passing the new code ([./src](./src/)) using the [`manager-account`](../base/README.md#1-build-and-deploy-the-contract).

```bash
# run from project-root/contracts
NEW_CONTRACT_BYTES=`cat ./target/wasm32-unknown-unknown/release/self_update.wasm | base64`
near call <dev-account> update_contract "$NEW_CONTRACT_BYTES" --base64 --accountId <manager-account> --gas 300000000000000
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