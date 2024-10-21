"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const wba_wallet_json_1 = __importDefault(require("../wba-wallet.json"));
const umi_bundle_defaults_1 = require("@metaplex-foundation/umi-bundle-defaults");
const umi_1 = require("@metaplex-foundation/umi");
const umi_uploader_irys_1 = require("@metaplex-foundation/umi-uploader-irys");
const promises_1 = require("fs/promises");
// Create a devnet connection
const umi = (0, umi_bundle_defaults_1.createUmi)('https://api.devnet.solana.com');
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wba_wallet_json_1.default));
const signer = (0, umi_1.createSignerFromKeypair)(umi, keypair);
umi.use((0, umi_uploader_irys_1.irysUploader)());
umi.use((0, umi_1.signerIdentity)(signer));
(async () => {
    try {
        // Fetch latest blockhash manually
        const { blockhash } = await umi.rpc.getLatestBlockhash();
        console.log("Latest Blockhash: ", blockhash);
        //1. Load image
        const image = await (0, promises_1.readFile)("/Users/nicknut/Desktop/Turbin3/firstDay/ts/generug.png");
        //2. Convert image to generic file.
        const genericFile = (0, umi_1.createGenericFile)(image, "generug");
        //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);
        console.log("Your image URI: ", myUri);
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
