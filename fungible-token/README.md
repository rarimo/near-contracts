# near-bridge-fungible-token

## Deploy
```commandline
$ near login
$ near create-account fungible_token.master-account.testnet --masterAccount master-account.testnet --initialBalance 10
$ near deploy --wasmFile target/wasm32-unknown-unknown/release/fungible_token.wasm --accountId fungible_token.master-account.testnet
```

## Initialization
```commandline
$ near call fungible_token.master-account.testnet new '{"owner_id":"'$ID'","total_supply":"100000000000","supply":[{"account":"'$ID'","supply":"100000000000"}],"metadata":{"spec":"ft-1.0.0","name":"Rarimo","symbol":"Rarimo Bridge Test Token","decimals":8}}' --accountId master-account.testnet
```
[Metadata specification](https://nomicon.io/Standards/FungibleToken/Metadata.html)

Supply specification: JSON array with the following structs - Example: {"account":"master-account.testnet","supply":"1000000000000000"}.
The sum of supplies should be equal to total_supply.

## Usage
Get metadata
```commandline
$ near view fungible_token.master-account.testnet ft_metadata
```

Balance
```commandline
$ near view fungible_token.master-account.testnet ft_balance_of '{"account_id": "master-account.testnet"}'
```

## More about
[Token standard](https://nomicon.io/Standards/FungibleToken/Core.html)
[Redeploy](https://www.near-sdk.io/upgrading/production-basics)

