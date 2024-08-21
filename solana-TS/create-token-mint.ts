import "dotenv/config";
import {
  getExplorerLink,
} from "@solana-developers/helpers";
import {
  Keypair,
  clusterApiUrl,
  Connection,
} from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { env_keypair } from "./load-keypair";

const sender = env_keypair();

const connection = new Connection(clusterApiUrl("devnet"));

console.log(`Our public key is: ${sender.publicKey.toBase58()}`);
const tokenMint = await createMint(
  connection,
  sender,
  sender.publicKey,
  null,
  2
);

const link = getExplorerLink("address", tokenMint.toString(), "devnet");

console.log(`Token Mint: ${link}`);
