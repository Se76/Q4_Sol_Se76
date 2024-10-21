"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const wba_wallet_json_1 = __importDefault(require("../wba-wallet.json"));
const spl_token_1 = require("@solana/spl-token");
// We're going to import our keypair from the wallet file
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(wba_wallet_json_1.default));
//Create a Solana devnet connection
const commitment = "confirmed";
const connection = new web3_js_1.Connection("https://api.devnet.solana.com", commitment);
// Mint address
const mint = new web3_js_1.PublicKey("38LBTGkyKdFKsJs9QiNHMDCTuCZUQ8kEsQ2Pug1AiiSt");
// Recipient address
const to = new web3_js_1.PublicKey("DASA2yWgfTZWhh66SNcRDqrBFttMQzM6Ty4KcvB7Rm9z");
(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromAta = await (0, spl_token_1.getOrCreateAssociatedTokenAccount)(connection, keypair, mint, keypair.publicKey);
        // Get the token account of the toWallet address, and if it does not exist, create it
        console.log(fromAta);
        const toAta = await (0, spl_token_1.getOrCreateAssociatedTokenAccount)(connection, keypair, mint, to);
        // Transfer the new token to the "toTokenAccount" we just created
        console.log(toAta);
        const transferm = await (0, spl_token_1.transfer)(connection, keypair, fromAta.address, toAta.address, keypair, 10000000n);
        console.log(transferm);
    }
    catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
