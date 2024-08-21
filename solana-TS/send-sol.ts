import "dotenv/config";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  Connection,
  sendAndConfirmTransaction,
  TransactionInstruction
} from "@solana/web3.js";

import { env_keypair } from "./load-keypair";
import { check_balance } from "./check-balance";

const sender = env_keypair();

const connection = new Connection(clusterApiUrl("devnet"));

console.log(`Our public key is: ${sender.publicKey.toBase58()}`);
const recipient_keypair = Keypair.generate();
//const recipient = new PublicKey("SOMEONE_IN_CLASS");
const recipient = new PublicKey(recipient_keypair.publicKey);
console.log(`Attempting to send 0.01 SOL to ${recipient.toBase58()}...`);

const transaction = new Transaction();

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: recipient,
  lamports: 0.01 * LAMPORTS_PER_SOL,
});
transaction.add(sendSolInstruction);


// Get this address from https://spl.solana.com/memo
const memoProgram = new PublicKey(
  "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
);

const memoText = `I have sent 0.01 SOL to ${recipient}`;
const addMemoInstruction = new TransactionInstruction({
  keys: [{ pubkey: sender.publicKey, isSigner: true, isWritable: true }],
  data: Buffer.from(memoText, "utf-8"),
  programId: memoProgram,
});

transaction.add(addMemoInstruction);

const signature = await sendAndConfirmTransaction(connection, transaction, [
  sender,
]);

console.log(`Transaction confirmed, signature: ${signature}!`);
