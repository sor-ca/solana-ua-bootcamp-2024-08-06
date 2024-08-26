import "dotenv/config";
import {
  Connection,
  Keypair,
  clusterApiUrl,
} from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, createMultisig, mintTo, getMint } from "@solana/spl-token";

import { env_keypair, from_env, env_recipient } from "./load-keypair";

const payer = env_keypair();

const connection = new Connection(clusterApiUrl("devnet"));

const signer1 = payer;
const signer2 = from_env("SECRET_KEY1");
const signer3 = from_env("SECRET_KEY2");

console.log(signer1.publicKey.toBase58());
console.log(signer2.publicKey.toBase58());
console.log(signer3.publicKey.toBase58());

const multisigKey = await createMultisig(
  connection,
  payer,
  [
    signer1.publicKey,
    signer2.publicKey,
    signer3.publicKey
  ],
  2
);

console.log(`Created 2/3 multisig ${multisigKey.toBase58()}`);

const mint = await createMint(
    connection,
    payer,
    multisigKey,
    multisigKey,
    2
  );

console.log(`Created mint for multisig ${mint.toBase58()}`);

// const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
//   connection,
//   payer,
//   mint,
//   env_recipient()
// );

// await mintTo(
//   connection,
//   payer,
//   mint,
//   associatedTokenAccount.address,
//   multisigKey,
//   1,
//   [
//     signer1,
//     signer2
//   ]
// )

// const mintInfo = await getMint(
//   connection,
//   mint
// )

// console.log(`Minted ${mintInfo.supply} token`);