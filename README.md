# near-bridge
This app was initialized with [create-near-app]

## Quick Start
### Deploy

Every smart contract in NEAR has its [own associated account][NEAR accounts].
When you run `sh deploy.sh`, your smart contract gets deployed to the live NEAR TestNet with a temporary dev account.
When you're ready to make it permanent, here's how:

#### Step 0: Install near-cli (optional)
[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `npm install`, but for best ergonomics you may want to install it globally:

```commandline
$ npm install --global near-cli
```

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

#### Step 1: Create an account for the contract
Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `near-blank-project.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `near-blank-project.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:
```commandline
$ near login
```

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):
```commandline
$ near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet
```

## Step 2: Build contracts
```commandline
$ sh ./build.sh
```

#### Step 3: Deploy the contract
Use the CLI to deploy the contract to TestNet with your account ID.
Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.
```commandline
$ near deploy --accountId near-blank-project.YOUR-NAME.testnet --wasmFile PATH_TO_WASM_FILE
```

### Test
To test contracts, run:
```commandline
$ sh test.sh
```

### Troubleshooting
On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.

### License
[MIT](./LICENSE)

[NEAR accounts]: https://docs.near.org/concepts/basics/account
[near-cli]: https://github.com/near/near-cli
[create-near-app]: https://github.com/near/create-near-app
[NEAR Wallet]: https://wallet.testnet.near.org/
