use solana_client::rpc_client::RpcClient;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::{keypair::Keypair, EncodableKey, EncodableKeypair};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{signature::Signature, system_transaction};
use std::error::Error;

pub const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub fn generate_keypair() -> Keypair {
    Keypair::new()
}

pub fn generate_personal_keypair(personal: &str) -> Result<Keypair, ()> {
    //do not work too long
    for _ in 0..10000000 {
        let keypair = Keypair::new();
        let pubkey_str = keypair.encodable_pubkey().to_string();
        if pubkey_str.starts_with(personal) {
            return Ok(keypair);
        }
    }
    Err(())
}

pub fn write_to_env_file(keypair: &Keypair) -> Result<String, Box<dyn Error>> {
    std::fs::File::create(".env")?;
    keypair.write_to_file(".env")
}

pub fn load_from_env_file() -> Result<Keypair, Box<dyn Error>> {
    Keypair::read_from_file(".env")
}

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn request_airdrop(
    rpc_client: &RpcClient,
    pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn transfer_funds(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;

    let sig = rpc_client.send_and_confirm_transaction(&system_transaction::transfer(
        &sender_keypair,
        &receiver_pub_key,
        amount_lamports,
        rpc_client.get_latest_blockhash()?,
    ))?;

    Ok(sig)
}

use solana_program::instruction::{AccountMeta, Instruction};
use std::str::FromStr;

fn create_memo_instruction(memo: &str, signer: Pubkey) -> Instruction {
    // Memo program ID
    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap();

    // Memo data (converted to bytes)
    let memo_data = memo.as_bytes().to_vec();

    // Instruction creation
    Instruction {
        program_id: memo_program_id,
        accounts: vec![AccountMeta::new(signer, true)], // signers of the transaction
        data: memo_data,                                // memo data
    }
}

pub fn transfer_funds_with_memo(
    memo: &str,
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;
    let sender_pubkey = sender_keypair.encodable_pubkey();
    let memo_instruction = create_memo_instruction(memo, sender_pubkey);
    let transfer_instruction =
        system_instruction::transfer(&sender_pubkey, receiver_pub_key, lamports);
    let message = Message::new(
        &[memo_instruction, transfer_instruction],
        Some(&sender_pubkey),
    );
    let transaction = Transaction::new(
        &[sender_keypair],
        message,
        rpc_client.get_latest_blockhash()?,
    );
    let sig = rpc_client.send_and_confirm_transaction(&transaction)?;
    Ok(sig)
}

#[test]
fn test_generate_keypair() {
    let keypair = generate_keypair();
    dbg!(keypair.secret());
    dbg!(keypair.encodable_pubkey());
}

#[test]
fn test_generate_personal_keypair() {
    let keypair = generate_personal_keypair("MG").unwrap();
    assert!(keypair.encodable_pubkey().to_string().starts_with("MG"));
}

#[test]
#[ignore]
fn write_personal_key_into_env() {
    let keypair = generate_personal_keypair("MG").unwrap();
    write_to_env_file(&keypair).unwrap();
}

#[test]
fn test_load_from_file() {
    let keypair = generate_keypair();
    write_to_env_file(&keypair).unwrap();
    let keypair_from_file = load_from_env_file().unwrap();
    assert_eq!(
        keypair.encodable_pubkey(),
        keypair_from_file.encodable_pubkey()
    );
}

#[test]
fn test_transfer_funds_with_memo() {
    let sender_keypair = load_from_env_file().unwrap();
    let memo = "I have transfered SOL with rust";
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let receiver_pub_key =
        Pubkey::from_str("Eej3zf2RjHdVM4cCAwYAXwtPjWF1osekus5W3hzaqY9K").unwrap();
    let amount_sol = 0.01;

    let sig = transfer_funds_with_memo(
        memo,
        &rpc_client,
        &sender_keypair,
        &receiver_pub_key,
        amount_sol,
    )
    .unwrap();
    dbg!(sig);
}

use solana_sdk::program_pack::Pack;
use solana_sdk::signature::Signer;
use spl_token::{instruction::initialize_mint, state::Mint};

pub fn create_token(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    // Create the mint account
    let mint_account = Keypair::new();
    let mint_pubkey = mint_account.pubkey();

    // Create the transaction to create the token mint
    let lamports = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),
        &mint_pubkey,
        lamports,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_instruction = initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        mint_authority,
        freeze_authority,
        decimals,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&payer.pubkey()),
        &[payer, &mint_account],
        rpc_client.get_latest_blockhash()?,
    );

    // Send and confirm the transaction
    rpc_client.send_and_confirm_transaction(&transaction)?;

    Ok(mint_pubkey)
}

#[test]
fn test_create_token() {
    let payer = load_from_env_file().unwrap();
    let mint_authority = payer.pubkey();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let token_key = create_token(&rpc_client, &payer, &mint_authority, None, 2).unwrap();
    dbg!(token_key);
}

use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;

pub fn create_receiver_associated_token_account(
    rpc_client: &RpcClient,
    payer: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let associated_token_address = get_associated_token_address(owner, mint);

    let create_ata_instruction =
        create_associated_token_account(&payer.pubkey(), owner, mint, &spl_token::id());

    let transaction = Transaction::new_signed_with_payer(
        &[create_ata_instruction],
        Some(&payer.pubkey()),
        &[payer],
        rpc_client.get_latest_blockhash()?,
    );

    rpc_client.send_and_confirm_transaction(&transaction)?;

    Ok(associated_token_address)
}

#[test]
fn test_create_receiver_associated_token_account() {
    let mint = Pubkey::from_str("DzS38m6BL4nLnVroGJeHfnjLH5aAXPiRCpvP7ztLuNn8").unwrap();
    let payer = load_from_env_file().unwrap();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let owner = Pubkey::from_str("Eej3zf2RjHdVM4cCAwYAXwtPjWF1osekus5W3hzaqY9K").unwrap();
    let associated_token_address =
        create_receiver_associated_token_account(&rpc_client, &payer, &owner, &mint).unwrap();
    dbg!(associated_token_address);
}

use spl_token::instruction::mint_to;

pub fn mint_tokens(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    destination: &Pubkey,
    authority: &Keypair,
    amount: u64,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let mint_to_instruction = mint_to(
        &spl_token::id(),
        mint,
        destination,
        &authority.pubkey(),
        &[],
        amount,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction],
        Some(&payer.pubkey()),
        &[payer, authority],
        rpc_client.get_latest_blockhash()?,
    );

    let sig = rpc_client.send_and_confirm_transaction(&transaction)?;

    Ok(sig)
}

#[test]
fn test_mint_tokens() {
    let mint = Pubkey::from_str("DzS38m6BL4nLnVroGJeHfnjLH5aAXPiRCpvP7ztLuNn8").unwrap();
    let payer = load_from_env_file().unwrap();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let destination = Pubkey::from_str("CgdCJ6HEvDHRTKZtTm7oy8jdQqVMBWFPYBgb5beKR1ot").unwrap();
    let sig =
        mint_tokens(&rpc_client, &payer, &mint, &destination, &payer, 10).unwrap();
    dbg!(sig);
}
