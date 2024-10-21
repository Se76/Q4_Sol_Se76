import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())


const name = "RugNFT";
const symbol = "RNFT";
const sellerFeeBasisPoints = percentAmount(5,2);
const mint = generateSigner(umi);
const uri = "https://devnet.irys.xyz/De4umsSog6AE2RDc1181wv7iGHdXwssDe4zxAz9ojLaP";
const image = "https://devnet.irys.xyz/4C5XJ6bkSzsBwzqg5mTM4CtGtmTU4Vpdnyrjg6a8PXii";
(async () => {
    let tx = createNft(
        umi,
        {
            mint,
            name,
            symbol,
            uri,
            sellerFeeBasisPoints
        }
    )
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();