CONTRACT_BYTES=`cat ./res/bridge.wasm | base64`
ARGS='{"code": "'$CONTRACT_BYTES'" ,"signature": "", "recovery_id": 0}'
UPDATE_ARGS=`echo $ARGS | base64`

near call bridge.rarimo.testnet update_contract "$UPDATE_ARGS" --base64 --accountId $ID --gas 300000000000000
near view bridge.rarimo.testnet test_update
