source <(sed -E -n 's/[^#]+/export &/ p' .env)

echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Account ID:" "${BGreen}$ID"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$USDC"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$USDC" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$USDC_NAME"
echo "${BYellow}Symbol:" "${BGreen}$USDC_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$USDC_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$USDC_GOERLI"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$USDC_GOERLI" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$USDC_GOERLI_NAME"
echo "${BYellow}Symbol:" "${BGreen}$USDC_GOERLI_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$USDC_GOERLI_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$USDC_FUJI"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$USDC_FUJI" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$USDC_FUJI_NAME"
echo "${BYellow}Symbol:" "${BGreen}$USDC_FUJI_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$USDC_FUJI_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$USDC_SOL"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$USDC_SOL" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$USDC_SOL_NAME"
echo "${BYellow}Symbol:" "${BGreen}$USDC_SOL_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$USDC_SOL_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$SOL"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$SOL" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$SOL_NAME"
echo "${BYellow}Symbol:" "${BGreen}$SOL_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$SOL_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$ETH_GOERLI"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$ETH_GOERLI" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$ETH_GOERLI_NAME"
echo "${BYellow}Symbol:" "${BGreen}$ETH_GOERLI_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$ETH_GOERLI_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$AVAX_FUJI"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$AVAX_FUJI" | xxd -p)"
echo "${BYellow}Name:" "${BGreen}$AVAX_FUJI_NAME"
echo "${BYellow}Symbol:" "${BGreen}$AVAX_FUJI_SYMBOL"
echo "${BYellow}Decimals:" "${BGreen}$AVAX_FUJI_DECIMALS"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"

# delete created accounts
near delete $USDC $ID --accountId $ID
near delete $USDC_GOERLI $ID --accountId $ID
near delete $USDC_FUJI $ID --accountId $ID
near delete $USDC_SOL $ID --accountId $ID
near delete $SOL $ID --accountId $ID
near delete $ETH_GOERLI $ID --accountId $ID
near delete $AVAX_FUJI $ID --accountId $ID

# create account
near create-account $USDC --masterAccount $ID --initialBalance 2
near create-account $USDC_GOERLI --masterAccount $ID --initialBalance 2
near create-account $USDC_FUJI --masterAccount $ID --initialBalance 2
near create-account $USDC_SOL --masterAccount $ID --initialBalance 2
near create-account $SOL --masterAccount $ID --initialBalance 2
near create-account $ETH_GOERLI --masterAccount $ID --initialBalance 2
near create-account $AVAX_FUJI --masterAccount $ID --initialBalance 2

# deploy
near deploy --wasmFile .././res/fungible_token.wasm --accountId $USDC
near deploy --wasmFile .././res/fungible_token.wasm --accountId $USDC_GOERLI
near deploy --wasmFile .././res/fungible_token.wasm --accountId $USDC_FUJI
near deploy --wasmFile .././res/fungible_token.wasm --accountId $USDC_SOL
near deploy --wasmFile .././res/fungible_token.wasm --accountId $SOL
near deploy --wasmFile .././res/fungible_token.wasm --accountId $ETH_GOERLI
near deploy --wasmFile .././res/fungible_token.wasm --accountId $AVAX_FUJI

# init
near call $USDC new '{"owner_id":"'$ID'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$USDC_NAME'","symbol":"'$USDC_SYMBOL'","decimals":'$USDC_DECIMALS'}}' --accountId $ID
near call $USDC_GOERLI new '{"owner_id":"'$BRIDGE'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$USDC_GOERLI_NAME'","symbol":"'$USDC_GOERLI_SYMBOL'","decimals":'$USDC_GOERLI_DECIMALS'}}' --accountId $ID
near call $USDC_FUJI new '{"owner_id":"'$BRIDGE'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$USDC_FUJI_NAME'","symbol":"'$USDC_FUJI_SYMBOL'","decimals":'$USDC_FUJI_DECIMALS'}}' --accountId $ID
near call $USDC_SOL new '{"owner_id":"'$BRIDGE'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$USDC_SOL_NAME'","symbol":"'$USDC_SOL_SYMBOL'","decimals":'$USDC_SOL_DECIMALS'}}' --accountId $ID
near call $SOL new '{"owner_id":"'$BRIDGE'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$SOL_NAME'","symbol":"'$SOL_SYMBOL'","decimals":'$SOL_DECIMALS'}}' --accountId $ID
near call $ETH_GOERLI new '{"owner_id":"'$BRIDGE'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$ETH_GOERLI_NAME'","symbol":"'$ETH_GOERLI_SYMBOL'","decimals":'$ETH_GOERLI_DECIMALS'}}' --accountId $ID
near call $AVAX_FUJI new '{"owner_id":"'$BRIDGE'","total_supply":"0","metadata":{"spec":"ft-1.0.0","name":"'$AVAX_FUJI_NAME'","symbol":"'$AVAX_FUJI_SYMBOL'","decimals":'$AVAX_FUJI_DECIMALS'}}' --accountId $ID

near call $USDC storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
near call $USDC_GOERLI storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
near call $USDC_FUJI storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
near call $USDC_SOL storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
near call $SOL storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
near call $ETH_GOERLI storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
near call $AVAX_FUJI storage_deposit '{"account_id": "'$BRIDGE'"}' --accountId $ID --amount 0.00125
