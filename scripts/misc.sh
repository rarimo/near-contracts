source <(sed -E -n 's/[^#]+/export &/ p' .env)

# get balance of usdc
near view $USDC ft_balance_of '{"account_id": "'$ID'"}'
