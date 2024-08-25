import "dotenv/config";
import { Keypair, PublicKey } from "@solana/web3.js";

//DLDCwh36VKZgw7VthptcowHLuVfjJb7WoiAm7zcfV3Mc
export function env_keypair(): Keypair {
  let privateKey = process.env["SECRET_KEY"];
  if (privateKey === undefined) {
    console.log("Add SECRET_KEY to .env!");
    process.exit(1);
  }
  let asArray = Uint8Array.from(JSON.parse(privateKey));
  return Keypair.fromSecretKey(asArray);
}

//SSPUmjtSMzB9ihxzX47UDEjHa33Rz7ZtpVTwCAA3NSn
export function receiver_keypair(): Keypair {
  let privateKey = process.env["RECEIVER_SECRET_KEY"];
  if (privateKey === undefined) {
    console.log("Add RECEIVER_SECRET_KEY to .env!");
    process.exit(1);
  }
  let asArray = Uint8Array.from(JSON.parse(privateKey));
  return Keypair.fromSecretKey(asArray);
}

export function env_token_mint(): PublicKey {
  let tokenKey = process.env["UNKNOWN_TOKEN"];
  if (tokenKey === undefined) {
    console.log("Add UNKNOWN_TOKEN to .env!");
    process.exit(1);
  }
  return new PublicKey(tokenKey);
}

export function env_recipient(): PublicKey {
  let recipient = process.env["RECIPIENT"];
  if (recipient === undefined) {
    console.log("Add RECIPIENT to .env!");
    process.exit(1);
  }
  return new PublicKey(recipient);
}

const keypair = env_keypair();

console.log(`Public key: ${keypair.publicKey.toBase58()}`);
