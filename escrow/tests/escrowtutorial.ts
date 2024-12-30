import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrowtutorial } from "../target/types/escrowtutorial";

describe("escrowtutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Escrowtutorial as Program<Escrowtutorial>;

  it("The vault was created!", async () => {
    // Add your test here.
    const tx = await program.methods.make().rpc();
    console.log("Your transaction signature", tx);
  });
  it("The vault was used and closed!", async () => {
    // Add your test here.
    const tx = await program.methods.take().rpc();
    console.log("Your transaction signature", tx);
  });
  it("The deal was refunded!", async () => {
    // Add your test here.
    const tx = await program.methods.refund().rpc();
    console.log("Your transaction signature", tx);
  });
});
