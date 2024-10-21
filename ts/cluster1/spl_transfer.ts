import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("38LBTGkyKdFKsJs9QiNHMDCTuCZUQ8kEsQ2Pug1AiiSt");

// Recipient address
const to = new PublicKey("DASA2yWgfTZWhh66SNcRDqrBFttMQzM6Ty4KcvB7Rm9z");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)
        // Get the token account of the toWallet address, and if it does not exist, create it
        console.log(fromAta)
        const toAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to)
        // Transfer the new token to the "toTokenAccount" we just created
        console.log(toAta)
        const transferm = await transfer(connection, keypair, fromAta.address, toAta.address, keypair, 10_000_000n)
        console.log(transferm)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();