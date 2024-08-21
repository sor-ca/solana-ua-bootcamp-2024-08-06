import "dotenv/config";
import { Keypair, PublicKey } from "@solana/web3.js";

export function env_keypair(): Keypair {
  let privateKey = process.env["SECRET_KEY"];
  if (privateKey === undefined) {
    console.log("Add SECRET_KEY to .env!");
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
