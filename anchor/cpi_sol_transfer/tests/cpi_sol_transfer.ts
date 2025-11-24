import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiSolTransfer } from "../target/types/cpi_sol_transfer";
import { assert } from "chai";

describe("cpi_sol_transfer", () => {
  // configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.cpiSolTransfer as Program<CpiSolTransfer>;
  const provider = anchor.getProvider();
  const recipient = anchor.web3.Keypair.generate();

  it("transfers sol via cpi", async () => {
    // get initial balances
    const senderInitialBalance = await provider.connection.getBalance(
      provider.publicKey
    );
    const recipientInitialBalance = await provider.connection.getBalance(
      recipient.publicKey
    );

    // transfer 1 sol (1000000000 lamports)
    const transferAmount = new anchor.BN(1_000_000_000);

    const tx = await program.methods
      .solTransfer(transferAmount)
      .accounts({
        sender: provider.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    console.log("transaction signature:", tx);

    // get final balances
    const senderFinalBalance = await provider.connection.getBalance(
      provider.publicKey
    );
    const recipientFinalBalance = await provider.connection.getBalance(
      recipient.publicKey
    );

    // verify recipient received the amount
    assert.equal(
      recipientFinalBalance,
      recipientInitialBalance + transferAmount.toNumber(),
      "recipient should receive transfer amount"
    );

    // verify sender balance decreased (amount + fees)
    assert.isBelow(
      senderFinalBalance,
      senderInitialBalance - transferAmount.toNumber(),
      "sender balance should decrease"
    );
  });

  it("transfers multiple times to same recipient", async () => {
    const recipient2 = anchor.web3.Keypair.generate();
    const transferAmount = new anchor.BN(500_000_000); // 0.5 sol

    // first transfer
    await program.methods
      .solTransfer(transferAmount)
      .accounts({
        sender: provider.publicKey,
        recipient: recipient2.publicKey,
      })
      .rpc();

    const balanceAfterFirst = await provider.connection.getBalance(
      recipient2.publicKey
    );

    // second transfer
    await program.methods
      .solTransfer(transferAmount)
      .accounts({
        sender: provider.publicKey,
        recipient: recipient2.publicKey,
      })
      .rpc();

    const balanceAfterSecond = await provider.connection.getBalance(
      recipient2.publicKey
    );

    // verify both transfers succeeded
    assert.equal(
      balanceAfterFirst,
      transferAmount.toNumber(),
      "first transfer should succeed"
    );
    assert.equal(
      balanceAfterSecond,
      transferAmount.toNumber() * 2,
      "second transfer should succeed"
    );
  });
});
