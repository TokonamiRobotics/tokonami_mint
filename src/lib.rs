/*!
Non-Fungible Token implementation with JSON serialization.
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
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::{ValidAccountId};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

use std::convert::TryInto;

near_sdk::setup_alloc!();

pub fn assert_one_or_more_yocto() {
    assert!(env::attached_deposit() >= 1, "Requires attached deposit of 1 yoctoNEAR or more")
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub tokens: NonFungibleToken,
    pub metadata: LazyOption<NFTContractMetadata>,

    pub whitelist: LookupMap<AccountId, u128>,
    pub mint_cost: u128,
    pub current_id: u128,
    pub id_metadata_lookup: LookupMap<u128, TokenMetadata>
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Whitelist,
    MetadataLookup
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
            U128(10)
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata, mint_cost: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            whitelist: LookupMap::new(StorageKey::Whitelist),
            mint_cost: mint_cost.0,
            current_id: 1,
            id_metadata_lookup: LookupMap::new(StorageKey::MetadataLookup)
        }
    }

    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.

    //minter must be whitelisted possibility to mint multiple nfts in batch
    #[payable]
    pub fn nft_mint(
        &mut self,
        quantity: U128
    ) -> Vec<Token> {
        let account_id: AccountId = env::predecessor_account_id();
        let allowance: u128 = self.whitelist.get(&account_id).unwrap_or(0);

        assert!(&allowance >= &quantity.0, "Whitelist error: this account has no allowance for minitng this amount of NFTs");

        let mut return_vector = Vec::new();

        let mut i: u128 = 0;
        while i < quantity.0 {
            return_vector.push( 
                self.tokens.internal_mint( 
                    self.current_id.to_string(), 
                    account_id.clone().try_into().expect("a"), 
                    Some(self.id_metadata_lookup.get(&self.current_id).expect("b")),
                    self.mint_cost
                )
            );
            self.current_id = self.current_id + 1;
            i = i + 1;
        }
        self.whitelist.insert(&account_id, &(allowance - quantity.0));
        return_vector
    }

    //burn token
    #[payable]
    pub fn nft_burn(
        &mut self,
        sender_id: &AccountId,
        token_id: &TokenId,
    ) -> bool {
        self.tokens.internal_transfer(
            sender_id,
            &"system".to_string(),
            token_id,
            None,
            None
        );
        true
    }

    //add people to whitelist
    #[payable]
    pub fn add_to_whitelist(
        &mut self,
        whitelist_map: HashMap<AccountId, u128>
    ) -> bool {
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
        assert_one_or_more_yocto();
        for key in whitelist_map.keys() {
            self.whitelist.insert(key, whitelist_map.get(key).unwrap());
        }
        true
    }

    //add metadatalookup
    #[payable]
    pub fn add_metadatalookup(
        &mut self,
        metadata_map: HashMap<u128, TokenMetadata>
    ) -> bool {
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
        assert_one_or_more_yocto();
        for key in metadata_map.keys() {
            self.id_metadata_lookup.insert(key, metadata_map.get(key).unwrap());
        }
        true
    }

}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    use super::*;

    const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
    const MINT_STORAGE_COST: u128 = 5890000000000000000000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            nft_rarity: None,
            nft_type: None
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ONE_NEAR + MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "1".to_string();
        contract.add_to_whitelist(HashMap::from([(accounts(0).to_string(), 2)]));
        contract.add_metadatalookup(HashMap::from([(1, sample_token_metadata())]));
        let tokena = contract.nft_mint(U128(1));
        let token = tokena.get(0).unwrap();
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0).to_string());
        assert_eq!(token.metadata.clone().unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.clone().unwrap(), HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ONE_NEAR + MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "1".to_string();
        contract.add_to_whitelist(HashMap::from([(accounts(0).to_string(), 2)]));
        contract.add_metadatalookup(HashMap::from([(1, sample_token_metadata())]));
        contract.nft_mint(U128(1));

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1).to_string());
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    #[test]
    fn test_approve() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ONE_NEAR + MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "1".to_string();
        contract.add_to_whitelist(HashMap::from([(accounts(0).to_string(), 2)]));
        contract.add_metadatalookup(HashMap::from([(1, sample_token_metadata())]));
        contract.nft_mint(U128(1));

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }

    #[test]
    fn test_revoke() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ONE_NEAR + MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "1".to_string();
        contract.add_to_whitelist(HashMap::from([(accounts(0).to_string(), 2)]));
        contract.add_metadatalookup(HashMap::from([(1, sample_token_metadata())]));
        contract.nft_mint(U128(1));

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke(token_id.clone(), accounts(1));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
    }

    #[test]
    fn test_revoke_all() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(ONE_NEAR + MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "1".to_string();
        contract.add_to_whitelist(HashMap::from([(accounts(0).to_string(), 2)]));
        contract.add_metadatalookup(HashMap::from([(1, sample_token_metadata())]));
        contract.nft_mint(U128(1));

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke_all(token_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }
}
