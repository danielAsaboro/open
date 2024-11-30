// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { OpenHouseRedone } from "../../target/types/open_house_redone";
// import { assert } from "chai";
// import { PublicKey } from "@solana/web3.js";

// describe("open-house-listing-and-voting", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

//   // Test data
//   const longitude = new anchor.BN(123456);
//   const latitude = new anchor.BN(789012);
//   const currentTimestamp = new anchor.BN(Math.floor(Date.now() / 1000));

//   it("Creates a listing and votes on it", async () => {
//     console.log("Starting test...");
    
//     // Derive the listing PDA
//     const [listingPda] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("listing"),
//         provider.wallet.publicKey.toBuffer(),
//         longitude.toArrayLike(Buffer, "le", 8),
//         latitude.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     // Create listing first
//     await program.methods
//       .createListing({
//         location: {
//           long: longitude,
//           lat: latitude,
//         },
//         creator: provider.wallet.publicKey,
//         created: currentTimestamp,
//         updated: currentTimestamp,
//         status: { active: {} },
//       })
//       .accounts({
//         creator: provider.wallet.publicKey,
//         listing: listingPda,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     // Create voter wallet
//     const voter = anchor.web3.Keypair.generate();
//     const airdropSignature = await provider.connection.requestAirdrop(
//       voter.publicKey,
//       1 * anchor.web3.LAMPORTS_PER_SOL
//     );
//     await provider.connection.confirmTransaction(airdropSignature);

//     // Derive vote PDA
//     const [votePda] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("listing_vote"),
//         listingPda.toBuffer(),
//         voter.publicKey.toBuffer(),
//       ],
//       program.programId
//     );

//     console.log("About to submit vote with accounts:", {
//       listing: listingPda.toBase58(),
//       vote: votePda.toBase58(),
//       voter: voter.publicKey.toBase58(),
//     });

//     // Submit vote with exact instruction structure from IDL
//     try {
//       const tx = await program.methods
//         .voteOnListing(true)
//         .accounts({
//           listing: listingPda,
//           vote: votePda,
//           voter: voter.publicKey,
//           systemProgram: anchor.web3.SystemProgram.programId,
//         })
//         .signers([voter])
//         .transaction();

//       console.log("Transaction built successfully");
//       console.log("Transaction:", tx.instructions[0].data);
      
//       const txid = await anchor.web3.sendAndConfirmTransaction(
//         provider.connection,
//         tx,
//         [voter]
//       );
      
//       console.log("Vote transaction confirmed:", txid);
//     } catch (e) {
//       console.error("Detailed error:", e);
//       throw e;
//     }
//   });
// });

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenHouseRedone } from "../../target/types/open_house_redone";
import { assert } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("open-house-listing-and-voting", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

  const longitude = new anchor.BN(123456);
  const latitude = new anchor.BN(789012);
  const currentTimestamp = new anchor.BN(Math.floor(Date.now() / 1000));

  it("Creates a listing and tests both upvoting and downvoting", async () => {
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

    // Create two voters
    const upvoter = anchor.web3.Keypair.generate();
    const downvoter = anchor.web3.Keypair.generate();

    // Airdrop to both voters
    const airdropUp = await provider.connection.requestAirdrop(
      upvoter.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    const airdropDown = await provider.connection.requestAirdrop(
      downvoter.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropUp);
    await provider.connection.confirmTransaction(airdropDown);

    // Initial state
    const listingStart = await program.account.listing.fetch(listingPda);
    console.log("Initial vote count:", listingStart.voteCount);

    // Upvoter PDA
    const [upvotePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing_vote"),
        listingPda.toBuffer(),
        upvoter.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Submit upvote
    console.log("Submitting upvote...");
    await program.methods
      .voteOnListing(true)
      .accounts({
        listing: listingPda,
        vote: upvotePda,
        voter: upvoter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([upvoter])
      .rpc();

    // Verify upvote
    const listingAfterUp = await program.account.listing.fetch(listingPda);
    console.log("Vote count after upvote:", listingAfterUp.voteCount);
    assert.equal(listingAfterUp.voteCount, listingStart.voteCount + 1, "Upvote should increment by 1");

    // Downvoter PDA
    const [downvotePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing_vote"),
        listingPda.toBuffer(),
        downvoter.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Submit downvote
    console.log("Submitting downvote...");
    await program.methods
      .voteOnListing(false)
      .accounts({
        listing: listingPda,
        vote: downvotePda,
        voter: downvoter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([downvoter])
      .rpc();

    // Verify final state
    const listingFinal = await program.account.listing.fetch(listingPda);
    console.log("Final vote count:", listingFinal.voteCount);
    assert.equal(listingFinal.voteCount, listingStart.voteCount, "Vote count should be back to initial after up and down vote");

    // Verify both vote accounts
    const upvoteAccount = await program.account.vote.fetch(upvotePda);
    assert.ok(upvoteAccount.owner.equals(upvoter.publicKey), "Upvote owner should be upvoter");
    assert.ok(upvoteAccount.target.equals(listingPda), "Upvote target should be listing");
    assert.equal(upvoteAccount.isUpvote, true, "Should be an upvote");

    const downvoteAccount = await program.account.vote.fetch(downvotePda);
    assert.ok(downvoteAccount.owner.equals(downvoter.publicKey), "Downvote owner should be downvoter");
    assert.ok(downvoteAccount.target.equals(listingPda), "Downvote target should be listing");
    assert.equal(downvoteAccount.isUpvote, false, "Should be a downvote");
  });
});