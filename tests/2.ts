// // - create_listing(listing_data: ListingData)
// // - update_listing(new_location: Option<Location>, new_status: Option<ListingStatus>)

// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { OpenHouseRedone } from "../target/types/open_house_redone";
// import { assert } from "chai";
// import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";

// describe("OpenHouse - Listing Management", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

//   // Test accounts
//   let creator: Keypair;
//   let listing: PublicKey;

//   // Test data
//   const testLocation = {
//     long: new anchor.BN(123),
//     lat: new anchor.BN(456),
//     address: "123 Test St",
//     nearest_bus_stop: "Test Station",
//     landmark: "Test Landmark",
//   };

//   beforeEach(async () => {
//     // Create fresh keypair for each test
//     creator = Keypair.generate();

//     // Fund creator account
//     const signature = await provider.connection.requestAirdrop(
//       creator.publicKey,
//       1 * anchor.web3.LAMPORTS_PER_SOL
//     );
//     await provider.connection.confirmTransaction(signature);

//     // Derive listing PDA
//     [listing] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("listing"),
//         creator.publicKey.toBuffer(),
//         testLocation.long.toArrayLike(Buffer, "le", 8),
//         testLocation.lat.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//   });

//   it("Creates a listing", async () => {
//     try {
//       const listingData = {
//         location: testLocation,
//         creator: creator.publicKey,
//         created: new anchor.BN(Math.floor(Date.now() / 1000)),
//         updated: new anchor.BN(Math.floor(Date.now() / 1000)),
//         status: { active: {} },
//       };

//       await program.methods
//         .createListing(listingData)
//         .accounts({
//           creator: creator.publicKey,
//           listing,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([creator])
//         .rpc();

//       // Fetch and verify the listing
//       const listingAccount = await program.account.listing.fetch(listing);
//       assert.ok(listingAccount.creator.equals(creator.publicKey));
//       assert.ok(listingAccount.location.long.eq(testLocation.long));
//       assert.ok(listingAccount.location.lat.eq(testLocation.lat));
//       assert.equal(listingAccount.location.address, testLocation.address);
//       assert.equal(
//         listingAccount.location.nearest_bus_stop,
//         testLocation.nearest_bus_stop
//       );
//       assert.equal(listingAccount.status.active !== undefined, true);
//       assert.equal(listingAccount.vote_count, 0);
//     } catch (err) {
//       console.error("Error creating listing:", err);
//       throw err;
//     }
//   });

//   it("Updates listing location", async () => {
//     // First create the listing
//     const listingData = {
//       location: testLocation,
//       creator: creator.publicKey,
//       created: new anchor.BN(Math.floor(Date.now() / 1000)),
//       updated: new anchor.BN(Math.floor(Date.now() / 1000)),
//       status: { active: {} },
//     };

//     await program.methods
//       .createListing(listingData)
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();

//     // Update the location
//     const newLocation = {
//       ...testLocation,
//       long: new anchor.BN(789),
//       lat: new anchor.BN(101),
//       address: "456 New St",
//     };

//     await program.methods
//       .updateListing(newLocation, null)
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//       })
//       .signers([creator])
//       .rpc();

//     // Verify the update
//     const updatedListing = await program.account.listing.fetch(listing);
//     assert.ok(updatedListing.location.long.eq(newLocation.long));
//     assert.ok(updatedListing.location.lat.eq(newLocation.lat));
//     assert.equal(updatedListing.location.address, newLocation.address);
//   });

//   it("Updates listing status to sold", async () => {
//     // First create the listing
//     const listingData = {
//       location: testLocation,
//       creator: creator.publicKey,
//       created: new anchor.BN(Math.floor(Date.now() / 1000)),
//       updated: new anchor.BN(Math.floor(Date.now() / 1000)),
//       status: { active: {} },
//     };

//     await program.methods
//       .createListing(listingData)
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();

//     // Update status to sold
//     await program.methods
//       .updateListing(null, { sold: {} })
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//       })
//       .signers([creator])
//       .rpc();

//     // Verify the status update
//     const updatedListing = await program.account.listing.fetch(listing);
//     assert.equal(updatedListing.status.sold !== undefined, true);
//   });

//   it("Prevents unauthorized updates", async () => {
//     // Create listing with original creator
//     const listingData = {
//       location: testLocation,
//       creator: creator.publicKey,
//       created: new anchor.BN(Math.floor(Date.now() / 1000)),
//       updated: new anchor.BN(Math.floor(Date.now() / 1000)),
//       status: { active: {} },
//     };

//     await program.methods
//       .createListing(listingData)
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();

//     // Try to update with different user
//     const unauthorizedUser = Keypair.generate();
//     await provider.connection.requestAirdrop(
//       unauthorizedUser.publicKey,
//       1 * anchor.web3.LAMPORTS_PER_SOL
//     );

//     try {
//       await program.methods
//         .updateListing(testLocation, null)
//         .accounts({
//           creator: unauthorizedUser.publicKey,
//           listing,
//         })
//         .signers([unauthorizedUser])
//         .rpc();

//       assert.fail("Should not allow unauthorized update");
//     } catch (err) {
//       assert.ok(err);
//     }
//   });

//   it("Prevents invalid status transitions", async () => {
//     // First create the listing
//     const listingData = {
//       location: testLocation,
//       creator: creator.publicKey,
//       created: new anchor.BN(Math.floor(Date.now() / 1000)),
//       updated: new anchor.BN(Math.floor(Date.now() / 1000)),
//       status: { active: {} },
//     };

//     await program.methods
//       .createListing(listingData)
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();

//     // Update to sold
//     await program.methods
//       .updateListing(null, { sold: {} })
//       .accounts({
//         creator: creator.publicKey,
//         listing,
//       })
//       .signers([creator])
//       .rpc();

//     // Try to update from sold back to active
//     try {
//       await program.methods
//         .updateListing(null, { active: {} })
//         .accounts({
//           creator: creator.publicKey,
//           listing,
//         })
//         .signers([creator])
//         .rpc();

//       assert.fail("Should not allow invalid status transition");
//     } catch (err) {
//       assert.ok(err);
//     }
//   });
// });
