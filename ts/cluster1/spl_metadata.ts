import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";

// Define our Mint address
const mint = publicKey("38LBTGkyKdFKsJs9QiNHMDCTuCZUQ8kEsQ2Pug1AiiSt")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint,
            mintAuthority: signer,
            updateAuthority: keypair.publicKey,
        }
        console.log(accounts)
        let data: DataV2Args = {
            name: "NickNutTurbin3",
            symbol: "NNT3",
            uri: "",
            sellerFeeBasisPoints: 1,
            creators: null,
            uses: null,
            collection: null
        }
        console.log(data)
        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: false,
            collectionDetails: null,
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
            
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(result);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
