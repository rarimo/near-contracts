source <(sed -E -n 's/[^#]+/export &/ p' .env)

echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Account ID:" "${BGreen}$ID"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$BRIDGE"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$BRIDGE" | xxd -p -u)"
echo "${BYellow}Signer Public Key:" "${BGreen}$SIGNER_PUB_KEY"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"
echo "${BYellow}Contract:" "${BGreen}$FEE"
echo "${BYellow}Hex:" "${BGreen}0x$(printf '%s' "$FEE" | xxd -p -u)"
echo "${BWhite}–––––––––––––––––––––––––––––––––––––––––––––––––––"

# delete existing accounts
near delete $BRIDGE $ID --accountId $ID
near delete $FEE $ID --accountId $ID

# create accounts
near create-account $BRIDGE --masterAccount $ID --initialBalance 5
near create-account $FEE --masterAccount $ID --initialBalance 5

# deploy contracts
near deploy --wasmFile .././res/bridge.wasm --accountId $BRIDGE
near deploy --wasmFile .././res/feer.wasm --accountId $FEE

# initialize contracts
CHAIN="Near"

near call $BRIDGE new '{"signer": "'$SIGNER_PUB_KEY'", "fee_contract": "'$FEE'", "chain": "'$CHAIN'"}' --accountId $ID
near call $FEE new '{"chain": "'$CHAIN'", "bridge_addr": "'$BRIDGE'", "tokens": [{"token_type": "Native", "fee": "1"}, {"token_addr": "'$USDC'", "token_type": "FT", "fee": "1"}]}' --accountId $ID
near call $USDC storage_deposit '{"account_id": "'$FEE'"}' --accountId $ID --amount 0.00125
near call $FEE storage_deposit '{"account_id": "'$ID'"}' --accountId $ID --amount 0.00663
