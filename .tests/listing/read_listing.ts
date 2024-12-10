// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { OpenHouseRedone } from "../../target/types/open_house_redone";
// import { assert } from "chai";

// describe("open_house_redone_update", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

//   it("Updates a listing location", async () => {
//     // Initial location and listing
//     const location = {
//       long: new anchor.BN(123),
//       lat: new anchor.BN(456),
//     };

//     const listing = {
//       location,
//       creator: provider.wallet.publicKey,
//       created: new anchor.BN(Date.now()),
//       updated: new anchor.BN(Date.now()),
//       status: { active: {} },
//     };

//     // Create initial PDA
//     const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("listing"),
//         provider.wallet.publicKey.toBuffer(),
//         location.long.toArrayLike(Buffer, "le", 8),
//         location.lat.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     // Create initial listing
//     await program.methods
//       .createListing(listing)
//       .accounts({
//         creator: provider.wallet.publicKey,
//         listing: listingPda,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     // Update location
//     const updatedLocation = {
//       long: new anchor.BN(789),
//       lat: new anchor.BN(101),
//     };

//     await program.methods
//       .updateListing({ some: updatedLocation }, null)
//       .accounts({
//         creator: provider.wallet.publicKey,
//         listing: listingPda,
//       })
//       .rpc();

//     // Verify the update
//     const listingAccount = await program.account.listing.fetch(listingPda);
//     assert(listingAccount.creator.equals(provider.wallet.publicKey));
//     assert(listingAccount.location.long.eq(updatedLocation.long));
//     assert(listingAccount.location.lat.eq(updatedLocation.lat));
//     assert(listingAccount.created.gt(new anchor.BN(0)));
//     assert(listingAccount.updated.gt(new anchor.BN(0)));
//     assert(listingAccount.status.active !== undefined);
//   });

//   it("Updates a listing status", async () => {
//     const location = {
//       long: new anchor.BN(123),
//       lat: new anchor.BN(456),
//     };

//     const listing = {
//       location,
//       creator: provider.wallet.publicKey,
//       created: new anchor.BN(Date.now()),
//       updated: new anchor.BN(Date.now()),
//       status: { active: {} },
//     };

//     const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("listing"),
//         provider.wallet.publicKey.toBuffer(),
//         location.long.toArrayLike(Buffer, "le", 8),
//         location.lat.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     await program.methods
//       .createListing(listing)
//       .accounts({
//         creator: provider.wallet.publicKey,
//         listing: listingPda,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     await program.methods
//       .updateListing(null, { some: { sold: {} } })
//       .accounts({
//         creator: provider.wallet.publicKey,
//         listing: listingPda,
//       })
//       .rpc();

//     const listingAccount = await program.account.listing.fetch(listingPda);
//     assert(listingAccount.status.sold !== undefined);
//   });

//   it("Fails to update another user's listing", async () => {
//     const location = {
//       long: new anchor.BN(123),
//       lat: new anchor.BN(456),
//     };

//     const listing = {
//       location,
//       creator: provider.wallet.publicKey,
//       created: new anchor.BN(Date.now()),
//       updated: new anchor.BN(Date.now()),
//       status: { active: {} },
//     };

//     const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("listing"),
//         provider.wallet.publicKey.toBuffer(),
//         location.long.toArrayLike(Buffer, "le", 8),
//         location.lat.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );

//     await program.methods
//       .createListing(listing)
//       .accounts({
//         creator: provider.wallet.publicKey,
//         listing: listingPda,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     const otherUser = anchor.web3.Keypair.generate();

//     try {
//       await program.methods
//         .updateListing({ some: location }, null)
//         .accounts({
//           creator: otherUser.publicKey,
//           listing: listingPda,
//         })
//         .signers([otherUser])
//         .rpc();

//       assert.fail("Expected error");
//     } catch (error) {
//       // Expected error
//       assert(error);
//     }
//   });
// });
