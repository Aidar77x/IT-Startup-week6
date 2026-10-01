use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::system_program,
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize_profile() {
    let program_id = week6_profile::id();
    let payer = Keypair::new();

    let profile = Pubkey::find_program_address(
        &[
            week6_profile::constants::PROFILE_SEED,
            payer.pubkey().as_ref(),
        ],
        &program_id,
    )
    .0;

    let mut svm = LiteSVM::new();

    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/week6_profile.so"
    ));

    svm.add_program(program_id, bytes).unwrap();

    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let instruction = anchor_lang::solana_program::instruction::Instruction::new_with_bytes(
        program_id,
        &week6_profile::instruction::Initialize {
            username: "roma".to_string(),
            bio: "Solana Web3 developer".to_string(),
        }
        .data(),
        week6_profile::accounts::Initialize {
            payer: payer.pubkey(),
            profile,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();

    let msg = Message::new_with_blockhash(
        &[instruction],
        Some(&payer.pubkey()),
        &blockhash,
    );

    let tx = VersionedTransaction::try_new(
        VersionedMessage::Legacy(msg),
        &[&payer],
    )
    .unwrap();

    let result = svm.send_transaction(tx);

    assert!(result.is_ok());

    let profile_account = svm.get_account(&profile).unwrap();

    let mut data: &[u8] = &profile_account.data;

    let profile_state =
        week6_profile::state::Profile::try_deserialize(&mut data).unwrap();

    assert_eq!(profile_state.authority, payer.pubkey());
    assert_eq!(profile_state.username, "roma");
    assert_eq!(profile_state.bio, "Solana Web3 developer");
}
