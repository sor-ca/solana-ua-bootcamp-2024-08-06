use solana_client::rpc_client::RpcClient;
use solana_rust::{
    check_balance, generate_keypair, load_from_env_file, request_airdrop, transfer_funds,
};
use solana_sdk::signer::Signer;

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc_client = RpcClient::new(URL);

    let me = load_from_env_file().unwrap();
    let my_pubkey = me.pubkey();

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

    let receiver = generate_keypair();

    let transfer_amount = 0.5;

    match transfer_funds(&rpc_client, &me, &receiver.pubkey(), transfer_amount) {
        Ok(_) => {
            println!("Transfer of {:?} finished", transfer_amount);
            if let Ok(balance) = check_balance(&rpc_client, &my_pubkey) {
                println!("My balance after transfer: {:?}", balance);
            }
            if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                println!("Receiver balance after transfer: {:?}", balance);
            }
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
