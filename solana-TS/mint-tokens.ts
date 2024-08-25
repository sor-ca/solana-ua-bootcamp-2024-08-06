import "dotenv/config";
import { Connection, clusterApiUrl } from "@solana/web3.js";
import { mintTo, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { getExplorerLink } from "@solana-developers/helpers";
import { env_keypair, env_recipient, env_token_mint } from "./load-keypair";

const sender = env_keypair();

const connection = new Connection(clusterApiUrl("devnet"));

// Our token has two decimal places
const MINOR_UNITS_PER_MAJOR_UNITS = Math.pow(10, 2);

const tokenMintAccount = env_token_mint();

let recipient = env_recipient();
let recipientTokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  sender,
  tokenMintAccount,
  recipient
);
const recipientAssociatedTokenAccount = recipientTokenAccount.address;

const transactionSignature = await mintTo(
  connection,
  sender,
  tokenMintAccount,
  recipientAssociatedTokenAccount,
  sender,
  10 * MINOR_UNITS_PER_MAJOR_UNITS
);

const link = getExplorerLink("transaction", transactionSignature, "devnet");

console.log(`Success! Mint Token Transaction: ${link}`);
