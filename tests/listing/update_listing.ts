import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenHouseRedone } from "../../target/types/open_house_redone";
import { assert } from "chai";

describe("open_house_redone_update", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

  it("Updates a listing to sold", async () => {
    const initialLocation = {
      long: new anchor.BN(777),
      lat: new anchor.BN(888),
    };

    const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        provider.wallet.publicKey.toBuffer(),
        initialLocation.long.toArrayLike(Buffer, "le", 8),
        initialLocation.lat.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createListing({
        location: initialLocation,
        creator: provider.wallet.publicKey,
        created: new anchor.BN(Date.now()),
        updated: new anchor.BN(Date.now()),
        status: { active: {} },
      })
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Update the listing status to sold, leaving location unchanged
    await program.methods
      .updateListing(
        null, // no location change
        { sold: {} } // new status
      )
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
      })
      .rpc();

    const updatedListing = await program.account.listing.fetch(listingPda);
    assert(updatedListing.status.sold !== undefined);
  });

  it("Updates a listing to deleted", async () => {
    const initialLocation = {
      long: new anchor.BN(777),
      lat: new anchor.BN(889),
    };

    const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        provider.wallet.publicKey.toBuffer(),
        initialLocation.long.toArrayLike(Buffer, "le", 8),
        initialLocation.lat.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createListing({
        location: initialLocation,
        creator: provider.wallet.publicKey,
        created: new anchor.BN(Date.now()),
        updated: new anchor.BN(Date.now()),
        status: { active: {} },
      })
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Update the listing status to deleted, leaving location unchanged
    await program.methods
      .updateListing(
        null, // no location change
        { deleted: {} } // new status
      )
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
      })
      .rpc();

    const updatedListing = await program.account.listing.fetch(listingPda);
    assert(updatedListing.status.deleted !== undefined);
  });

  // Additional test for updating location
  it("Updates a listing's location", async () => {
    const initialLocation = {
      long: new anchor.BN(777),
      lat: new anchor.BN(890),
    };

    const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        provider.wallet.publicKey.toBuffer(),
        initialLocation.long.toArrayLike(Buffer, "le", 8),
        initialLocation.lat.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createListing({
        location: initialLocation,
        creator: provider.wallet.publicKey,
        created: new anchor.BN(Date.now()),
        updated: new anchor.BN(Date.now()),
        status: { active: {} },
      })
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const newLocation = {
      long: new anchor.BN(888),
      lat: new anchor.BN(999),
    };

    // Update only the location, leaving status unchanged
    await program.methods
      .updateListing(
        newLocation, // new location
        null // no status change
      )
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
      })
      .rpc();

    const updatedListing = await program.account.listing.fetch(listingPda);
    assert(updatedListing.location.long.eq(newLocation.long));
    assert(updatedListing.location.lat.eq(newLocation.lat));
  });
});
