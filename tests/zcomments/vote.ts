import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenHouseRedone } from "../../target/types/open_house_redone";
import { assert } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("open_house_redone_comment_voting", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

  // Test data
  const longitude = new anchor.BN(12345806);
  const latitude = new anchor.BN(789012);
  const currentTimestamp = new anchor.BN(Math.floor(Date.now() / 1000));
  const commentContent = "Test comment for voting!";

  it("Creates a listing with a comment and tests voting", async () => {
    // Create listing
    const [listingPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        provider.wallet.publicKey.toBuffer(),
        longitude.toArrayLike(Buffer, "le", 8),
        latitude.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createListing({
        location: {
          long: longitude,
          lat: latitude,
        },
        creator: provider.wallet.publicKey,
        created: currentTimestamp,
        updated: currentTimestamp,
        status: { active: {} },
      })
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Create comment
    const [commentPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("comment"),
        listingPda.toBuffer(),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .createComment(commentContent)
      .accounts({
        owner: provider.wallet.publicKey,
        listing: listingPda,
        comment: commentPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Create two voters for testing
    const upvoter = anchor.web3.Keypair.generate();
    const downvoter = anchor.web3.Keypair.generate();

    // Airdrop SOL to voters
    await provider.connection.requestAirdrop(
      upvoter.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      downvoter.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    
    // Wait for airdrops to confirm
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Create vote PDAs
    const [upvotePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("comment_vote"),
        commentPda.toBuffer(),
        upvoter.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [downvotePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("comment_vote"),
        commentPda.toBuffer(),
        downvoter.publicKey.toBuffer(),
      ],
      program.programId
    );

    console.log("Initial state:");
    let comment = await program.account.comment.fetch(commentPda);
    console.log("Initial vote count:", comment.voteCount.toString());

    // Submit upvote
    console.log("Submitting upvote...");
    await program.methods
      .voteOnListingComment(true)
      .accounts({
        comment: commentPda,
        vote: upvotePda,
        voter: upvoter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([upvoter])
      .rpc();

    // Verify upvote
    comment = await program.account.comment.fetch(commentPda);
    console.log("Vote count after upvote:", comment.voteCount.toString());
    assert.equal(comment.voteCount.toString(), "1", "Vote count should be 1 after upvote");

    // Verify upvote account
    const upvoteAccount = await program.account.vote.fetch(upvotePda);
    assert.ok(upvoteAccount.owner.equals(upvoter.publicKey));
    assert.ok(upvoteAccount.target.equals(commentPda));
    assert.equal(upvoteAccount.isUpvote, true);

    // Submit downvote
    console.log("Submitting downvote...");
    await program.methods
      .voteOnListingComment(false)
      .accounts({
        comment: commentPda,
        vote: downvotePda,
        voter: downvoter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([downvoter])
      .rpc();

    // Verify final state
    comment = await program.account.comment.fetch(commentPda);
    console.log("Final vote count:", comment.voteCount.toString());
    assert.equal(comment.voteCount.toString(), "0", "Vote count should be 0 after up and down vote");

    // Verify downvote account
    const downvoteAccount = await program.account.vote.fetch(downvotePda);
    assert.ok(downvoteAccount.owner.equals(downvoter.publicKey));
    assert.ok(downvoteAccount.target.equals(commentPda));
    assert.equal(downvoteAccount.isUpvote, false);

    // Test changing vote
    console.log("Testing vote change...");
    await program.methods
      .voteOnListingComment(false)
      .accounts({
        comment: commentPda,
        vote: upvotePda,
        voter: upvoter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([upvoter])
      .rpc();

    // Verify vote change
    comment = await program.account.comment.fetch(commentPda);
    console.log("Vote count after vote change:", comment.voteCount.toString());
    assert.equal(comment.voteCount.toString(), "-2", "Vote count should be -2 after vote change");

    const changedVote = await program.account.vote.fetch(upvotePda);
    assert.equal(changedVote.isUpvote, false, "Vote should be changed to downvote");
  });
});