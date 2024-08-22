import "dotenv/config";
import { getExplorerLink } from "@solana-developers/helpers";
import {
  Connection,
  Keypair,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, createMultisig, mintTo, getMint } from "@solana/spl-token";

import { env_keypair, env_recipient, env_token_mint } from "./load-keypair";

const payer = env_keypair();

const connection = new Connection(clusterApiUrl("devnet"));

const signer1 = Keypair.generate();
const signer2 = Keypair.generate();
const signer3 = Keypair.generate();

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

const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
  connection,
  payer,
  mint,
  signer1.publicKey
);

await mintTo(
  connection,
  payer,
  mint,
  associatedTokenAccount.address,
  multisigKey,
  1,
  [
    signer1,
    signer2
  ]
)

const mintInfo = await getMint(
  connection,
  mint
)

console.log(`Minted ${mintInfo.supply} token`);