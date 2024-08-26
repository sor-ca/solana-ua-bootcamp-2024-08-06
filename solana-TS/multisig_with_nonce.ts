import "dotenv/config";
import {
  Connection,
  Keypair,
  NONCE_ACCOUNT_LENGTH,
  NonceAccount,
  SystemProgram,
  Transaction,
  TransactionConfirmationStrategy,
  clusterApiUrl,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

import nacl from "tweetnacl";

import { createMint, getOrCreateAssociatedTokenAccount, createMultisig, mintTo, getMint, createMintToInstruction, TOKEN_PROGRAM_ID } from "@solana/spl-token";

import { env_keypair, env_recipient, from_env, from_env_pub, multisig, multisig_mint } from "./load-keypair";

const connection = new Connection(clusterApiUrl("devnet"));

const payer = env_keypair();
const signer2 = from_env("SECRET_KEY1");

const nonceAccount = from_env("NONCE");

const nonceAccountData = await connection.getNonce(
  nonceAccount.publicKey,
  'confirmed',
);

console.log(nonceAccountData);

const nonceInstruction = SystemProgram.nonceAdvance({
  authorizedPubkey: payer.publicKey,
  noncePubkey: nonceAccount.publicKey
});

const nonce = nonceAccountData.nonce;

let receiver = env_recipient();
const mint = multisig_mint();

const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  payer,
  mint,
  receiver
);

console.log(`recepient token ${associatedTokenAccount.address}`);

const mintToTransaction = new Transaction({
  feePayer: payer.publicKey,
  nonceInfo: {nonce, nonceInstruction}
})
  .add(
    createMintToInstruction(
      multisig_mint(),
      associatedTokenAccount.address,
      multisig(),
      1,
      [
        payer,
        signer2,
      ],
      TOKEN_PROGRAM_ID
    )
  );

// Partially sign the transaction by the sender
mintToTransaction.partialSign(payer);

// Serialize the message (without the signatures)
const serializedMessage = mintToTransaction.serialize({ requireAllSignatures: false });

setTimeout(async () => {
  // Deserialize the transaction
  const transaction = Transaction.from(serializedMessage);

  // The receiver signs the transaction
  transaction.partialSign(signer2);
  // Serialize the transaction with all signatures
  const serializedTransaction = transaction.serialize();

  // Send the fully signed transaction
  const signature = await connection.sendRawTransaction(serializedTransaction);

  // Confirm the transaction
  const confirmationStrategy: TransactionConfirmationStrategy = {
      signature,
      blockhash: transaction.recentBlockhash!,
      lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight,
  };

  await connection.confirmTransaction(confirmationStrategy, 'confirmed');

  console.log('Транзакция успешно выполнена:', signature);
}, 3 * 60 * 1000);


