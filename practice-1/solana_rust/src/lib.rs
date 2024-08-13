use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::{keypair::Keypair, EncodableKey, EncodableKeypair};
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
