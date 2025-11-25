import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingContract } from "../target/types/staking_contract";
import { assert } from "chai";

describe("staking_contract", () => {
  // configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stakingContract as Program<StakingContract>;
  const provider = anchor.getProvider();

  // derive pda address
  const [pdaAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("client1")],
    program.programId
  );

  it("creates pda account", async () => {
    const tx = await program.methods
      .createPdaAccount()
      .accounts({
        payer: provider.publicKey,
      })
      .rpc();

    console.log("create pda account transaction:", tx);

    // fetch and verify the account
    const account = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(account.owner.toBase58(), provider.publicKey.toBase58());
    assert.equal(account.stakedAmount.toNumber(), 0);
    assert.equal(account.totalPoints.toNumber(), 0);
  });

  it("stakes sol", async () => {
    const stakeAmount = new anchor.BN(1_000_000_000); // 1 sol

    const pdaBalanceBefore = await provider.connection.getBalance(pdaAccount);

    const tx = await program.methods
      .stake(stakeAmount)
      .accounts({
        user: provider.publicKey,
      })
      .rpc();

    console.log("stake transaction:", tx);

    // verify staked amount
    const account = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(account.stakedAmount.toString(), stakeAmount.toString());

    // verify pda balance increased
    const pdaBalanceAfter = await provider.connection.getBalance(pdaAccount);
    assert.equal(
      pdaBalanceAfter - pdaBalanceBefore,
      stakeAmount.toNumber(),
      "pda balance should increase by stake amount"
    );
  });

  it("stakes more sol", async () => {
    const stakeAmount = new anchor.BN(500_000_000); // 0.5 sol

    const accountBefore = await program.account.stakeAccount.fetch(pdaAccount);
    const previousStake = accountBefore.stakedAmount;

    const tx = await program.methods
      .stake(stakeAmount)
      .accounts({
        user: provider.publicKey,
      })
      .rpc();

    console.log("second stake transaction:", tx);

    // verify total staked amount
    const accountAfter = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(
      accountAfter.stakedAmount.toString(),
      previousStake.add(stakeAmount).toString(),
      "staked amount should be cumulative"
    );
  });

  it("gets points (read-only)", async () => {
    const tx = await program.methods
      .getPoints()
      .accounts({
        user: provider.publicKey,
      })
      .rpc();

    console.log("get points transaction:", tx);
  });

  it("unstakes partial amount", async () => {
    const unstakeAmount = new anchor.BN(250_000_000); // 0.25 sol

    const accountBefore = await program.account.stakeAccount.fetch(pdaAccount);
    const previousStake = accountBefore.stakedAmount;
    const userBalanceBefore = await provider.connection.getBalance(
      provider.publicKey
    );

    const tx = await program.methods
      .unstake(unstakeAmount)
      .accounts({
        user: provider.publicKey,
      })
      .rpc();

    console.log("unstake transaction:", tx);

    // verify staked amount decreased
    const accountAfter = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(
      accountAfter.stakedAmount.toString(),
      previousStake.sub(unstakeAmount).toString(),
      "staked amount should decrease"
    );

    // verify user balance increased (accounting for tx fees)
    const userBalanceAfter = await provider.connection.getBalance(
      provider.publicKey
    );
    assert.isAbove(
      userBalanceAfter,
      userBalanceBefore,
      "user balance should increase after unstake"
    );
  });

  it("claims points", async () => {
    const accountBefore = await program.account.stakeAccount.fetch(pdaAccount);
    const pointsBefore = accountBefore.totalPoints;

    const tx = await program.methods
      .claimPoints()
      .accounts({
        user: provider.publicKey,
      })
      .rpc();

    console.log("claim points transaction:", tx);

    // verify points were reset after claiming
    const accountAfter = await program.account.stakeAccount.fetch(pdaAccount);
    assert.equal(
      accountAfter.totalPoints.toNumber(),
      0,
      "points should be reset after claiming"
    );

    console.log(
      `claimed ${pointsBefore.toNumber() / 1_000_000} points`
    );
  });

  it("fails to unstake more than staked", async () => {
    const account = await program.account.stakeAccount.fetch(pdaAccount);
    const tooMuch = account.stakedAmount.add(new anchor.BN(1_000_000_000));

    try {
      await program.methods
        .unstake(tooMuch)
        .accounts({
          user: provider.publicKey,
          })
        .rpc();
      assert.fail("should have thrown error");
    } catch (error) {
      assert.include(
        error.toString(),
        "InsufficientStake",
        "should fail with insufficient stake error"
      );
    }
  });

  it("fails to stake zero amount", async () => {
    try {
      await program.methods
        .stake(new anchor.BN(0))
        .accounts({
          user: provider.publicKey,
          })
        .rpc();
      assert.fail("should have thrown error");
    } catch (error) {
      assert.include(
        error.toString(),
        "InvalidAmount",
        "should fail with invalid amount error"
      );
    }
  });
});
