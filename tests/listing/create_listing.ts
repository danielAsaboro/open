import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenHouseRedone } from "../../target/types/open_house_redone";
import { assert } from "chai";

describe("open_house_redone_create", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.OpenHouseRedone as Program<OpenHouseRedone>;

  it("Creates a listing", async () => {
    const location = {
      long: new anchor.BN(123),
      lat: new anchor.BN(456),
    };

    const listing = {
      location,
      creator: provider.wallet.publicKey,
      created: new anchor.BN(Date.now()),
      updated: new anchor.BN(Date.now()),
      status: { active: {} },
    };

    const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        provider.wallet.publicKey.toBuffer(),
        location.long.toArrayLike(Buffer, "le", 8),
        location.lat.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createListing(listing)
      .accounts({
        creator: provider.wallet.publicKey,
        listing: listingPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const listingAccount = await program.account.listing.fetch(listingPda);
    assert(listingAccount.creator.equals(provider.wallet.publicKey));
    assert(listingAccount.location.long.eq(location.long));
    assert(listingAccount.location.lat.eq(location.lat));
    assert(listingAccount.created.gt(new anchor.BN(0)));
    assert(listingAccount.updated.gt(new anchor.BN(0)));
    assert(listingAccount.status.active !== undefined);
  });

  

  it("Creates multiple listings", async () => {
    const locations = [
      { long: new anchor.BN(100), lat: new anchor.BN(200) },
      { long: new anchor.BN(300), lat: new anchor.BN(400) },
      { long: new anchor.BN(500), lat: new anchor.BN(600) },
    ];

    const listings = await Promise.all(
      locations.map(async (location) => {
        const listing = {
          location,
          creator: provider.wallet.publicKey,
          created: new anchor.BN(Date.now()),
          updated: new anchor.BN(Date.now()),
          status: { active: {} },
        };

        const [listingPda] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("listing"),
            provider.wallet.publicKey.toBuffer(),
            location.long.toArrayLike(Buffer, "le", 8),
            location.lat.toArrayLike(Buffer, "le", 8),
          ],
          program.programId
        );

        await program.methods
          .createListing(listing)
          .accounts({
            creator: provider.wallet.publicKey,
            listing: listingPda,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const listingAccount = await program.account.listing.fetch(listingPda);
        assert(listingAccount.creator.equals(provider.wallet.publicKey));
        assert(listingAccount.location.long.eq(location.long));
        assert(listingAccount.location.lat.eq(location.lat));
        assert(listingAccount.created.gt(new anchor.BN(0)));
        assert(listingAccount.updated.gt(new anchor.BN(0)));
        assert(listingAccount.status.active !== undefined);

        return { pda: listingPda, account: listingAccount };
      })
    );

    const allListings = await program.account.listing.all();
    assert.equal(allListings.length, locations.length + 1);
  });

  // it("Can filter listings by author", async () => {
  //   const authorPublicKey = provider.wallet.publicKey;
  //   const listings = await program.account.listing.all([
  //     {
  //       memcmp: {
  //         offset: 8, // Discriminator.
  //         bytes: authorPublicKey.toBase58(),
  //       },
  //     },
  //   ]);

  //   assert.equal(listings.length, 4);
  //   assert.ok(
  //     listings.every((eachListing) => {
  //       return (
  //         eachListing.account.creator.toBase58() === authorPublicKey.toBase58()
  //       );
  //     })
  //   );
  // });
});
