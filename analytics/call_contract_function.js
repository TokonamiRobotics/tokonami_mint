// demonstrates how to query the state without setting
// up an account. (View methods only)
import fs from "fs";
import { providers } from "near-api-js";
//network config (replace testnet with mainnet or betanet)
const provider = new providers.JsonRpcProvider("https://rpc.mainnet.near.org");

async function getNFTCount() {
    const rawResult = await provider.query({
        request_type: "call_function",
        account_id: "tokodao.near",
        method_name: "nft_total_supply",
        args_base64: "",
        finality: "optimistic",
    });

    // format result
    const res = JSON.parse(Buffer.from(rawResult.result).toString());
    return res
}

async function getNFTData(id) {
    const rawResult = await provider.query({
        request_type: "call_function",
        account_id: "tokodao.near",
        method_name: "nft_token",
        args_base64: Buffer.from(JSON.stringify({ "token_id": id })).toString('base64'),
        finality: "optimistic",
    });

    // format result
    const res = JSON.parse(Buffer.from(rawResult.result).toString());
    return res
}

let total_nft_count = await getNFTCount();
total_nft_count = 2500;

let counter = 0;
let total_pulled = 0;
let token;
let tokens_array = [];
while (total_pulled < total_nft_count) {
    token = await getNFTData(counter.toString());
    counter += 1
    if (token == null) {
        continue;
    } else {
        total_pulled += 1;
        tokens_array.push(token);
    }

}
fs.writeFile(`all.json`, JSON.stringify(tokens_array), function(err) {
    if (err) {
        console.log("An error occured while writing JSON Object to File.");
        return console.log(err);
    }

    console.log(`all.json saved!`);
});