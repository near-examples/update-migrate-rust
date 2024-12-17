# Guest Book Contract

The [v2](../v2) contract was modified, removing the `payments` field and including that information in the `PostedMessage` structure.

```rust
pub struct PostedMessage {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub struct GuestBook {
  messages: Vector<PostedMessage>,
  owner: AccountId
}
```

If we deploy this contract on top of the [v2](../v2/) one and call any method we again will get the error:

```
panicked at 'Cannot deserialize the contract state.: ... }',
```

This is because the new contract expects to find `PostedMessages` with 4 fields (`payment`, `premium`, `sender`, `text`)
but the saved messages only have 3 fields (they lack the `payment` field).

In order to fix this problem we need to `migrate` the state, i.e. iterate through the current saved messages transforming them to the new version.

We're getting rid of Vector of `payments` entirely, so just removing the field from the state won't cut it. The data is spread across multiple keys in the storage, so to clean it up properly, we need to explicitly call `payments::clear()`.

```rust
impl GuestBook {
    // Upgrades from V2 to V3
    fn unsafe_add_payment_to_message() {
        let GuestBookV2 {
            messages: old_messages,
            mut payments,
            owner,
        } = env::state_read().unwrap();

        let default_payment = NearToken::from_yoctonear(0);

        // New messages must be written to storage
        let mut messages = Vector::new(StorageKey::Messages);

        for (idx, old_message) in old_messages.iter().enumerate() {
            let payment = payments.get(idx as u32).unwrap_or(&default_payment);

            messages.push(PostedMessageV3 {
                premium: old_message.premium.clone(),
                sender: old_message.sender.clone(),
                text: old_message.text.clone(),
                payment: payment.clone(),
            });
        }

        // Payments must be removed from storage
        payments.clear();

        env::state_write(&GuestBookV3 { messages, owner });
    }

    fn migration_done() {
        near_sdk::log!("Migration done.");
        env::value_return(b"\"done\"");
    }

    fn needs_migration() {
        env::value_return(b"\"needs-migration\"");
    }

    pub fn unsafe_migrate() {
        near_sdk::assert_self();
        let current_version = state_version_read();
        near_sdk::log!("Migrating from version: {:?}", current_version);
        match current_version {
            StateVersion::V1 => {
                GuestBook::unsafe_add_owner();
                state_version_write(&StateVersion::V2);
            }
            StateVersion::V2 => {
                GuestBook::unsafe_add_payment_to_message();
                state_version_write(&StateVersion::V3);
            }
            _ => {
                return GuestBook::migration_done();
            }
        }
        GuestBook::needs_migration();
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
cd advanced-multi-version-updates/v2
cargo near build --no-docker
```

You can deploy the updated contract by running:

```bash
# `update-migrate-rust-advanced-multiversion-updates.testnet` was used as example of <target-account-id>
cargo near deploy --no-docker <target-account-id> without-init-call network-config testnet sign-with-keychain send
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
# near-cli-rs (may be useful to specify more gas for large state migrations)
near contract call-function as-transaction <target-account-id> unsafe_migrate json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as <target-account-id> network-config testnet sign-with-keychain send
```

#### Deploying and Migrating

You can actually deploy the contract and migrate the state in one line:

```bash
# near-cli-rs (may be useful to specify more gas for large state migrations)
cargo near deploy --no-docker <target-account-id> with-init-call unsafe_migrate json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

<br />

## 2. Retrieve the Stored Messages

`get_messages` will now return messages that include a `payment` field.

```bash
# NEAR CLI
near view <target-account-id> get_messages
# near-cli-rs
near contract call-function as-read-only <target-account-id> get_messages json-args {} network-config testnet now
```

`get_payments` will raise the error `MethodResolveError(MethodNotFound)` since the method does not exist anymore.

```bash
# NEAR CLI
near view <target-account-id> get_payments
# near-cli-rs
near contract call-function as-read-only <target-account-id> get_payments json-args {} network-config testnet now
```
