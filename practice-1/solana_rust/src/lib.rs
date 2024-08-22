use solana_client::rpc_client::RpcClient;
use solana_sdk::message::Message;
//use solana_sdk::pubkey::Pubkey;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::{keypair::Keypair, EncodableKey, EncodableKeypair};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{signature::Signature, system_transaction};

use dotenv::dotenv;
use std::error::Error;

pub const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub const DEVNET_URL: &str = "https://api.devnet.solana.com";

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

pub fn my_keypair() -> Result<Keypair, Box<dyn Error>> {
    let private_key = std::env::var("SECRET_KEY")?;
    let as_array: Vec<u8> = serde_json::from_str(&private_key)?;
    let keypair = Keypair::from_bytes(&as_array)?;
    Ok(keypair)
}

pub fn receiver_pubkey() -> Result<Pubkey, Box<dyn Error>> {
    let receiver_pubkey = Pubkey::from_str(&std::env::var("RECEIVER")?)?;
    Ok(receiver_pubkey)
}

pub fn token_mint() -> Result<Pubkey, Box<dyn Error>> {
    let token_mint = Pubkey::from_str(&std::env::var("TOKEN_MINT")?)?;
    Ok(token_mint)
}

#[test]
fn test_my_keypair() {
    // Load environment variables from .env file
    dotenv().ok();
    let keypair = my_keypair().unwrap();
    dbg!(keypair.encodable_pubkey());
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

#[test]
fn test_request_airdrop() {
    dotenv().ok();
    let me = my_keypair().unwrap();
    let my_pubkey = me.pubkey();

    let rpc_client = RpcClient::new(DEVNET_URL);

    let initial_balance = check_balance(&rpc_client, &my_pubkey).unwrap();
    dbg!(&initial_balance);

    if let Ok(_) = request_airdrop(&rpc_client, &my_pubkey, 1.0) {
        println!("Airdrop finished");
    } else {
        println!("Airdrop failed");
    }

    let final_balance = check_balance(&rpc_client, &my_pubkey).unwrap();
    dbg!(&final_balance);
    assert!(initial_balance < final_balance);
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
fn test_transfer_funds_with_memo() {
    dotenv().ok();
    let sender_keypair = my_keypair().unwrap();
    let memo = "I have transfered SOL with rust";
    let rpc_client = RpcClient::new(DEVNET_URL);
    let receiver_pubkey = receiver_pubkey().unwrap();
    let amount_sol = 0.01;

    let sig = transfer_funds_with_memo(
        memo,
        &rpc_client,
        &sender_keypair,
        &receiver_pubkey,
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
    dotenv().ok();
    let payer = my_keypair().unwrap();
    let mint_authority = payer.pubkey();
    let rpc_client = RpcClient::new(DEVNET_URL);
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
    dotenv().ok();
    let mint = token_mint().unwrap();
    let payer = my_keypair().unwrap();
    let rpc_client = RpcClient::new(DEVNET_URL);
    let owner = receiver_pubkey().unwrap();
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
    dotenv().ok();
    let mint = token_mint().unwrap();
    let payer = my_keypair().unwrap();
    let rpc_client = RpcClient::new(DEVNET_URL);
    let destination = Pubkey::from_str("CgdCJ6HEvDHRTKZtTm7oy8jdQqVMBWFPYBgb5beKR1ot").unwrap();
    let sig = mint_tokens(&rpc_client, &payer, &mint, &destination, &payer, 10).unwrap();
    dbg!(sig);
}

use mpl_token_metadata::ID;

pub fn create_metadata(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    mint_authority: &Keypair,
    update_authority: &Pubkey,
    name: String,
    symbol: String,
    uri: String,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let metadata_seeds = &[b"metadata", ID.as_ref(), mint.as_ref()];
    let token_metadata_program_id = solana_program::pubkey::Pubkey::from(ID.to_bytes());
    let (metadata_pubkey, _) =
        Pubkey::find_program_address(metadata_seeds, &token_metadata_program_id);

    let data = mpl_token_metadata::types::DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0, // Change this if you want to set a fee
        creators: None,             // Optionally add creators
        collection: None,           // Optionally add a collection
        uses: None,                 // Optionally add uses
    };

    let metadata_instruction =
        mpl_token_metadata::instructions::CreateMetadataAccountV3Builder::new()
            .metadata(metadata_pubkey)
            .mint(*mint)
            .mint_authority(mint_authority.pubkey())
            .payer(payer.pubkey())
            .update_authority(*update_authority, true)
            .data(data)
            .is_mutable(true)
            .instruction();

    let transaction = Transaction::new_signed_with_payer(
        &[metadata_instruction],
        Some(&payer.pubkey()),
        &[payer, mint_authority],
        rpc_client.get_latest_blockhash()?,
    );

    let sig = rpc_client.send_and_confirm_transaction(&transaction)?;

    Ok(sig)
}

#[test]
fn test_create_metadata() {
    dotenv().ok();
    //let mint = token_mint().unwrap();
    let mint = Pubkey::from_str("69YQqyZ615x54jEyaBdcPh9SUz7PopFi9maUDv3zGjZC").unwrap();
    let payer = my_keypair().unwrap();
    let rpc_client = RpcClient::new(DEVNET_URL);

    // let name = "My Token".to_string();
    // let symbol = "MTK".to_string();
    // let uri = "https://example.com/mytoken.json".to_string();

    let name = "MGToken".to_string();
    let symbol = "MGT".to_string();
    let uri = "https://example.com/token.json".to_string();

    let sig = create_metadata(
        &rpc_client,
        &payer,
        &mint,
        &payer,
        &payer.pubkey(),
        name,
        symbol,
        uri,
    )
    .unwrap();
    dbg!(sig);
}
