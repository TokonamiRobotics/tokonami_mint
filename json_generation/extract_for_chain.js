import fs from "fs";
import hasha from "hasha";

let rawdata = fs.readFileSync('./json/_metadata.json');
let allData = JSON.parse(rawdata);

let counter = 0;
let currentObj;
for (let item of allData) {
    counter += 1;
    
    let fileHash = await hasha.fromFile(`./new_json/${counter}.json`, {algorithm: 'sha256'});
    currentObj = {
        title: item.name,
        description: item.description,
        media: item.image.replace("NewUriToReplace", "QmehZFCwtyubKgPBRpiJ4BHURMkgWFuU2UUg4nw66bqvpb"),
        media_hash: item.dna,
        copies: 1, // number of copies of this set of metadata in existence when token was minted.
        issued_at: null, // When token was issued or minted, Unix epoch in milliseconds
        expires_at: null, // When token expires, Unix epoch in milliseconds
        starts_at: null, // When token starts being valid, Unix epoch in milliseconds
        updated_at: null, // When token was last updated, Unix epoch in milliseconds
        extra: null, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
        reference: `ipfs://QmbLfzD6rix9bQLiSSajhL7JpTSHzc5c3M8wrWBkjqTMBG/${counter}.json`,
        reference_hash: fileHash // B
    };
    console.log(currentObj)
    fs.writeFile(`./chain_json/${counter}.json`, JSON.stringify(currentObj), function (err) {
        if (err) {
            console.log("An error occured while writing JSON Object to File.");
            return console.log(err);
        }
     
        console.log(`./chain_json/${counter}.json saved!`);
    });
}