# Guest Book Contract

The smart contract stores messages, keeping track of how much money was deposited when adding the message.

```rust
#[payable]
pub fn add_message(&mut self, text: String) {
  let payment = env::attached_deposit();
  let premium = payment >= POINT_ONE;
  let sender = env::predecessor_account_id();

  let message = PostedMessage {
    premium,
    sender,
    text,
  };
  self.messages.push(&message);
  self.payments.push(&payment);
}
```

<br />

# Quickstart

## 1. Build and Deploy the Contract
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
# run from project-root/contracts
./deploy.sh
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

## 3. Retrieve the Stored Messages & Payments
`get_messages` and `get_payments` are read-only method (`view` method)

```bash
near view <dev-account> get_messages
near view <dev-account> get_payments
```

<br />

## 4. Continue in the Update Folder
Navigate to the [update](../update/) folder to continue