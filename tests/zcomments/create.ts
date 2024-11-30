import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenHouseRedone } from "../../target/types/open_house_redone";
import { assert } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("open_house_redone_comments", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

  // Test data
  const longitude = new anchor.BN(1234856);
  const latitude = new anchor.BN(789012);
  const currentTimestamp = new anchor.BN(Math.floor(Date.now() / 1000));
  const commentContent = "This is a test comment!";

  it("Creates a listing and adds a comment", async () => {
    console.log("Creating listing...");
    
    // First create a listing
    const [listingPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        provider.wallet.publicKey.toBuffer(),
        longitude.toArrayLike(Buffer, "le", 8),
        latitude.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    console.log("Listing PDA:", listingPda.toBase58());

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

    console.log("Listing created successfully");

    // Create comment PDA - matches the seeds in your IDL
    const [commentPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("comment"),
        listingPda.toBuffer(),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    console.log("Comment PDA:", commentPda.toBase58());
    console.log("Creating comment...");

    try {
      // Add comment - matches the instruction structure in your IDL
      await program.methods
        .createComment(commentContent)
        .accounts({
          owner: provider.wallet.publicKey,
          listing: listingPda,
          comment: commentPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      console.log("Comment created successfully");

      // Fetch and verify the comment
      const comment = await program.account.comment.fetch(commentPda);
      
      // Verify all fields match what we expect
      assert.ok(comment.owner.equals(provider.wallet.publicKey), "Comment owner should match creator");
      assert.ok(comment.listing.equals(listingPda), "Comment listing reference should match listing PDA");
      assert.equal(comment.content, commentContent, "Comment content should match input");
      assert.equal(comment.voteCount.toString(), "0", "Initial vote count should be 0");
      assert.ok(comment.bump > 0, "Bump seed should be set");

      console.log("Comment verification passed");
    } catch (e) {
      console.error("Error creating comment:", e);
      throw e;
    }
  });
});