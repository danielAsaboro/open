// // const accountsInTotal = accounts.length
// // const accountPublicKeys = accounts.map(account => account.pubkey)


// // const candyMachineDiscriminator = Buffer.from(sha256.digest('account:CandyMachine')).slice(0, 8);
// // dataSlice: { offset: 0, length: 0 }, // Fetch without any data.
// // const accounts = await connection.getProgramAccounts(candyMachineV2Program, {
// //     filters: [
// //         { memcmp: { offset: 0, bytes: bs58.encode(candyMachineDiscriminator) } }, // Ensure it's a CandyMachine account.
// //     ],
// // })

// const getPage = async (page, perPage) => {
//     const paginatedPublicKeys = accountPublicKeys.slice(
//         (page - 1) * perPage,
//         page * perPage,
//     );

//     if (paginatedPublicKeys.length === 0) {
//         return [];
//     }

//     const accountsWithData = await connection.getMultipleAccountsInfo(paginatedPublicKeys);

//     return accountsWithData;
// }