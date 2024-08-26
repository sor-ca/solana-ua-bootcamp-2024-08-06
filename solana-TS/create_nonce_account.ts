import "dotenv/config";
import {
  Connection,
  NONCE_ACCOUNT_LENGTH,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

import { env_keypair, from_env } from "./load-keypair";

const connection = new Connection(clusterApiUrl("devnet"));

const nonce_auth = env_keypair();;
const nonceAccount = from_env("NONCE");


const minimumAmount = await connection.getMinimumBalanceForRentExemption(
  NONCE_ACCOUNT_LENGTH,
);

// Form CreateNonceAccount transaction
const transaction = new Transaction()
  .add(
  SystemProgram.createNonceAccount({
    fromPubkey: nonce_auth.publicKey, //fee payer
    noncePubkey: nonceAccount.publicKey,
    authorizedPubkey: nonce_auth.publicKey, //nonce authority
    lamports: minimumAmount,
  }),
);

await sendAndConfirmTransaction(connection, transaction, [nonce_auth, nonceAccount])