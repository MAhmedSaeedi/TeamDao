import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Teamdao } from "../target/types/teamdao";

describe("teamdao", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Teamdao as Program<Teamdao>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
