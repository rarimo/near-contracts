/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
 */
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::fungible_token::metadata::{
    FT_METADATA_SPEC, FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_sdk::{AccountId, Balance, env, log, near_bindgen, PanicOnDefault, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    owner_id: AccountId,
}

const DEFAULT_TOKEN_LOGO: &str = "data:image/svg+xml;charset=UTF-8,%3Csvg version='1.0' xmlns='http://www.w3.org/2000/svg' width='512.000000pt' height='512.000000pt' viewBox='0 0 512.000000 512.000000' preserveAspectRatio='xMidYMid meet'%3E%3Cg transform='translate(0.000000,512.000000) scale(0.100000,-0.100000)'%0Afill='%23000000' stroke='none'%3E%3Cpath d='M2567 2912 c-22 -24 -21 -55 1 -75 37 -33 92 -11 92 37 0 48 -62 73%0A-93 38z'/%3E%3Cpath d='M1068 2904 c-5 -4 -8 -162 -8 -351 l0 -343 45 0 45 0 0 135 0 136 86%0A-3 87 -3 71 -132 71 -133 48 0 c26 0 47 2 47 5 0 3 -34 67 -75 142 l-74 137%0A27 17 c71 43 109 125 97 213 -10 80 -41 123 -115 162 -28 15 -64 19 -190 22%0A-85 2 -158 0 -162 -4z m317 -88 c42 -18 65 -61 65 -123 0 -104 -44 -133 -197%0A-133 l-103 0 0 135 0 135 100 0 c64 0 113 -5 135 -14z'/%3E%3Cpath d='M1776 2726 c-39 -15 -90 -57 -107 -89 -8 -15 -4 -20 25 -32 34 -14%0A35 -14 68 19 80 80 218 49 218 -50 0 -18 -4 -36 -10 -39 -5 -3 -54 -13 -107%0A-21 -170 -25 -234 -78 -218 -179 12 -84 79 -135 175 -135 62 0 110 19 138 55%0Al22 28 0 -37 c0 -35 1 -36 41 -36 l40 0 -3 214 c-3 200 -4 217 -24 243 -12 15%0A-36 37 -53 47 -43 25 -153 31 -205 12z m187 -388 c-48 -82 -194 -92 -231 -16%0A-16 34 -6 75 23 94 13 9 65 22 117 30 51 9 95 18 98 21 3 2 6 -18 8 -47 2 -36%0A-2 -61 -15 -82z'/%3E%3Cpath d='M2354 2721 c-18 -11 -39 -30 -48 -42 -16 -23 -16 -23 -16 14 0 37 0%0A37 -40 37 l-40 0 0 -260 0 -260 40 0 40 0 0 180 c0 200 6 227 60 255 16 8 49%0A15 75 15 l45 0 0 40 0 40 -42 0 c-25 0 -57 -8 -74 -19z'/%3E%3Cpath d='M2944 2721 c-17 -10 -40 -31 -52 -46 l-22 -28 0 42 0 41 -40 0 -40 0%0A0 -260 0 -260 39 0 39 0 4 183 c3 206 11 232 74 262 45 21 92 14 125 -21 23%0A-25 24 -30 27 -225 l3 -199 40 0 39 0 0 196 0 196 29 28 c36 37 72 46 122 31%0A63 -19 69 -40 69 -261 l0 -190 40 0 40 0 0 203 c0 191 -1 205 -22 247 -51 98%0A-196 110 -270 22 l-25 -30 -23 30 c-42 56 -136 75 -196 39z'/%3E%3Cpath d='M3755 2725 c-54 -19 -118 -91 -136 -152 -17 -57 -17 -152 -1 -209 15%0A-53 76 -122 126 -145 98 -45 227 -9 285 78 73 110 63 282 -22 372 -63 66 -161%0A88 -252 56z m183 -97 c90 -88 74 -284 -28 -340 -41 -23 -114 -19 -154 7 -59%0A39 -88 175 -57 266 13 39 52 82 91 98 44 19 110 4 148 -31z'/%3E%3Cpath d='M2570 2470 l0 -260 40 0 40 0 0 260 0 260 -40 0 -40 0 0 -260z'/%3E%3C/g%3E%3C/svg%3E%0A";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Example NEAR fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DEFAULT_TOKEN_LOGO.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        mut metadata: FungibleTokenMetadata,
    ) -> Self {
        metadata.assert_valid();

        if metadata.icon.is_none() {
            metadata.icon = Some(String::from(DEFAULT_TOKEN_LOGO));
        }

        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            owner_id: owner_id.clone(),
        };

        // Need to register system account to make possible to burn tokens
        this.token.internal_register_account(&AccountId::new_unchecked("system".to_string()));

        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
            .emit();
        this
    }

    #[payable]
    pub fn ft_mint(&mut self, receiver_id: AccountId, amount: U128) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized: Only owner can mint");

        let balance =  self.token.accounts.get(&receiver_id);

        if balance.is_none() {
            self.token.internal_register_account(&receiver_id);
        }

        self.token.internal_deposit(&receiver_id, amount.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &receiver_id,
            amount: &amount,
            memo: None,
        }
            .emit();
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::{Balance, testing_env};
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::{accounts, VMContextBuilder};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
