import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { PerpDex } from "../target/types/perp_dex";

describe("perp-dex", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.perpDex as Program<PerpDex>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
