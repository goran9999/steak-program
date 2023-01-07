import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SteakProgram } from "../target/types/steak_program";

describe("steak-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SteakProgram as Program<SteakProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
