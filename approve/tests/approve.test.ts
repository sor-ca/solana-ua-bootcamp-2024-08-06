// const anchor = require("@coral-xyz/anchor");

// describe("approve", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const program = anchor.workspace.Approve;
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Transaction } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount
} from "@solana/spl-token";
import { Approve } from "../target/types/approve";
import { Program, BN } from "@coral-xyz/anchor";
import { randomBytes } from "crypto";

export const getRandomBigNumber = (size: number = 8) => {
  return new BN(randomBytes(size));
};

function sleep(millis: number) {
  var t = (new Date()).getTime();
  var i = 0;
  while (((new Date()).getTime() - t) < millis) {
      i++;
  }
}

describe("token-exchange", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Approve as Program<Approve>;;

  let mintATK: PublicKey;
  let mintBTK: PublicKey;
  let aliceTokenAccountATK: PublicKey;
  let bobTokenAccountBTK: PublicKey;
  let bobTokenAccountATK: PublicKey;
  let aliceTokenAccountBTK: PublicKey;

  const alice = Keypair.generate();
  const bob = Keypair.generate();

  const amountATK = 20;
  const amountBTK = 100;

  // Pick a random ID for the new offer.
  const offerId = getRandomBigNumber();


  beforeAll(async () => {

    // Airdrop SOL to Alice and Bob
    await provider.connection.requestAirdrop(alice.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(bob.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);

    const AliceInitBalance = await provider.connection.getBalance(alice.publicKey);
    const BobInitBalance = await provider.connection.getBalance(bob.publicKey);

    sleep(500)

    console.log("Alice's initial balance:", AliceInitBalance);
    console.log("Bob's initial balance:", BobInitBalance);

    // Create ATK mint and associated token account for Alice
    mintATK = await createMint(provider.connection, alice, alice.publicKey, null, 0);
    aliceTokenAccountATK = (await getOrCreateAssociatedTokenAccount(provider.connection, alice, mintATK, alice.publicKey)).address;

    // Create BTK mint and associated token account for Bob
    mintBTK = await createMint(provider.connection, bob, bob.publicKey, null, 0);
    bobTokenAccountBTK = (await getOrCreateAssociatedTokenAccount(provider.connection, bob, mintBTK, bob.publicKey)).address;

    // Mint tokens to Alice and Bob
    await mintTo(provider.connection, alice, mintATK, aliceTokenAccountATK, alice, amountATK);
    await mintTo(provider.connection, bob, mintBTK, bobTokenAccountBTK, bob, amountBTK);

    // Create associated token accounts for the exchanged tokens
    bobTokenAccountATK = (await getOrCreateAssociatedTokenAccount(provider.connection, bob, mintATK, bob.publicKey)).address;
    aliceTokenAccountBTK = (await getOrCreateAssociatedTokenAccount(provider.connection, alice, mintBTK, alice.publicKey)).address;

    // Log initial balances
    const aliceInitialATK = await provider.connection.getTokenAccountBalance(aliceTokenAccountATK);
    const bobInitialBTK = await provider.connection.getTokenAccountBalance(bobTokenAccountBTK);

    console.log("Alice's initial ATK balance:", aliceInitialATK.value.uiAmount);
    console.log("Bob's initial BTK balance:", bobInitialBTK.value.uiAmount);

    sleep(500)
  });

  it("Alice makes an offer to exchange ATK for BTK", async () => {
    try {
      await program.methods
      .makeOffer(
        new anchor.BN(amountATK), 
        new anchor.BN(amountBTK) 
        //offerId
      )
      .accounts({
        maker: alice.publicKey,
        atkMint: mintATK,
        btkMint: mintBTK,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([alice])
      .rpc();

      // Check the approval for the escrow account
      const aliceAccount = await provider.connection.getTokenAccountBalance(aliceTokenAccountATK);
      expect(aliceAccount.value.uiAmount).toBe(20); // Alice's tokens are still in her account but approved for transfer

      const [offerAddress, _offerBump] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          alice.publicKey.toBuffer(),
          //offerId.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      // Check our Offer account contains the correct data
      const offerAccount = await program.account.escrowAccount.fetch(offerAddress);
      expect(offerAccount.maker).toEqual(alice.publicKey);
      expect(offerAccount.atkMint).toEqual(mintATK);
      expect(offerAccount.btkMint).toEqual(mintBTK);
      // expect(offerAccount.makerAtkAmount).toEqual(amountATK);
      // expect(offerAccount.takerBtkAmount).toEqual(amountBTK);

    } catch (error) {
      console.error("Error during make_offer:", error);
      throw error;
    }
  });

  it("Bob takes the offer", async () => {
    try {

      const [escrowAccount, _offerBump] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          alice.publicKey.toBuffer(),
          //offerId.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      await program.methods
        .takeOffer()
        .accounts({
          taker: bob.publicKey,
          escrowAccount: escrowAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([bob])
        .rpc();
  
      // Check balances after the exchange
      const aliceBtkAccount = await provider.connection.getTokenAccountBalance(aliceTokenAccountBTK);
      const bobAtkAccount = await provider.connection.getTokenAccountBalance(bobTokenAccountATK);
      const aliceAtkAccount = await provider.connection.getTokenAccountBalance(aliceTokenAccountATK);
      const bobBtkAccount = await provider.connection.getTokenAccountBalance(bobTokenAccountBTK);

      console.log("Alice's BTK balance:", aliceBtkAccount.value.uiAmount);
      console.log("Bob's ATK balance:", bobAtkAccount.value.uiAmount);
      console.log("Alice's ATK balance after transfer:", aliceAtkAccount.value.uiAmount);
      console.log("Bob's BTK balance after transfer:", bobBtkAccount.value.uiAmount);

      // expect(aliceBtkAccount.value.uiAmount).toBe(100); // Alice should receive 100 BTK
      // expect(bobAtkAccount.value.uiAmount).toBe(20); // Bob should receive 20 ATK
      // expect(aliceAtkAccount.value.uiAmount).toBe(0); // Alice's ATK should be transferred
      // expect(bobBtkAccount.value.uiAmount).toBe(0); // Bob's BTK should be transferred

    } catch (error) {
      console.error("Error during take_offer:", error);
      throw error;
    } 
  });
});