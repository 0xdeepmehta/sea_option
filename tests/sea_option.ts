import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SeaOption } from "../target/types/sea_option";

describe("sea_option", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SeaOption as Program<SeaOption>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
