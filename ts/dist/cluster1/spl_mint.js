"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const spl_token_1 = require("@solana/spl-token");
const wba_wallet_json_1 = __importDefault(require("../wba-wallet.json"));
// Import our keypair from the wallet file
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(wba_wallet_json_1.default));
//Create a Solana devnet connection
const commitment = "confirmed";
const connection = new web3_js_1.Connection("https://api.devnet.solana.com", commitment);
const token_decimals = 10000000n;
// Mint address
const mint = new web3_js_1.PublicKey("38LBTGkyKdFKsJs9QiNHMDCTuCZUQ8kEsQ2Pug1AiiSt"); //  base58
(async () => {
    try {
        // Create an ATA
        const ata = await (0, spl_token_1.getOrCreateAssociatedTokenAccount)(connection, keypair, mint, keypair.publicKey);
        console.log(`Your ata is: ${ata.address.toBase58()}`);
        // Mint to ATA
        const mintTx = await (0, spl_token_1.mintTo)(connection, keypair, mint, ata.address, keypair, 10000000n);
        console.log(`Your mint txid: ${mintTx}`);
    }
    catch (error) {
        console.log(`Oops, something went wrong: ${error}`);
    }
})();
