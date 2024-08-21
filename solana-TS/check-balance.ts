import "dotenv/config";
import { Keypair } from "@solana/web3.js";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { env_keypair } from "./load-keypair";

const connection = new Connection(clusterApiUrl("devnet"));

const keypair = env_keypair();
const publicKey = new PublicKey(keypair.publicKey.toBase58());

export async function check_balance(connection: Connection, publicKey: PublicKey): Promise<number> {
  let balanceInLamports = await connection.getBalance(publicKey);
  let balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;
  return balanceInSOL;
}

const balance = await check_balance(connection, publicKey)

console.log(
  `The balance for the wallet at address ${publicKey} is: ${balance}`
);
