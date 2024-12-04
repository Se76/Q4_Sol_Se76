import { FinalCapstone } from "../target/types/final_capstone";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import BN from "bn.js";
import wallet from "../id.json";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

const amountOfToken = new BN("1000000");
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);
let mint: anchor.web3.PublicKey;
let mint1: anchor.web3.PublicKey;

describe("final_capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.FinalCapstone as Program<FinalCapstone>;

  const wallet_core = anchor.Wallet.local();

  const reciever = anchor.web3.Keypair.generate();
  connection.requestAirdrop(reciever.publicKey, 10000000);

  const asset = Keypair.generate();
  const collection = Keypair.generate();
  console.log("Colection", collection.publicKey.toBase58());

  // First of all we are initializing the GiftConfig account and the program in general

  it("Program and GiftConfig are initialized", async () => {

    const tx = await program.methods.initialize(
      "hello world",
    ).accountsPartial({
      creator: provider.wallet.publicKey,
      reciever: reciever.publicKey,
      asset: asset.publicKey,
    }).rpc();
    console.log("Transaction signature: ", tx);
  });

  // Then we will create two tokens as example and use "addToken" to add them to the gift,
  // but it is possible to make a gift from up to 20 assets in general

  // First token

  it("Token 1 is created and added to the gift", async () => {
    mint = await createMint(connection, keypair, keypair.publicKey, null, 6)
    console.log(`Your mint is: ${mint.toBase58()}`)

    const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)
    console.log(`Your ata is: ${ata.address.toBase58()}`);

    const mintTx = await mintTo(connection, keypair, mint, ata.address, keypair, 100_000_00n)
    console.log(`Your mint txid: ${mintTx}`);

    const tx = await program.methods.addToken(
      amountOfToken,
      6,
    ).accountsPartial({
      creator: provider.wallet.publicKey,
      reciever: reciever.publicKey,
      mint: mint,
      creatorAta: ata.address,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  // Second token

  it("Token 2 is created and added to the gift", async () => {
    mint1 = await createMint(connection, keypair, keypair.publicKey, null, 6)
    console.log(`Your mint is: ${mint1.toBase58()}`)

    const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint1, keypair.publicKey)
    console.log(`Your ata is: ${ata.address.toBase58()}`);

    const mintTx = await mintTo(connection, keypair, mint1, ata.address, keypair, 100_000_00n)
    console.log(`Your mint txid: ${mintTx}`);

    const tx = await program.methods.addToken(
      amountOfToken,
      6,
    ).accountsPartial({
      creator: provider.wallet.publicKey,
      reciever: reciever.publicKey,
      mint: mint1,
      creatorAta: ata.address,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  // creating collection for Gift NFTs, but exactly for this test it is not necessary

  it("Collection created", async () => {
    const tx = await program.methods.createCollection(

    )
      .accountsPartial({
        signer: provider.publicKey,
        payer: provider.publicKey,
        collection: collection.publicKey,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([wallet_core.payer, collection])
      .rpc();
    console.log("Your transaction signature", tx);

  }
  );

  // Creating Metaplex Core NFT with updateAuthority as reciever 

  it("NFT minted and sent to reciever", async () => {
    const tx = await program.methods.mintAndSendNft(

    ).accountsPartial({
      signer: provider.publicKey,
      payer: provider.publicKey,
      asset: asset.publicKey,
      collection: collection.publicKey,
      mplCoreProgram: MPL_CORE_PROGRAM_ID,
      reciever: reciever.publicKey

    }).signers([wallet_core.payer, asset])
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("NFT", asset.publicKey.toBase58());
  })

  // Burning NFT as pre-last step

  it("NFT burnt", async () => {
    const tx = await program.methods.burn(

    ).accountsPartial({
      signer: reciever.publicKey,
      reciever: reciever.publicKey,
      asset: asset.publicKey,
      collection: collection.publicKey,
      mplCoreProgram: MPL_CORE_PROGRAM_ID,
    }).signers([reciever, asset])
      .rpc();
    console.log("Your transaction signature", tx);
  })

  // Unpacking Gift

  // First token

  it("Asset 1 unpacked", async () => {
    const tx = await program.methods.unpack(

    ).accountsPartial({
      reciever: reciever.publicKey,
      creator: provider.publicKey,
      mint: mint,

    }).signers([reciever])
      .rpc();
    console.log("Your transaction signature", tx);
  })

    // First token

  it("Asset 2 unpacked", async () => {
    const tx = await program.methods.unpack(

    ).accountsPartial({
      reciever: reciever.publicKey,
      creator: provider.publicKey,
      mint: mint1,

    }).signers([reciever])
      .rpc();
    console.log("Your transaction signature", tx);
  })

});
