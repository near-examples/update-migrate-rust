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
  await guestBook.deploy("./contracts/target/wasm32-unknown-unknown/release/enums_base.wasm");

  // add messages
  await guestBook.call(guestBook, "add_message", { text: "hello" }, { attachedDeposit: NEAR.parse('0.09') });
  await alice.call(guestBook, "add_message", { text: "bye" }, { attachedDeposit: NEAR.parse('1') });

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

test("version update", async (t) => {
  const { guestBook, alice } = t.context.accounts;

  await guestBook.deploy("./contracts/target/wasm32-unknown-unknown/release/enums_update.wasm");

  // gets the version one messages
  const msgs = await guestBook.view("get_messages");

  const expected = [
    { payment: 0, premium: false, sender: guestBook.accountId, text: "hello" },
    { payment: 0, premium: true, sender: alice.accountId, text: "bye" },
  ];

  t.deepEqual(msgs, expected)

  // gets version one and two messages
  await alice.call(guestBook, "add_message", { text: "howdy" }, { attachedDeposit: NEAR.parse('1') });

  const new_msgs = await guestBook.view("get_messages");

  const new_expected = [
    { payment: 0, premium: false, sender: guestBook.accountId, text: "hello" },
    { payment: 0, premium: true, sender: alice.accountId, text: "bye" },
    { payment: 1e24, premium: true, sender: alice.accountId, text: "howdy" },
  ];

  t.deepEqual(new_msgs, new_expected)

});
