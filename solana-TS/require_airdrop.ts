import {
  airdropIfRequired,
} from "@solana-developers/helpers";
import "dotenv/config";
import { Keypair } from "@solana/web3.js";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { env_keypair } from "./load-keypair";
import { check_balance } from "./check-balance";

const connection = new Connection(clusterApiUrl("devnet"));
console.log(`Connected to devnet`);
const keypair = env_keypair();
const publicKey = new PublicKey(keypair.publicKey.toBase58());

const init_balance = await check_balance(connection, publicKey);
const min_balance = init_balance + 0.5

await airdropIfRequired(
  connection,
  publicKey,
  1 * LAMPORTS_PER_SOL,
  //0.5 * LAMPORTS_PER_SOL
  min_balance * LAMPORTS_PER_SOL
);

const balance = await check_balance(connection, publicKey);
console.log(`The balance for the wallet at address ${publicKey} is: ${balance}`); 