// - initialize_program()
// - initialize_user()
// - initialize_rewards()
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenHouseRedone } from "../target/types/open_house_redone";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

describe("open_house_redone_initialization", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

  let rewardsMint: PublicKey;
  let programStatePda: PublicKey;
  let rewardsTreasuryPda: PublicKey;

  before(async () => {
    // Find PDA addresses we'll need
    [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId
    );

    [rewardsTreasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("rewards_treasury")],
      program.programId
    );
  });

  it("Initializes program state lol", async () => {
    // Generate a keypair for the mint - BUT DON'T CREATE IT
    // Anchor will create it for us because of #[account(init)]
    // Generate keypair for the mint
    // Generate keypair for the mint
    const rewardsMintKeypair = Keypair.generate();

    // Find PDAs
    const [programStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("program_state")],
      program.programId
    );

    const [rewardsTreasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("rewards_treasury")],
      program.programId
    );

    console.log("Starting account creation...");
    console.log("Program ID:", program.programId.toBase58());
    console.log("Authority:", provider.wallet.publicKey.toBase58());
    console.log("Program State PDA:", programStatePda.toBase58());
    console.log("Rewards Treasury PDA:", rewardsTreasuryPda.toBase58());
    console.log("Rewards Mint:", rewardsMintKeypair.publicKey.toBase58());

    try {
      const tx = await program.methods
        .initializeProgram()
        .accounts({
          authority: provider.wallet.publicKey,
          programState: programStatePda,
          rewardsMint: rewardsMintKeypair.publicKey,
          rewardsTreasury: rewardsTreasuryPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([rewardsMintKeypair])
        .rpc({ commitment: "confirmed" });

      console.log("Transaction signature:", tx);

      const programState = await program.account.programState.fetch(
        programStatePda
      );
      console.log("Program state:", {
        authority: programState.authority.toBase58(),
        rewardsMint: programState.rewardsMint.toBase58(),
        rewardsTreasury: programState.rewardsTreasury.toBase58(),
        totalUsers: programState.totalUsers.toString(),
        bump: programState.bump,
      });
    } catch (error) {
      console.error("Detailed error:", error);
      if (error.logs) {
        console.error("Program logs:", error.logs);
      }
      throw error;
    }
  });

  it("Initializes program state", async () => {
    // Create a new mint for rewards

    rewardsMint = await createMint(
      provider.connection,
      await (provider.wallet as any).payer,
      provider.wallet.publicKey,
      provider.wallet.publicKey,
      6 // decimals
    );

    // Initialize program state
    await program.methods
      .initializeProgram()
      .accounts({
        authority: provider.wallet.publicKey,
        programState: programStatePda,
        rewardsMint: rewardsMint,
        rewardsTreasury: rewardsTreasuryPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    // // Verify program state
    // const programState = await program.account.programState.fetch(
    //   programStatePda
    // );
    // assert.ok(programState.authority.equals(provider.wallet.publicKey));
    // assert.ok(programState.rewardsMint.equals(rewardsMint));
    // assert.ok(programState.rewardsTreasury.equals(rewardsTreasuryPda));
    // assert.equal(programState.totalUsers.toString(), "0");
    // assert(programState.bump > 0);
  });

  it("Initializes rewards treasury", async () => {
    // Find treasury PDA
    const [treasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    );

    await program.methods
      .initializeRewards()
      .accounts({
        authority: provider.wallet.publicKey,
        treasury: treasuryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Verify treasury state
    const treasury = await program.account.rewardsTreasury.fetch(treasuryPda);
    assert.ok(treasury.authority.equals(provider.wallet.publicKey));
    assert.equal(treasury.totalTokens.toString(), "0");
    assert.equal(treasury.tokensDistributed.toString(), "0");
    assert(treasury.bump > 0);
  });

  it("Initializes user account", async () => {
    // Find user PDA
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // Get the associated token account for the user
    const userTokenAccount = await getAssociatedTokenAddress(
      rewardsMint,
      provider.wallet.publicKey
    );

    await program.methods
      .initializeUser()
      .accounts({
        user: provider.wallet.publicKey,
        userState: userPda,
        userTokenAccount: userTokenAccount,
        rewardsMint: rewardsMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.web3.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    // Verify user state
    const user = await program.account.user.fetch(userPda);
    assert.ok(user.authority.equals(provider.wallet.publicKey));
    assert.equal(user.tokens.toString(), "0");
    assert.equal(user.totalVotesCast.toString(), "0");
    assert.equal(user.totalVotesReceived.toString(), "0");
    assert.equal(user.commentsCount.toString(), "0");
    assert.equal(user.listingsCount.toString(), "0");
    assert(user.lastRewardClaim.gt(new anchor.BN(0)));
    assert(user.bump > 0);
  });

  it("Fails to initialize program state twice", async () => {
    try {
      await program.methods
        .initializeProgram()
        .accounts({
          authority: provider.wallet.publicKey,
          programState: programStatePda,
          rewardsMint: rewardsMint,
          rewardsTreasury: rewardsTreasuryPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();
      assert.fail("Should have failed to initialize twice");
    } catch (err) {
      assert.ok(err.toString().includes("Error"));
    }
  });

  it("Initializes multiple user accounts", async () => {
    // Create a new user
    const newUser = anchor.web3.Keypair.generate();

    // Airdrop some SOL to the new user
    const airdropSig = await provider.connection.requestAirdrop(
      newUser.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);

    // Find user PDA for new user
    const [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), newUser.publicKey.toBuffer()],
      program.programId
    );

    // Get the associated token account for the new user
    const userTokenAccount = await getAssociatedTokenAddress(
      rewardsMint,
      newUser.publicKey
    );

    await program.methods
      .initializeUser()
      .accounts({
        user: newUser.publicKey,
        userState: userPda,
        userTokenAccount: userTokenAccount,
        rewardsMint: rewardsMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.web3.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([newUser])
      .rpc();

    // Verify new user state
    const user = await program.account.user.fetch(userPda);
    assert.ok(user.authority.equals(newUser.publicKey));
    assert.equal(user.tokens.toString(), "0");
  });
});
