source <(sed -E -n 's/[^#]+/export &/ p' .env)

# deposit to USDC contract for storage
near call $USDC storage_deposit '{"account_id": "'$ID'"}' --accountId $ID --amount 0.00125

# deposit to feer contract for storage
near call $FEE storage_deposit '{"account_id": "'$ID'"}' --accountId $ID --amount 0.00663

