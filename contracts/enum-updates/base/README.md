# Guest Book Contract - Versioned Messages

The smart contract stores **versioned** messages. This simplifies further updates of the contract.

```rust
// Versioned Messages
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

pub enum VersionedPostedMessage {
    V1(PostedMessageV1),
}

// Smart Contract
pub struct GuestBook {
    messages: Vector<VersionedPostedMessage>,
}

pub fn add_message(&mut self, text: String) {
    let payment = env::attached_deposit();
    let sender = env::predecessor_account_id();
    let premium = payment > POINT_ONE;
    let message = VersionedPostedMessage::V1(PostedMessageV1 {
        sender,
        premium,
        text,
    });
    self.messages.push(&message);
}
```

<br />

# Quickstart

## 1. Build and Deploy the Contract
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
# build all examples, run from project-root/contracts
./build.sh

# delete the project-root/contracts/neardev folder if present
# rm -rf ./neardev

# deploy enum base contract
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/enums_base.wasm
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account # e.g. dev-X-Y
```
<br />

## 2. Add a Message
```bash
near call <dev-account> add_message '{"text": "a message"}' --amount 0.1 --accountId <account>
```

<br />

## 3. Retrieve the Messages
```bash
near view <dev-account> get_messages
```

<br />

## 4. Continue in the Update Folder
Navigate to the [update](../update/) folder to continue