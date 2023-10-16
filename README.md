# near-bridge

Rarimo Near Protocol Smart Contracts

## Build contracts
```commandline
$ sh ./build.sh
```

## Test
To test contracts, run:
```commandline
$ sh test.sh
```

## Deployed Contracts

### Testnet
#### Bridge

| **Contract** | **Near Account Address** | **Hex-Encoded Account Address**                | 
|--------------|--------------------------|------------------------------------------------|
| **Bridge**   | `bridge.rarimo.testnet`  | `0x6272696467652e726172696d6f2e746573746e6574` |
| **Feer**     | `fee.rarimo.testnet`     | `0x6665652e726172696d6f2e746573746e6574`       |

#### Fungible Tokens

| **Contract**            | **Near Account Address**      | **Hex-Encoded Account Address**                            | **Symbol** | **Decimals** |
|-------------------------|-------------------------------|------------------------------------------------------------|------------|--------------|
| USDC                    | `usdc.rarimo.testnet`         | `0x757364632e726172696d6f2e746573746e6574`                 | USDC       | 6            |
| Wrapped Goerli USDC     | `wusdc_goerli.rarimo.testnet` | `0x77757364635f676f65726c692e726172696d6f2e746573746e6574` | WGUSDC     | 6            |
| Wrapped Fuji USDC       | `wusdc_fuji.rarimo.testnet`   | `0x77757364635f66756a692e726172696d6f2e746573746e6574`     | WFUSDC     | 6            |
| Wrapped Solana USDC     | `wusdc_sol.rarimo.testnet`    | `0x77757364635f736f6c2e726172696d6f2e746573746e6574`       | WSUSDC     | 6            |
| Wrapped Solana SOL      | `wsol.rarimo.testnet`         | `0x77736f6c2e726172696d6f2e746573746e6574`                 | WSOL       | 9            |
| Wrapped Goerli Ethereum | `weth_goerli.rarimo.testnet`  | `0x776574685f676f65726c692e726172696d6f2e746573746e6574`   | WGETH      | 18           |
| Wrapped Fuji Avax       | `wavax_fuji.rarimo.testnet`   | `0x77617661785f66756a692e726172696d6f2e746573746e6574`     | WFAVAX     | 18           |
| Wrapped Rarimo RMO      | `wrmo.rarimo.testnet`         | `0x77726d6f2e726172696d6f2e746573746e6574`                 | WRMO       | 6            |


### Troubleshooting
On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.

### License
[MIT](./LICENSE)

[NEAR accounts]: https://docs.near.org/concepts/basics/account
[near-cli]: https://github.com/near/near-cli
[create-near-app]: https://github.com/near/create-near-app
[NEAR Wallet]: https://wallet.testnet.near.org/
