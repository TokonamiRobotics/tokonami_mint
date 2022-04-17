#! /usr/bin/env node

import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { deployContractAccountExists, deployContractNewAccount } from './../deploy/deploy_contract.js';
import {
    initializeContract,
    mintNfts,
    addToWhiteList,
    addMetadata,
    retrieveFunds,
    unlockSales,
    updateMintingCost
} from "./../deploy/call_contract_function.js"

//import 

yargs(hideBin(process.argv))
    .command(
        'deploy <accountToDeploy> <binaryLocation>', 'deploy contract to the blockchain',
        (yargs) => {
            yargs.positional(
                'accountToDeploy', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account to which contract will be deployed'
                });
            yargs.positional(
                'binaryLocation', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'absolute path to contract binary'
                });
        },
        (argv) => {
            deployContractAccountExists(argv.accountToDeploy, argv.binaryLocation);
        }
    )
    .command(
        'initialize <ownerAccount> <contractAccount> <mintCost>', 'initialize state for deployed contract',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'mintCost', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'fee to be paid to nft holders from every bet base /100000'
                });

        },
        (argv) => {
            initializeContract(argv.ownerAccount, argv.contractAccount, argv.mintCost);
        }
    )
    .command(
        'addToWhiteList <ownerAccount> <contractAccount> <listBeneficiaries> <allowance>', 'Add wallets to whitelist',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'listBeneficiaries', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'allowance', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
        },
        (argv) => {
            addToWhiteList(argv.ownerAccount, argv.contractAccount, argv.listBeneficiaries, argv.allowance);
        }
    )
    .command(
        'addMetadata <ownerAccount> <contractAccount>', 'Add metadata for token minting',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
        },
        (argv) => {
            addMetadata(argv.ownerAccount, argv.contractAccount);
        }
    )
    .command(
        'updateMintingCost <ownerAccount> <contractAccount> <newCost>', 'activate emergency panic mode',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'newCost', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'New cost to mint token (besides storage)'
                });
        },
        (argv) => {
            updateMintingCost(argv.ownerAccount, argv.contractAccount, argv.newCost);
        }
    )
    .command(
        'unlockSales <ownerAccount> <contractAccount> <status>', 'change locked status of minting',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'status', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'new locked status for contract'
                });
        },
        (argv) => {
            unlockSales(argv.ownerAccount, argv.contractAccount, argv.status);
        }
    )
    .command(
        'retrieveFunds <ownerAccount> <contractAccount> <quantity>', 'withdraw balance from contract',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'quantity', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'fee to be paid to nft holders from every bet base /100000'
                });
        },
        (argv) => {
            retrieveFunds(argv.ownerAccount, argv.contractAccount, argv.quantity);
        }
    )
    .command(
        'mintNfts <ownerAccount> <contractAccount> <quantity> <cost>', 'mint nfts in batch',
        (yargs) => {
            yargs.positional(
                'ownerAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that will send the transaction'
                });
            yargs.positional(
                'contractAccount', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'account that hosts the contract'
                });
            yargs.positional(
                'quantity', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'quantity of nfts to mint'
                });
            yargs.positional(
                'cost', {
                    type: 'string',
                    default: 'Cambi',
                    describe: 'price to be paid per mint'
                });
        },
        (argv) => {
            mintNfts(argv.ownerAccount, argv.contractAccount, argv.quantity, argv.cost);
        }
    )
    .demandCommand(1)
    .parse();