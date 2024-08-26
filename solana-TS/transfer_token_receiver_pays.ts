import {
    Connection,
    PublicKey,
    Keypair,
    Transaction,
    clusterApiUrl,
    TransactionConfirmationStrategy
} from '@solana/web3.js';
import {
    getOrCreateAssociatedTokenAccount,
    createTransferInstruction,
    TOKEN_PROGRAM_ID,
    mintTo,
} from '@solana/spl-token';

import { env_keypair, env_token_mint, receiver_keypair } from './load-keypair';
import { check_balance } from './check-balance';

export async function getTokenBalance(connection: Connection, tokenAccount: PublicKey): Promise<number> {
    const info = await connection.getTokenAccountBalance(tokenAccount);
    if (info.value.uiAmount == null) throw new Error('No balance found');
    return info.value.uiAmount;
}

// Step 1: Sender creates and partially signs the transaction
async function createPartialSignedTransaction(
    connection: Connection,
    sender: Keypair,
    receiverPubkey: PublicKey,
    mint: PublicKey,
    amount: number
): Promise<Uint8Array> {

     // Create or get the associated token accounts
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

    // Create the transfer instruction
    const transferInstruction = createTransferInstruction(
        senderTokenAccount.address,
        receiverTokenAccount.address,
        sender.publicKey,
        amount,
        [],
        TOKEN_PROGRAM_ID
    );

    // Create the transaction and add the instruction
    const transaction = new Transaction().add(transferInstruction);

    // Set the recent blockhash
    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    transaction.feePayer = receiverPubkey; // Receiver will pay the fee

    // Partially sign the transaction by the sender
    transaction.partialSign(sender);

    // Serialize the message (without the signatures)
    const serializedMessage = transaction.serialize({ requireAllSignatures: false });

    return serializedMessage;
}

// Step 2: Receiver finalizes the transaction, signs it, and sends it
export async function finalizeAndSendTransaction(
    connection: Connection,
    receiver: Keypair,
    serializedMessage: Uint8Array
): Promise<string> {
    // Deserialize the transaction
    const transaction = Transaction.from(serializedMessage);

    // The receiver signs the transaction
    transaction.partialSign(receiver);

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

    return signature;
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
    
    // Sender creates and partially signs the transaction
    const serializedMessage = await createPartialSignedTransaction(connection, sender, receiver.publicKey, mint, amount);

    // Receiver finalizes, signs, and sends the transaction
    const signature = await finalizeAndSendTransaction(connection, receiver, serializedMessage);

    console.log('Transaction successful with signature:', signature);

    // receiverSOLbalance = await check_balance(connection, receiver.publicKey);
    // receiverTokenBalance = await getTokenBalance(connection, receiverTokenAccount.address);
    // console.log(`Receiver final SOL balance ${receiverSOLbalance}, token balance ${receiverTokenBalance}`);
})();

//solana balance 