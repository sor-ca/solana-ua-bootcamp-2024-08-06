import {
    Connection,
    PublicKey,
    Keypair,
    Transaction,
    SystemProgram,
    clusterApiUrl,
} from '@solana/web3.js';
import {
    getOrCreateAssociatedTokenAccount,
    createTransferInstruction,
    TOKEN_PROGRAM_ID,
    mintTo,
} from '@solana/spl-token';
import { env_keypair, receiver_keypair, env_token_mint, from_env } from './load-keypair';
import { check_balance } from './check-balance';
import { finalizeAndSendTransaction, getTokenBalance } from './transfer_token_receiver_pays';

async function transferTokenWithDurableNonce(
    connection: Connection,
    sender: Keypair,
    receiverPubkey: PublicKey,
    mint: PublicKey,
    amount: number,
    nonceAccountPubkey: PublicKey,
    nonce_auth: PublicKey,
) {
    // Step 1: Get or create the associated token accounts
    const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        sender,
        mint,
        sender.publicKey
    );

    const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        sender,
        mint,
        receiverPubkey
    );

    // Step 2: Get the nonce value and the blockhash
    const { nonce } = await connection.getNonce(nonceAccountPubkey);

    // Step 3: Create the transfer instruction
    const transferInstruction = createTransferInstruction(
        senderTokenAccount.address,
        receiverTokenAccount.address,
        sender.publicKey,
        amount,
        [],
        TOKEN_PROGRAM_ID
    );

    // Step 4: Create the nonce advance instruction (if needed)
    const nonceAdvanceInstruction = SystemProgram.nonceAdvance({
        noncePubkey: nonceAccountPubkey,
        authorizedPubkey: nonce_auth,
    });

    // Step 5: Create the transaction, add the instructions, and set the nonce
    const transaction = new Transaction().add(nonceAdvanceInstruction, transferInstruction);
    transaction.recentBlockhash = nonce;
    transaction.feePayer = receiverPubkey; // The receiver will pay the fee

    // Step 6: Sender partially signs the transaction
    transaction.partialSign(sender);

    // Serialize the message (without the signatures)
    const serializedMessage = transaction.serialize({ requireAllSignatures: false });

    return serializedMessage;
}

// Example usage
(async () => {
    const connection = new Connection(clusterApiUrl("devnet"));

    const sender = env_keypair();
    const receiver = receiver_keypair();
    const mint = env_token_mint();

    const amount = 100;  // Number of tokens to transfer (adjust decimals as necessary)

    const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        sender,
        mint,
        sender.publicKey
    );

    //mint tokens to sender token account before transfering
    await mintTo(
        connection,
        sender,
        mint,
        senderTokenAccount.address,
        sender,
        amount
    );

    // EcogY4ea4TjFTzAuPkRZawHawj9RgHmsqsZF5Dv991hV
    const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        sender,
        mint,
        receiver.publicKey
    );

    console.log(`Receiver pubkey ${receiver.publicKey}`);
    console.log(`Receiver token account ${receiverTokenAccount.address}`);

    let receiverSOLbalance = await check_balance(connection, receiver.publicKey);
    let receiverTokenBalance = await getTokenBalance(connection, receiverTokenAccount.address);
    console.log(`Receiver initial SOL balance ${receiverSOLbalance}, token balance ${receiverTokenBalance}`);

    const nonceAccount = from_env("NONCE").publicKey; // The nonce account public key

    // Step 1: Sender creates and partially signs the transaction with durable nonce
    const serializedMessage = await transferTokenWithDurableNonce(connection, sender, receiver.publicKey, mint, amount, nonceAccount, sender.publicKey);

    // The receiver has up to 3 minutes to finalize and send the transaction
    setTimeout(async () => {
        const signature = await finalizeAndSendTransaction(connection, receiver, serializedMessage);
        console.log('Transaction successful with signature:', signature);
    }, 180000); // 3 minutes delay
})();



