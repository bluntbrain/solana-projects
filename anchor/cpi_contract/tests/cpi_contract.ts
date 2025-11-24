import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiContract } from "../target/types/cpi_contract";
import { assert } from "chai";

describe("cpi-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const recipient = anchor.web3.Keypair.generate();

  const program = anchor.workspace.cpiContract as Program<CpiContract>;
  console.log("Program", program);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.solTransfer(new anchor.BN(1000000000)).accounts({
      sender: provider.publicKey,
      recipient: recipient.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);
    const account = await provider.connection.getAccountInfo(recipient.publicKey);
    assert.equal(account?.lamports, 1000000000);
  });
});