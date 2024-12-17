# Guest Book Contract

The [v1](../v1) contract was modified, adding `owner` field and view function `get_owner` to retrieve the value.

```rust
pub struct GuestBook {
  messages: Vector<PostedMessage>,
  payments: Vector<NearToken>,
  owner: AccountId
}
```

If we deploy this contract on top of the [v1](../v1/) one and call any method we will get the error:

```
panicked at 'Cannot deserialize the contract state.: ... }',
```

This is because the new contract expects to find metadata about 3 fields (`messages`, `payments`, `owner`) in the contract state, but the current contract only has 2 fields (it lacks the `owner` field).

In order to fix this problem we need to `migrate` the state, i.e. place some account address under `owner` field.

```rust
impl GuestBook {
    // Upgrades from V1 to V2
    fn unsafe_add_owner() {
        let GuestBookV1 { messages, payments } = env::state_read().unwrap();
        let owner = AccountId::from_str("bob.near").unwrap();

        env::state_write(&GuestBookV2 {
            messages,
            payments,
            owner,
        });
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

`get_messages` will now return messages.

```bash
# NEAR CLI
near view <target-account-id> get_messages
# near-cli-rs
near contract call-function as-read-only <target-account-id> get_messages json-args {} network-config testnet now
```

And `get_owner` will return the owner we've set, in our case it's `bob.near`.

```bash
# NEAR CLI
near view <target-account-id> get_owner
# near-cli-rs
near contract call-function as-read-only <target-account-id> get_owner json-args {} network-config testnet now
```

<br />

### 3. Continue in the V3 Folder

Navigate to the [v3](../v3/) folder to continue
