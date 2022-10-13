# Guest Book Contract - Self Update

A [Guest Book Contract](../../basic-updates/base/) that can self-update.

This contract has a `update_contract` that takes as input a wasm file and deploys it on itself.

```rust
pub fn update_contract(&self) -> Promise {
  // Check the caller is authorized to update the code
  assert!(env::predecessor_account_id() == self.manager, "Only the manager can update the code");

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

## 1. Build and Deploy the Contract
Compile, deploy, and initialize the contract setting the `manager`, this is the account that will be able
to trigger the code update.

```bash
# build all examples, run from project-root/contracts
./build.sh

# delete the project-root/contracts/neardev folder if present
# rm -rf ./neardev

# deploy enum base contract
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/self_base.wasm --initFunction init --initArgs '{"manager":"<manager-account>"}'
```

Once finished, check the `neardev/dev-account` file to find in which address the contract was deployed:

```bash
cat ./neardev/dev-account # e.g. dev-X-Y
```
<br />

## 2. Lock the Account
Check the account's full-access key and remove it from the account, thus leaving it locked:

```bash
near keys <dev-account>
# result: [access_key: {"nonce": ..., "public_key": '<key>'}]

near delete-key <dev-account> '<key>'
```

<br />

## 3. Add a Message
```bash
near call <dev-account> add_message '{"text": "a message"}' --amount 0.1 --accountId <account>
```

<br />

## 4. Retrieve the Stored Messages & Payments
`get_messages` and `get_payments` are read-only method (`view` method)

```bash
near view <dev-account> get_messages
near view <dev-account> get_payments
```

<br />

## 5. Continue in the Update Folder
Navigate to the [update](../update/) folder to continue