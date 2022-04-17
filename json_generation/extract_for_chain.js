import fs from "fs";
import hasha from "hasha";

let rawdata = fs.readFileSync('./json/_metadata.json');
let allData = JSON.parse(rawdata);

let counter = 0;
let currentObj;
for (let item of allData) {
    counter += 1;

    let fileHash = await hasha.fromFile(`./new_json/${counter}.json`, { algorithm: 'sha256' });
    currentObj = {
        title: item.name,
        description: item.description,
        media: item.image.replace("NewUriToReplace", "QmehZFCwtyubKgPBRpiJ4BHURMkgWFuU2UUg4nw66bqvpb").replace("ipfs://", "https://gateway.pinata.cloud/ipfs/"),
        media_hash: item.dna,
        copies: 1,
        issued_at: null,
        expires_at: null,
        starts_at: null,
        updated_at: null,
        extra: null,
        reference: `ipfs://QmbLfzD6rix9bQLiSSajhL7JpTSHzc5c3M8wrWBkjqTMBG/${counter}.json`,
        reference_hash: fileHash, // B
        nft_type: ((counter % 3) + 1).toString(),
        nft_rarity: "something here"
    };
    console.log(currentObj)
    fs.writeFile(`./chain_json/${counter}.json`, JSON.stringify(currentObj), function(err) {
        if (err) {
            console.log("An error occured while writing JSON Object to File.");
            return console.log(err);
        }

        console.log(`./chain_json/${counter}.json saved!`);
    });
}