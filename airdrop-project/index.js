const {
  Connection,
  PublicKey,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

// used to create a wallet
const wallet = new Keypair();
const publicKey = new PublicKey(wallet._keypair.publicKey);
const secretKey = wallet._keypair.secretKey;

console.log("publicKey", publicKey, "secretKey", secretKey);

const getWalletBalance = async () => {
  try {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const walletBalance = await connection.getBalance(publicKey);
    console.log("walletBalance ==", walletBalance);
  } catch (error) {
    console.log(error);
  }
};

const airDropSol = async () => {
  try {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    // there are 1 billion lamports per SOL
    const fromAirDropSignature = await connection.requestAirdrop(
      publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(fromAirDropSignature);
  } catch (error) {
    console.log(error);
  }
};

const main = async () => {
  await getWalletBalance();
  await airDropSol();
  await getWalletBalance();
};

main();
