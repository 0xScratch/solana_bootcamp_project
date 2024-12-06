import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaBootcampProject } from "../target/types/solana_bootcamp_project";

describe("solana_bootcamp_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaBootcampProject as Program<SolanaBootcampProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
