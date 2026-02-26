import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { PerpDex } from "../target/types/perp_dex";
import { PublicKey, Keypair } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { assert } from "chai";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.PerpDex as Program<PerpDex>;

describe("perp-dex", () => {
  let usdcMint: PublicKey;

  before(async () => {
    const payer = (provider.wallet as any).payer as Keypair;

    usdcMint = await createMint(
      provider.connection,
      payer,
      provider.wallet.publicKey,
      null,
      6
    );
  });

  it("initializes state", async () => {
    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("drift_state")],
      program.programId
    );

    const [driftSigner] = PublicKey.findProgramAddressSync(
      [Buffer.from("drift_signer")],
      program.programId
    );

    await program.methods
      .initializeState(new anchor.BN(100))
      .accounts({
        admin: provider.wallet.publicKey,
        quoteAssetMint: usdcMint,
        driftSigner,
      })
      .rpc();

    const state = await program.account.state.fetch(statePda);
    assert.equal(state.noOfMarkets.toNumber(), 0);
    assert.equal(state.perpFee.toNumber(), 100);
    assert.ok(state.signer.equals(driftSigner));
  });
});
