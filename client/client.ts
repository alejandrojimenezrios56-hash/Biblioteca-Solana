// Cliente 

console.log("Client wallet address:", pg.wallet.publicKey.toString());
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Client balance available for car rental: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
