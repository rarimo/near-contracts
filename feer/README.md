# commission

## Deploy
```commandline
$ near login
$ near create-account commission.master-account.testnet --masterAccount master-account.testnet --initialBalance 10
$ near deploy --wasmFile target/wasm32-unknown-unknown/release/commission.wasm --accountId commission.master-account.testnet
```

## Initialization
```commandline
$ near call commission.master-account.testnet new '{}' --accountId master-account.testnet
```

## More about
[Redeploy](https://www.near-sdk.io/upgrading/production-basics)

