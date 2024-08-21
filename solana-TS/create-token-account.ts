import "dotenv/config";
import { getExplorerLink } from "@solana-developers/helpers";
import {
  Connection,
  Keypair,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { env_keypair, env_recipient, env_token_mint } from "./load-keypair";

const sender = env_keypair();

const connection = new Connection(clusterApiUrl("devnet"));

console.log(
  `Our pubic key is: ${sender.publicKey.toBase58()}`
);

const tokenMintAccount = env_token_mint();
const recipient = env_recipient();

const tokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  sender,
  tokenMintAccount,
  recipient
);

console.log(`Token Account: ${tokenAccount.address.toBase58()}`);

const link = getExplorerLink(
  "address",
  tokenAccount.address.toBase58(),
  "devnet"
);

console.log(`Created token account: ${link}`);
