# Guest Book Contract - Self Update

The [base](../base) contract was modified, removing the `payments` field and including that information
in the `PostedMessage` structure.

```rust
pub struct PostedMessage {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub struct GuestBook {
    messages: Vector<PostedMessage>,
    manager: AccountId,
}
```

## 1. Build

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# from repo root
cd self-updates/update
cargo near build
```

## 2. Asking the Contract to Update Itself

_In this example we will be using [NEAR CLI](https://github.com/near/near-cli)
to intract with the NEAR blockchain and the smart contract and [near-cli-rs](https://near.cli.rs)
which provides more control over interactions and has interactive menus for subcommands selection_

The [base contract](../base/) implements a `update_contract` method that only the `manager` can call. That method takes
a compiled wasm as input and then:
1. Deploys it on itself.
2. Calls the `migrate` method on itself.

Lets call `update_contract` passing the new code ([./src](./src/)) using the [`manager-account`](../base/README.md#1-build-and-deploy-the-contract).

```bash
# `update-migrate-rust-self-updates.testnet` was used as example of <target-account-id>
# `update-migrate-rust-self-updates-manager.testnet` was used as example of <manager-account-id>
# near-cli-rs 
near contract call-function as-transaction <target-account-id> update_contract file-args ../../target/near/self_update/self_update.wasm prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as <manager-account-id> network-config testnet sign-with-keychain send
```
<br />

## 3. Retrieve the Stored Messages
`get_messages` will now return messages that include a `payment` field.

```bash
# NEAR CLI
near view <target-account-id> get_messages
```

`get_payments` will raise an error since the method does not exist anymore.

```bash
# raises an error since the method is gone
# NEAR CLI
near view <target-account-id> get_payments
```
