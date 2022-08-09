#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod token {
    use ink_storage::{traits::SpreadAllocate, Mapping};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Token {
        /// total number of tokens in circulation
        total_supply: Balance,
        /// owner, amount
        balances: Mapping<AccountId, Balance>,
        /// (owner, spender), amount
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Token {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            ink_lang::utils::initialize_contract(|c| Self::init(c, initial_supply))
        }

        fn init(&mut self, initial_supply: Balance) {
            let caller = Self::env().caller();

            self.total_supply = initial_supply;
            self.balances.insert(&caller, &initial_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
        }

        #[ink(message)]
        pub fn get_total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn get_balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((&owner, &spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, amount)
        }

        pub fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
        ) -> Result<()> {
            let from_bal = self.get_balance_of(from);
            if from_bal < amount {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_bal - amount));

            let to_bal = self.get_balance_of(to);

            self.balances.insert(to, &(to_bal + amount));

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value: amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
        ) -> Result<()> {
            let spender = self.env().caller();
            let allowance = self.get_allowance_of(from, spender);
            if allowance < amount {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(from, to, amount)?;

            self.allowances
                .insert((&from, &spender), &(allowance - amount));

            Ok(())
        }
    }
}
