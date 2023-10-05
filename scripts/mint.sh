source <(sed -E -n 's/[^#]+/export &/ p' .env)

# mint usdc (only owner can do it)
near call $USDC ft_mint '{"receiver_id":"'$ID'","amount":"100000000"}' --accountId $ID --gas 300000000000000
