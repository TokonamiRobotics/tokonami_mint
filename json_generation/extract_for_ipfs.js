const fs = require('fs');

let rawdata = fs.readFileSync('./json/_metadata.json');
let allData = JSON.parse(rawdata);

let counter = 0;
let currentObj;
for (let item of allData) {
    counter += 1;
    currentObj = {
        name: item.name,
        description: item.description,
        media: item.image.replace("NewUriToReplace", "QmehZFCwtyubKgPBRpiJ4BHURMkgWFuU2UUg4nw66bqvpb"),
        media_hash: item.dna,
        edition: item.edition,
        date: item.date,
        attributes: item.attributes,
        compiler: item.compiler
    };
    fs.writeFile(`./new_json/${counter}.json`, JSON.stringify(currentObj), function (err) {
        if (err) {
            console.log("An error occured while writing JSON Object to File.");
            return console.log(err);
        }
     
        console.log(`./new_json/${counter}.json saved!`);
    });
}