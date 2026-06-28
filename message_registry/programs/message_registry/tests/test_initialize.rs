use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize_and_update() {
    let program_id = message_registry::id();
    let payer = Keypair::new();
    let message_account = Pubkey::find_program_address(
        &[
            message_registry::constants::MESSAGE_SEED,
            payer.pubkey().as_ref(),
        ],
        &program_id,
    )
    .0;
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/message_registry.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // initialize
    let instruction = Instruction::new_with_bytes(
        program_id,
        &message_registry::instruction::Initialize {
            text: "Hello Speedrun".to_string(),
        }
        .data(),
        message_registry::accounts::Initialize {
            authority: payer.pubkey(),
            message_account,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let acc = svm.get_account(&message_account).unwrap();
    let mut data: &[u8] = &acc.data;
    let state = message_registry::state::MessageAccount::try_deserialize(&mut data).unwrap();
    assert_eq!(state.text, "Hello Speedrun");
    assert_eq!(state.authority, payer.pubkey());

    // update_message
    let instruction = Instruction::new_with_bytes(
        program_id,
        &message_registry::instruction::UpdateMessage {
            new_text: "Updated message".to_string(),
        }
        .data(),
        message_registry::accounts::UpdateMessage {
            message_account,
            authority: payer.pubkey(),
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let acc = svm.get_account(&message_account).unwrap();
    let mut data: &[u8] = &acc.data;
    let state = message_registry::state::MessageAccount::try_deserialize(&mut data).unwrap();
    assert_eq!(state.text, "Updated message");
    assert_eq!(state.authority, payer.pubkey());
}
