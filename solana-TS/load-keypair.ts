import "dotenv/config";
import { Keypair, PublicKey } from "@solana/web3.js";

export function from_env(name: string): Keypair {
  let privateKey = process.env[name];
  if (privateKey === undefined) {
    console.log(`Add ${name} to .env!`);
    process.exit(1);
  }
  let asArray = Uint8Array.from(JSON.parse(privateKey));
  return Keypair.fromSecretKey(asArray);
}

//DLDCwh36VKZgw7VthptcowHLuVfjJb7WoiAm7zcfV3Mc
export function env_keypair(): Keypair {
  return from_env("SECRET_KEY");
}

//SSPUmjtSMzB9ihxzX47UDEjHa33Rz7ZtpVTwCAA3NSn
export function receiver_keypair(): Keypair {
  return from_env("RECEIVER_SECRET_KEY");
}

export function from_env_pub(name: string): PublicKey {
  let pubKey = process.env[name];
  if (pubKey === undefined) {
    console.log(`Add ${name} to .env!`);
    process.exit(1);
  }
  return new PublicKey(pubKey);
}

export function env_token_mint(): PublicKey {
  return from_env_pub("UNKNOWN_TOKEN");
}

export function env_recipient(): PublicKey {
  return from_env_pub("RECIPIENT");
}

//multisig with signers with SECRET_KEY, SECRET_KEY1, SECRET_KEY2
export function multisig(): PublicKey {
  return from_env_pub("MULTISIG");
}

//multisig mint for the below multisig
export function multisig_mint(): PublicKey {
  return from_env_pub("MULTISIG_MINT");
}

const keypair = env_keypair();

console.log(`Public key: ${keypair.publicKey.toBase58()}`);

//env1 7A3fZmrmjtXvN9YiybRJ7NrZ8XABuomBnx6nwTYbECMJ
//env2 CiAQRwapUtzQnkp5G5tdny3tuZadSB2e9W7J5rxFcp1P
//nonce G2cQabooJd4d97ZtJZcrsfquXXdnZJSxotTsC3ZByy3G
