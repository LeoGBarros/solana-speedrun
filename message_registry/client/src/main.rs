//! Cliente de interação com o programa Message Registry na devnet.
//! Envia uma transação `initialize` (se a conta ainda não existir) e uma
//! `update_message`, imprimindo as assinaturas para conferência no Explorer.

use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::read_keypair_file,
    signer::Signer,
    transaction::Transaction,
};
use std::str::FromStr;

const PROGRAM_ID: &str = "DBfznUhc47xnK5B1wC6zWcqtZzWqu2MTNZckWBi6w5fX";
const RPC_URL: &str = "https://api.devnet.solana.com";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

// Discriminators Anchor = primeiros 8 bytes de sha256("global:<nome>").
const INIT_DISC: [u8; 8] = [0xaf, 0xaf, 0x6d, 0x1f, 0x0d, 0x98, 0x9b, 0xed];
const UPDATE_DISC: [u8; 8] = [0x17, 0x87, 0x22, 0xd3, 0x60, 0x78, 0x6b, 0x09];

/// Codifica os dados da instrução: discriminator + String (borsh: u32 len + bytes).
fn encode_string_ix(disc: [u8; 8], text: &str) -> Vec<u8> {
    let mut data = disc.to_vec();
    data.extend_from_slice(&(text.len() as u32).to_le_bytes());
    data.extend_from_slice(text.as_bytes());
    data
}

fn main() {
    let rpc = RpcClient::new(RPC_URL.to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let system_program_id = Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap();

    let wallet_path = format!("{}/.config/solana/id.json", std::env::var("HOME").unwrap());
    let payer = read_keypair_file(&wallet_path).expect("não consegui ler a keypair");
    let authority = payer.pubkey();

    let (pda, _bump) =
        Pubkey::find_program_address(&[b"message", authority.as_ref()], &program_id);

    println!("Program ID : {program_id}");
    println!("Authority  : {authority}");
    println!("Message PDA: {pda}");
    println!();

    // 1) initialize (só se a conta ainda não existir)
    if rpc.get_account(&pda).is_err() {
        let ix = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(authority, true),
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(system_program_id, false),
            ],
            data: encode_string_ix(INIT_DISC, "Hello Speedrun"),
        };
        let bh = rpc.get_latest_blockhash().unwrap();
        let tx = Transaction::new_signed_with_payer(&[ix], Some(&authority), &[&payer], bh);
        let sig = rpc.send_and_confirm_transaction(&tx).unwrap();
        println!("initialize     -> tx: {sig}");
    } else {
        println!("initialize     -> PDA já existe, pulando");
    }

    // 2) update_message
    let new_text = "Leonardo - teste 1";
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(authority, true),
        ],
        data: encode_string_ix(UPDATE_DISC, new_text),
    };
    let bh = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&authority), &[&payer], bh);
    let sig = rpc.send_and_confirm_transaction(&tx).unwrap();
    println!("update_message -> tx: {sig}");

    // 3) leitura do estado on-chain (8 disc + 32 authority + 4 len + texto)
    let acc = rpc.get_account(&pda).unwrap();
    let len = u32::from_le_bytes(acc.data[40..44].try_into().unwrap()) as usize;
    let text = String::from_utf8_lossy(&acc.data[44..44 + len]);
    println!();
    println!("Estado on-chain text = {text:?}");
    println!();
    println!("Explorer programa: https://explorer.solana.com/address/{program_id}?cluster=devnet");
    println!("Explorer tx (update): https://explorer.solana.com/tx/{sig}?cluster=devnet");
}
