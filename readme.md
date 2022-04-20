# Generic NFT coin-flip contract for NEAR protocol

## Contract  
  
Contract lives in the "nft_contract" folder. compile it using the instructions for rust near sdk available in: https://github.com/near/near-sdk-rs

## CLI usage  
For the convenience of the non technical user, a command line interface (CLI) has been built to perform the main administrative tasks in running your coin-flip app.  

### Install the CLI  
To install the CLI you must have node.js installed in your system and you must have added it to the PATH environment variable. You can check that by typing "node" in your command prompt, if the command is not recognized, you have to add it to PATH.  

Installation: https://nodejs.org/en/download/  
  
Add to PATH: https://www.tutorialspoint.com/nodejs/nodejs_environment_setup.htm  
  
After that you can install the command line interface by opening the "deploy_scripts" folder in your command line and typing:  
```
npm install -g .
```   

  
### Wallet setup  
Before starting the deployment you must have 2 addresses for use:  
1. Your owner address  
2. A contract address  

The owner address will receive the fees from the contract and have access to administrative functions such as defining whitelist, locking sales and changing minitng cost.  
  
Both contracts should have a NEAR balance for payment of gas fees and staking of storage.  
  
Log in with both accounts using the NEAR CLI.

### Deploy the contract  
To deploy the contract, you'll run the following script in your command line
```
nft-tokonami deploy <accountToDeploy> <binaryLocation>
```  
Substitute accountToDeploy for the contract address you created  
Substitute binaryLocation for the absolute path to the contract's compiled binaries  

### Initialize the contract  
Before the contract can be used, you'll need to setup its configuration:
```
nft-tokonami initialize <ownerAccount> <contractAccount> <mintCost> <royaltiesAccount> <royaltiesValue> <urlMedia> <urlReference>
```
ownerAccount is the owner account you created to manage the game  
contractAccount is the account to which the contract was deployed  
mintCost is the cost in NEAR to mint a NFT (excluding storage deposit)
royaltiesAccount is the account that will receive royalties from the NFT sales
royaltiesValues is the amount of royalties to be received in each transaction (integer divided by 10000)  
urlMedia refers to the path to an ipfs gatway pointing to the CID of the folder containing the images for the collection, v.g. https://gateway.pinata.cloud/ipfs/QmehZFCwtyubKgPBRpiJ4BHURMkgWFuU2UUg4nw66bqvpb
urlReference refers to the path to an ipfs gatway pointing to the CID of the folder containing the json files for the collection, v.g. https://gateway.pinata.cloud/ipfs/QmehZFCwtyubKgPBRpiJ4BHURMkgWFuU2UUg4nw66bqvpb
  
### add to whitelist
To add wallets to the buying whitelist:
```
nft-tokonami addToWhiteList <ownerAccount> <contractAccount> <listBeneficiaries> <allowance>
```
ownerAccount is the owner account you created to manage the game  
contractAccount is the account to which the contract was deployed  
listBeneficiaries is a list in the format '["acount1.near", "account2.near"]' with account you want to include
allowance is the amount of NFTs that the beneficiaries will be able to mint
  
### add metadata to chain
To add the metadata for minting:
```
nft-tokonami addMetadata <ownerAccount> <contractAccount>
```
ownerAccount is the owner account you created to manage the game  
contractAccount is the account to which the contract was deployed 
  
### change the minting cost 
To change the minting cost, use this call, use this call:
```
nft-tokonami updateMintingCost <ownerAccount> <contractAccount> <newCost>
```
ownerAccount is the owner account you created to manage the game  
contractAccount is the account to which the contract was deployed 
newCost is the new cost in NEAR that you want to set for the mint of a NFT
  
### lock/unlock minting
When the contract is initialize, the minting of NFTs is locked, to unlock it or relock it, run this call:
```
nft-tokonami unlockSales <ownerAccount> <contractAccount> <status>
```
ownerAccount is the owner account you created to manage the game  
contractAccount is the account to which the contract was deployed  
status use false to unlock and true to lock

### lock/unlock whitelist
When the contract is initialize, the minting of NFTs is only allowed for whitelisted wallets, to unlock it or relock it, run this call:
```
nft-tokonami unlockWhitelist <ownerAccount> <contractAccount> <status>
```
ownerAccount is the owner account you created to manage the game  
contractAccount is the account to which the contract was deployed  
status use false to unlock and true to lock
  
### retrieve the funds
To retrieve the minting funds to the owner of the project account, use this call:
```
nft-tokonami retrieveFunds <ownerAccount> <contractAccount> <quantity>
```
ownerAccount is the owner account you created to manage the game    
contractAccount is the account to which the contract was deployed    
quantity is the amount of NEAR that you wish to send from the contract account to the owner account  

### mint NFTs using CLI
To mint an NFT via CLI, use this call:
```
nft-tokonami mintNfts <ownerAccount> <contractAccount> <quantity> <cost>
```
ownerAccount is the owner account you created to manage the game    
contractAccount is the account to which the contract was deployed    
quantity is the amount of NFTs you want to mint
cost is the current cost to mint a NFT 
  
### mint NFTs without using this CLI
For special whitelisted wallets, such as advisors or investors, that will have to mint a lot of NFTs, it's recommended to use the near cli solution with the following command:
**User needs to first login with his/her wallet in near cli
```
near call <contractAccount> nft_mint '{"quantity": "<quantityToMint>"}' --account-id <userAccount> --deposit <mintCostPlusStorage>
```
contractAccount is the account that hosts the NFT contract
quantityToMint is the quantity of NFTs that they want to mint
userAccount is the wallet that wants to do the minting
mintCostPlusStorage is the amount of near to send to the contract, must be cost of minting plus enough to cover storage (attach 1 mora NEAR, any surplus will be transfered back to you after execution)

**Because of blockchain limitations, it's advisable to only send transactions in smaller batches, such as 30 per transaction