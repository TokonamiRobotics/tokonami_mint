import nearAPI from "near-api-js";
import loginNear from "./_login.js";
import fs from "fs";

import buildContractObject from "./_contract_object.js";
import { BN } from "bn.js";

async function initializeContract(ownerAccount, contractAccount, mint_cost, royalties_account, royalties_value, urlMedia, urlReference) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    let namedArgs = {
        owner_id: ownerAccount,
        metadata: {
            spec: "nft-1.0.0",
            name: "Tokonami",
            symbol: "TOKO",
            icon: "https://gateway.ipfs.io/ipfs/QmVtLZHbRthmXz5D2pVACVxeGWxj6EGeD7FvRrn2Mnsqk8",
            base_uri: null,
            reference: null,
            reference_hash: null,
        },
        mint_cost: nearAPI.utils.format.parseNearAmount(mint_cost),
        royalties_account: royalties_account,
        royalties_value: royalties_value,
        url_media_base: urlMedia,
        url_reference_base: urlReference

    };

    const result = await contract.new(
        namedArgs,
        "300000000000000"
    );

    console.log(result);
    return result;
}

//batch mint nfts
async function mintNfts(ownerAccount, contractAccount, token_quantity, mint_cost) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    const mintCostPlusStorage = (parseInt(mint_cost) * 1.1).toString();
    const token_price = nearAPI.utils.format.parseNearAmount(mintCostPlusStorage);
    let bnTokenPrice = new BN(token_price);
    let bnTokenQuantity = new BN(token_quantity);
    let totalTokenPrice = bnTokenPrice.mul(bnTokenQuantity);

    const result = await contract.nft_mint({
            quantity: token_quantity.toString()
        },
        "300000000000000",
        totalTokenPrice.toString(10)
    );

    // const result = await contract.mint({
    //         token_id: "1",
    //         token_owner_id: "toko_buyer1.testnet",
    //         token_metadata: {
    //             title: 'Tokonami #1',
    //             description: 'Tokonami to the Moon',
    //             media: 'ipfs://QmehZFCwtyubKgPBRpiJ4BHURMkgWFuU2UUg4nw66bqvpb/1.png',
    //             media_hash: '1d913f2ee339a761a17ca95d6aa957a19861197a',
    //             copies: 1,
    //             issued_at: null,
    //             expires_at: null,
    //             starts_at: null,
    //             updated_at: null,
    //             extra: null,
    //             reference: 'ipfs://QmbLfzD6rix9bQLiSSajhL7JpTSHzc5c3M8wrWBkjqTMBG/1.json',
    //             reference_hash: 'd7f0e56ab1a117937d39fad3f72b99cc831c73f819685ee0a8c6d2e4628fdfcf',
    //             nft_type: '2',
    //             nft_rarity: 'something here'
    //         }
    //     },
    //     "300000000000000",
    // );

    console.log(result);
    return result;
}

//get_contract_state
async function addToWhiteList(ownerAccount, contractAccount, listBeneficiaries, allowance) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    let whiteListMap = {};
    for (let item of JSON.parse(listBeneficiaries)) {
        console.log(item);
        whiteListMap[item] = parseInt(allowance)
    }


    const result = await contract.add_to_whitelist({
            whitelist_map: whiteListMap
        },
        "300000000000000",
        "1"
    );

    console.log(result);
    return result;
}

//add metadata to contract
async function addMetadata(ownerAccount, contractAccount) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    let result = false;
    while (!result) {
        console.log(result);
        result = await contract.initilize_random_generator({},
            "300000000000000",
            "0"
        )
    }

    console.log(result);
}

//update_contract
async function retrieveFunds(ownerAccount, contractAccount, quantity) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    const result = await contract.retrieve_funds({
            quantity: nearAPI.utils.format.parseNearAmount(quantity.toString())
        },
        "300000000000000",
        "1"
    );

    console.log(result);
    return result;
}

//lock/unlock minting
async function unlockSales(ownerAccount, contractAccount, status) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    const result = await contract.unlock_sales({
            sales_lock: (status === "true")
        },
        "300000000000000",
        "1"
    );

    console.log(result);
    return result;
}

//lock/unlock whitelist
async function unlockWhitelist(ownerAccount, contractAccount, status) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    const result = await contract.unlock_whitelist({
            whitelist_lock: (status === "true")
        },
        "300000000000000",
        "1"
    );

    console.log(result);
    return result;
}

//change minting cost
async function updateMintingCost(ownerAccount, contractAccount, newCost) {
    const contract = await buildContractObject(ownerAccount, contractAccount);

    const result = await contract.change_mint_cost({
            mint_cost: nearAPI.utils.format.parseNearAmount(newCost.toString())
        },
        "300000000000000",
        "1"
    );

    console.log(result);
    return result;
}

export {
    initializeContract,
    mintNfts,
    addToWhiteList,
    addMetadata,
    retrieveFunds,
    unlockSales,
    updateMintingCost,
    unlockWhitelist
};