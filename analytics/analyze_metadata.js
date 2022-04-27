import fs from "fs";

let rawdata = fs.readFileSync('./all.json');
let rawdataMeta = fs.readFileSync('./_metadata.json');
let allData = JSON.parse(rawdata);
let allMetaData = JSON.parse(rawdataMeta);

let dataObject = {
    "1": 0,
    "2": 0,
    "3": 0
};

let tokenIds = [];

for (let token of allData) {
    if (token.owner_id === "system") continue;
    tokenIds.push(token.token_id)
}

let onlyMintedList = []

for (let token of allMetaData) {
    if (tokenIds.indexOf(token.name.split("#")[1]) == -1) continue;
    onlyMintedList.push(token);
}

fs.writeFile(`onlyMintedList.json`, JSON.stringify(onlyMintedList), function(err) {
    if (err) {
        console.log("An error occured while writing JSON Object to File.");
        return console.log(err);
    }

    console.log(`onlyMintedList.json saved!`);
});

let objectTypes = {
    Background: {},
    Wing: {},
    Rightarm: {},
    Chest: {},
    Head: {},
    Helmet: {},
    Visor: {},
    Medal: {},
    Leftarm: {},
    Weapon: {}
};


for (let token of onlyMintedList) {
    for (let attr of token.attributes) {
        if (objectTypes[attr.trait_type][attr.value] == undefined) {
            objectTypes[attr.trait_type][attr.value] = 1;
        } else {
            objectTypes[attr.trait_type][attr.value] += 1;
        }
    }
}

console.log(objectTypes);

//get percentage
let total_count = onlyMintedList.length;

for (let type of Object.keys(objectTypes)) {
    for (let subtype of Object.keys(objectTypes[type])) {
        objectTypes[type][subtype] = (objectTypes[type][subtype] / total_count) * 100
    }
}

console.log(objectTypes);