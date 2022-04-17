use std::collections::HashMap;
pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U128, U64};
pub use near_sdk::serde_json::{json, value::Value};
pub use near_sdk_sim::{call, view, deploy, init_simulator, to_yocto, UserAccount, 
    ContractAccount, DEFAULT_GAS, ViewResult, ExecutionResult};
pub use near_sdk::AccountId;
pub use near_sdk_sim::transaction::ExecutionStatus;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NFT_BYTES => "./target/wasm32-unknown-unknown/release/nft_contract.wasm"
}

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";



const GAS_ATTACHMENT: u64 = 300_000_000_000_000;
const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

pub fn should_fail(r: ExecutionResult) {
    match r.status() {
        ExecutionStatus::Failure(_) => {}
        _ => panic!("Should fail"),
    }
}

//add tests for unauthorized stuff

#[test]
fn simulate_full_flow() {
    //Test full flow from deploying app
    //Deploy contract
    //owner uploads metadata
    //owner makes whitelist
    //users mint nfts
    //users mint nfts
    //users burn nfts

    let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
    genesis.gas_limit = u64::MAX;
    genesis.gas_price = 0;

    let root = init_simulator(Some(genesis));

    let dev_account = root.create_user("dev_account".to_string(), to_yocto("10000"));
    
    let consumer1 = root.create_user("consumer1".to_string(), to_yocto("10000"));
    let consumer2 = root.create_user("consumer2".to_string(), to_yocto("10000"));
    let consumer3 = root.create_user("consumer3".to_string(), to_yocto("10000"));

    //deploy contract
    let nft_account = root.deploy(
        &NFT_BYTES,
        "nft_contract".to_string(),
        to_yocto("100")
    );

    //initialize contract
    let nft_contract_metadata = NFTContractMetadata {
        spec: NFT_METADATA_SPEC.to_string(),
        name: "Example NEAR non-fungible token".to_string(),
        symbol: "EXAMPLE".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        base_uri: None,
        reference: None,
        reference_hash: None,
    };

    root.call(
        nft_account.account_id(), 
        "new", 
        &json!({
            "owner_id": dev_account.account_id(),
            "metadata": nft_contract_metadata,
            "mint_cost": U128(ONE_NEAR),
            "royalties_account": dev_account.account_id(),
            "royalties_value": U128(500)
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        0
    ).assert_success();
    
    let mut metadata_hashmap = HashMap::<String, TokenMetadata>::new();

    let mut i: u128 = 1;
    while i < 100 {
        metadata_hashmap.insert(i.to_string(), TokenMetadata {
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
            nft_rarity: Some(i.to_string()),
            nft_type: None
        });
        i = i + 1;
    }

    dev_account.call(
        nft_account.account_id(), 
        "add_metadatalookup",
        &json!({
            "metadata_map": metadata_hashmap
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).assert_success();

    let whitelist_hashmap = HashMap::from([(&consumer1.account_id, 2), (&consumer2.account_id, 4)]);

    dev_account.call(
        nft_account.account_id(), 
        "add_to_whitelist",
        &json!({
            "whitelist_map": whitelist_hashmap
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).assert_success();

    dev_account.call(
        nft_account.account_id(), 
        "unlock_sales",
        &json!({
            "sales_lock": false
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).assert_success();

    //mint correct number of nfts with consumer 1
    consumer1.call(
        nft_account.account_id(), 
        "nft_mint",
        &json!({
            "quantity": U128(2)
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        2012280000000000000000000
    ).assert_success();

    let minted: String = consumer1.call(
        nft_account.account_id(), 
        "nft_supply_for_owner",
        &json!({
            "account_id": consumer1.account_id()
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).unwrap_json();

    assert_eq!(minted, "2".to_string());

    should_fail(
        consumer2.call(
            nft_account.account_id(), 
            "nft_mint",
            &json!({
                "quantity": U128(5)
            }).to_string().into_bytes(),
            GAS_ATTACHMENT, 
            201228000000000000000000000
        )
    );
    
    should_fail(
        consumer3.call(
            nft_account.account_id(), 
            "nft_mint",
            &json!({
                "quantity": U128(1)
            }).to_string().into_bytes(),
            GAS_ATTACHMENT, 
            20122800000000000000000000
        )
    );
    
    //test burn feature
    consumer1.call(
        nft_account.account_id(), 
        "nft_burn",
        &json!({
            "sender_id": consumer1.account_id(),
            "token_id": "1"
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).assert_success();

    let minted_1: String = consumer1.call(
        nft_account.account_id(), 
        "nft_supply_for_owner",
        &json!({
            "account_id": consumer1.account_id()
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).unwrap_json();

    let minted_system: String = consumer1.call(
        nft_account.account_id(), 
        "nft_supply_for_owner",
        &json!({
            "account_id": "system"
        }).to_string().into_bytes(),
        GAS_ATTACHMENT, 
        1
    ).unwrap_json();

    assert_eq!(minted_1, "1".to_string());
    assert_eq!(minted_system, "1".to_string());

}
