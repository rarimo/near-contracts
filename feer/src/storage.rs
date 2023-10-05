use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
use near_sdk::{AccountId, assert_one_yocto, Balance, env, log, near_bindgen, Promise};
use near_sdk::json_types::U128;

use crate::deposit_operation::DepositOperation;

use super::*;

#[near_bindgen]
impl Feer {
    /// Internal method that returns the Account ID and the deposit operation in case the account was
    /// unregistered.
    #[private]
    pub fn internal_storage_unregister(
        &mut self,
        force: Option<bool>,
    ) -> Option<(AccountId, DepositOperation)> {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let force = force.unwrap_or(false);

        if let Some(op) = self.deposit_operations.get_deposit_op(account_id.clone()) {
            if !(op.deposited || op.fee_charged) || force {
                self.deposit_operations.remove_deposit_op(account_id.clone());
                Promise::new(account_id.clone()).transfer(self.storage_balance_bounds().min.0 + 1);
                Some((account_id.clone(), op))
            } else {
                env::panic_str(
                    "Can't unregister the account with the processing state without force",
                )
            }
        } else {
            log!("The account {} is not registered", &account_id.clone());
            None
        }
    }

    #[private]
    pub fn internal_register_account(&mut self, account_id: &AccountId) {
        assert_eq!(self.deposit_operations.is_exists(&account_id.clone()), false, "Deposits: User already exists");

        self.deposit_operations.add_deposit_op(DepositOperation {
            owner: account_id.clone(),
            deposited: false,
            fee_charged: false,
            token_addr: None,
            token_type: None,
            fee_token_addr: None,
            receiver: None,
            msg: None,
            amount: None,
            token_id: None,
        });
    }

    #[private]
    fn internal_storage_balance_of(&self, account_id: &AccountId) -> Option<StorageBalance> {
        if self.deposit_operations.is_exists(account_id) {
            Some(StorageBalance { total: self.storage_balance_bounds().min, available: 0.into() })
        } else {
            None
        }
    }
}

// https://nomicon.io/Standards/StorageManagement
#[near_bindgen]
impl StorageManagement for Feer {
    // `registration_only` doesn't affect the implementation for feer.
    #[allow(unused_variables)]
    #[payable]
    fn storage_deposit(&mut self, account_id: Option<AccountId>, registration_only: Option<bool>) -> StorageBalance {
        let amount: Balance = env::attached_deposit();
        let account_id = account_id.unwrap_or_else(env::predecessor_account_id);

        if self.deposit_operations.is_exists(&account_id) {
            log!("The account is already registered, refunding the deposit");
            if amount > 0 {
                Promise::new(env::predecessor_account_id()).transfer(amount);
            }
        } else {
            let min_balance = self.storage_balance_bounds().min.0;
            if amount < min_balance {
                panic_str(&*format!("The attached deposit is less than the minimum storage balance, minimum is: {}", min_balance));
            }

            self.internal_register_account(&account_id);
            let refund = amount - min_balance;
            if refund > 0 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        }

        self.internal_storage_balance_of(&account_id).unwrap()
    }

    /// While storage_withdraw normally allows the caller to retrieve `available` balance, the basic
    /// Feer implementation sets storage_balance_bounds.min == storage_balance_bounds.max,
    /// which means available balance will always be 0. So this implementation:
    /// * panics if `amount > 0`
    /// * never transfers Ⓝ to caller
    /// * returns a `storage_balance` struct if `amount` is 0
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        assert_one_yocto();
        let predecessor_account_id = env::predecessor_account_id();
        if let Some(storage_balance) = self.internal_storage_balance_of(&predecessor_account_id) {
            match amount {
                Some(amount) if amount.0 > 0 => {
                    panic_str("The amount is greater than the available storage balance");
                }
                _ => storage_balance,
            }
        } else {
            panic_str(
                format!("The account {} is not registered", &predecessor_account_id).as_str(),
            );
        }
    }

    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        self.internal_storage_unregister(force).is_some()
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        let required_storage_balance =
            Balance::from(self.operations_storage_usage) * env::storage_byte_cost();
        StorageBalanceBounds {
            min: required_storage_balance.into(),
            max: Some(required_storage_balance.into()),
        }
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.internal_storage_balance_of(&account_id)
    }
}
