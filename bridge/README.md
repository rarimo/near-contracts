# bridge

## Deploy
```commandline
$ near login
$ near create-account bridge.master-account.testnet --masterAccount master-account.testnet --initialBalance 10
$ near deploy --wasmFile target/wasm32-unknown-unknown/release/bridge.wasm --accountId bridge.master-account.testnet
```

## Initialization
```commandline
$ near call bridge.master-account.testnet new '{"signer":"...'", "chain": "Near"}' --accountId master-account.testnet
```

## More about
[Redeploy](https://www.near-sdk.io/upgrading/production-basics)

