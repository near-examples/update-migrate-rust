# State Migration Example

This example shows how to do a [state migration](https://docs.near.org/develop/upgrade/migration). It is composed by
4 contracts:
1. Base: The [guest-book example](https://github.com/near-examples/guest-book-rust) contract.
2. First update: A first example in which we add a new parameter to the state.
2. Second update: A second example in which we change a structure that is saved.
2. Third update: A last example in which we use Enums to maintain two versions of the state at the same time.

<br />

# Quickstart

Clone this repository locally or [**open it in gitpod**](https://gitpod.io/#/github.com/near-examples/multiple-cross-contract-calls). Then follow these steps:

### 1. Install Dependencies
```bash
npm install
```

### 2. Test the Contract
Deploy your contract in a sandbox and simulate interactions from users.

```bash
npm test
```

---

# Learn More
1. Learn more about the contract through its [README](./contract/README.md).
2. Check [**our documentation**](https://docs.near.org/develop/welcome).
