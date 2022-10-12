import { Worker, NEAR, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // deploy contract
  const root = worker.rootAccount;

  // some test accounts
  const alice = await root.createSubAccount("alice");
  const guestBook = await root.createSubAccount("guesbook");

  // Get wasm file path from package.json test script in folder above
  await guestBook.deploy("./contracts/target/wasm32-unknown-unknown/release/base.wasm");

  // add messages
  await guestBook.call(guestBook, "add_message", { text: "hello" }, { attachedDeposit: NEAR.parse('0.09') });
  await alice.call(guestBook, "add_message", { text: "bye" }, { attachedDeposit: NEAR.parse('0.1') });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, guestBook, alice };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("by default it return the two messages and their payments", async (t) => {
  const { guestBook, alice } = t.context.accounts;

  const msgs = await guestBook.view("get_messages");
  const payments = await guestBook.view("get_payments");

  const expected_msgs = [
    { premium: false, sender: guestBook.accountId, text: "hello" },
    { premium: true, sender: alice.accountId, text: "bye" },
  ];

  const expected_payments = [NEAR.parse("0.09").toString(), NEAR.parse("0.1").toString()];

  t.deepEqual(msgs, expected_msgs);
  t.deepEqual(payments, expected_payments);
});

test("the migration removes payments and updates PostedMessages", async (t) => {
  const { guestBook, alice } = t.context.accounts;

  await guestBook.deploy("./contracts/target/wasm32-unknown-unknown/release/update.wasm");
  await guestBook.call(guestBook, "migrate", {});

  await t.throwsAsync(guestBook.view("get_payments"));

  const msgs = await guestBook.view("get_messages");

  const expected = [
    { payment: 9e+22, premium: false, sender: guestBook.accountId, text: "hello" },
    { payment: 1e+23, premium: true, sender: alice.accountId, text: "bye" },
  ];

  t.deepEqual(msgs, expected)
});