source <(sed -E -n 's/[^#]+/export &/ p' .env)

# not forget to storage deposit to feer contract once (storage.sh)
#near call $FEE storage_deposit '{"account_id": "'$ID'"}' --accountId $ID --amount 0.00663

# pay fee usdc to feer contract
near call $USDC ft_transfer_call '{"receiver_id": "'$FEE'", "amount": "1", "msg": "{\"fee_token_addr\":\"'$USDC'\",\"token_addr\": \"'$USDC'\",\"token_type\":\"FT\",\"transfer_type\":\"Fee\",\"receiver\":\"'$ID'\",\"chain_to\":\"Near\",\"is_wrapped\":false}"}' --accountId $ID --gas 300000000000000 --depositYocto 1

# deposit usdc to feer contract (change amount according to required deposit)
near call $USDC ft_transfer_call '{"receiver_id": "'$FEE'", "amount": "100", "msg": "{\"fee_token_addr\":\"'$USDC'\",\"token_addr\": \"'$USDC'\",\"token_type\":\"FT\",\"transfer_type\":\"Deposit\",\"receiver\":\"'$ID'\",\"chain_to\":\"Near\",\"is_wrapped\":false}"}' --accountId $ID --gas 300000000000000 --depositYocto 1
