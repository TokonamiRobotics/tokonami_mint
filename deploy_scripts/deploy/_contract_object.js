import nearAPI from "near-api-js";

import loginNear from "./_login.js";

async function buildContractObject(ownerAccount, contractAccount) {
    const { near, account } = await loginNear(ownerAccount);

    const contract = new nearAPI.Contract(
        account, // the account object that is connecting
        contractAccount, {
            viewMethods: [],
            changeMethods: [
                "new", "nft_mint", "nft_burn", "mint",
                "add_to_whitelist", "initilize_random_generator", "retrieve_funds",
                "unlock_sales", "unlock_whitelist", "change_mint_cost"
            ],
            sender: account, // account object to initialize and sign transactions.
        }
    );
    return contract
}

export default buildContractObject;