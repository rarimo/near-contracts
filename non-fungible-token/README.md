# near-bridge-non-fungible-token

## Deploy
```commandline
$ near login
$ near create-account non_fungible_token.master-account.testnet --masterAccount master-account.testnet --initialBalance 10
$ near deploy --wasmFile target/wasm32-unknown-unknown/release/non_fungible_token.wasm --accountId non_fungible_token.master-account.testnet
```

## Initialization
```commandline
$ near call non_fungible_token.master-account.testnet new '{"owner_id":"'$ID'","metadata":{"spec":"nft-1.0.0","name":"Rarimo Bridge NFT Test Collection","symbol":"RNFT"}}' --accountId master-account.testnet
```
[Metadata specification](https://nomicon.io/Standards/NonFungibleToken/Metadata.html)

Supply specification: JSON array with the following structs - Example: {"account":"master-account.testnet","supply":"1000000000000000"}.
The sum of supplies should be equal to total_supply.

## Usage
Get contract metadata
```commandline
$ near view non_fungible_token.master-account.testnet nft_metadata
```

Mint NFT
```commandline
$ near call non_fungible_token.master-account.testnet nft_mint '{"token_id":"1","receiver_id":"master-account.testnet","token_metadata":{"title":"Rarimo Bridge NFT#1","description":"Rarimo Bridge Test Collection NFT#1","media":"https://bafkreiblbldzupel5ci36xhaw2kpci4q53yvjnq55ueqawep6nigjggcze.ipfs.nftstorage.link/","copies":1}}' --accountId non_fungible_token.master-account.testnet
```

Get NFTs by owner
```commandline
$ near view non_fungible_token.master-account.testnet nft_tokens_for_owner '{"account_id":"master-account.testnet"}'
```


## More about
[Token standard](https://nomicon.io/Standards/NonFungibleToken/Core.html)
[Redeploy](https://www.near-sdk.io/upgrading/production-basics)

