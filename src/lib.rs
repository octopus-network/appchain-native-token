use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    assert_one_yocto, env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue,
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct AppchainNativeToken {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    owner_id: AccountId,
}

#[near_bindgen]
impl AppchainNativeToken {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            owner_id: owner_id.clone().into(),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token
            .internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }

    #[payable]
    pub fn mint(&mut self, account_id: ValidAccountId, amount: U128) {
        self.assert_owner();
        self.storage_deposit(Some(account_id.clone()), None);
        self.token
            .internal_deposit(account_id.as_ref(), amount.into());
    }

    #[payable]
    pub fn burn(&mut self, account_id: ValidAccountId, amount: U128) {
        assert_one_yocto();
        self.assert_owner();
        self.token
            .internal_withdraw(account_id.as_ref(), amount.into());
    }
}

pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.get_owner(),
            "Only owner can call mint"
        );
    }
    fn get_owner(&self) -> AccountId;
    fn set_owner(&mut self, owner: AccountId);
}

#[near_bindgen]
impl Ownable for AppchainNativeToken {
    fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn set_owner(&mut self, owner_id: AccountId) {
        self.assert_owner();
        self.owner_id = owner_id;
    }
}

near_contract_standards::impl_fungible_token_core!(AppchainNativeToken, token);
near_contract_standards::impl_fungible_token_storage!(AppchainNativeToken, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for AppchainNativeToken {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::test_utils::{accounts, VMContextBuilder};
//     use near_sdk::{env, testing_env, MockedBlockchain};
//     const DECIMALS: u8 = 24;
//     const TOTAL_SUPPLY: Balance = 100 * 1_000_000 * (10 as u128).pow(DECIMALS as u32);

//     #[test]
//     fn test_basics() {
//         let mut context = VMContextBuilder::new();
//         testing_env!(context.build());
//         let mut contract = AppchainNativeToken::new(
//             accounts(1).into(),
//             U128::from(0),
//             FungibleTokenMetadata {
//                 spec: FT_METADATA_SPEC.to_string(),
//                 name: "TestToken".to_string(),
//                 symbol: "TEST".to_string(),
//                 icon: None,
//                 reference: None,
//                 reference_hash: None,
//                 decimals: DECIMALS,
//             },
//         );
//         contract.mint(accounts(1), 1_000_000.into());
//         assert_eq!(contract.ft_balance_of(accounts(1)), 1_000_000.into());

//         testing_env!(context
//             .attached_deposit(125 * env::storage_byte_cost())
//             .build());
//         contract.storage_deposit(Some(accounts(1)), None);
//         testing_env!(context
//             .attached_deposit(1)
//             .predecessor_account_id(accounts(0))
//             .build());
//         contract.ft_transfer(accounts(1), 1_000.into(), None);
//         assert_eq!(contract.ft_balance_of(accounts(1)), 1_000.into());

//         contract.burn(accounts(1), 500.into());
//         assert_eq!(contract.ft_balance_of(accounts(1)), 500.into());
//     }
// }
