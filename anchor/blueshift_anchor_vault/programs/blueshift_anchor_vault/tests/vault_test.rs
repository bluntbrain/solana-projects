use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use std::path::PathBuf;

// system program id
const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::from_str_const("11111111111111111111111111111111");

// program id from lib.rs - "22222222222222222222222222222222222222222222"
const PROGRAM_ID: Pubkey = Pubkey::from_str_const("22222222222222222222222222222222222222222222");

// anchor discriminators (first 8 bytes of sha256("global:<fn_name>"))
const DEPOSIT_DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
const WITHDRAW_DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

// helper to get program path
fn get_program_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy/blueshift_anchor_vault.so")
}

// helper to derive vault pda
fn get_vault_pda(signer: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault", signer.as_ref()], &PROGRAM_ID)
}

// helper to create deposit instruction
fn create_deposit_ix(signer: &Pubkey, vault: &Pubkey, amount: u64) -> Instruction {
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&DEPOSIT_DISCRIMINATOR);
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*signer, true),
            AccountMeta::new(*vault, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
        data,
    }
}

// helper to create withdraw instruction
fn create_withdraw_ix(signer: &Pubkey, vault: &Pubkey) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(*signer, true),
            AccountMeta::new(*vault, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
        data: WITHDRAW_DISCRIMINATOR.to_vec(),
    }
}

#[test]
fn test_deposit() {
    let mut svm = LiteSVM::new();

    // load program
    let program_path = get_program_path();
    svm.add_program_from_file(PROGRAM_ID, program_path.to_str().unwrap())
        .expect("failed to load program");

    // create and fund user
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    // derive vault pda
    let (vault_pda, _bump) = get_vault_pda(&user.pubkey());

    // deposit amount (1 sol)
    let deposit_amount: u64 = 1_000_000_000;

    // create deposit instruction
    let deposit_ix = create_deposit_ix(&user.pubkey(), &vault_pda, deposit_amount);

    // create and send transaction
    let tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "deposit failed: {:?}", result.err());

    // verify vault balance
    let vault_account = svm.get_account(&vault_pda).expect("vault account not found");
    assert_eq!(vault_account.lamports, deposit_amount, "vault balance mismatch");

    println!("deposit test passed - vault balance: {}", vault_account.lamports);
}

#[test]
fn test_withdraw() {
    let mut svm = LiteSVM::new();

    // load program
    let program_path = get_program_path();
    svm.add_program_from_file(PROGRAM_ID, program_path.to_str().unwrap())
        .expect("failed to load program");

    // create and fund user
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let (vault_pda, _bump) = get_vault_pda(&user.pubkey());
    let deposit_amount: u64 = 1_000_000_000;

    // first deposit
    let deposit_ix = create_deposit_ix(&user.pubkey(), &vault_pda, deposit_amount);
    let tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).expect("deposit failed");

    // get balance before withdraw
    let balance_before = svm.get_account(&user.pubkey()).unwrap().lamports;

    // now withdraw
    let withdraw_ix = create_withdraw_ix(&user.pubkey(), &vault_pda);
    let tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&user.pubkey()),
        &[&user],
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "withdraw failed: {:?}", result.err());

    // verify vault is empty
    let vault_account = svm.get_account(&vault_pda);
    assert!(
        vault_account.is_none() || vault_account.unwrap().lamports == 0,
        "vault should be empty after withdraw"
    );

    // verify user got funds back
    let balance_after = svm.get_account(&user.pubkey()).unwrap().lamports;
    assert!(balance_after > balance_before, "user should have more lamports after withdraw");

    println!("withdraw test passed");
}

#[test]
fn test_withdraw_empty_vault_fails() {
    let mut svm = LiteSVM::new();

    // load program
    let program_path = get_program_path();
    svm.add_program_from_file(PROGRAM_ID, program_path.to_str().unwrap())
        .expect("failed to load program");

    // create and fund user
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let (vault_pda, _bump) = get_vault_pda(&user.pubkey());

    // try to withdraw without depositing first
    let withdraw_ix = create_withdraw_ix(&user.pubkey(), &vault_pda);
    let tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&user.pubkey()),
        &[&user],
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(tx);

    // should fail
    assert!(result.is_err(), "withdraw from empty vault should fail");
    println!("withdraw from empty vault correctly failed");
}

#[test]
fn test_deposit_below_rent_exempt_fails() {
    let mut svm = LiteSVM::new();

    // load program
    let program_path = get_program_path();
    svm.add_program_from_file(PROGRAM_ID, program_path.to_str().unwrap())
        .expect("failed to load program");

    // create and fund user
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let (vault_pda, _bump) = get_vault_pda(&user.pubkey());

    // try to deposit tiny amount below rent-exempt minimum
    let tiny_amount: u64 = 100;
    let deposit_ix = create_deposit_ix(&user.pubkey(), &vault_pda, tiny_amount);
    let tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(tx);

    // should fail because amount is below rent-exempt minimum
    assert!(result.is_err(), "deposit below rent-exempt should fail");
    println!("deposit below rent-exempt correctly failed");
}

#[test]
fn test_multiple_users_separate_vaults() {
    let mut svm = LiteSVM::new();

    // load program
    let program_path = get_program_path();
    svm.add_program_from_file(PROGRAM_ID, program_path.to_str().unwrap())
        .expect("failed to load program");

    // create two users
    let user1 = Keypair::new();
    let user2 = Keypair::new();
    svm.airdrop(&user1.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&user2.pubkey(), 10_000_000_000).unwrap();

    let (vault1, _) = get_vault_pda(&user1.pubkey());
    let (vault2, _) = get_vault_pda(&user2.pubkey());

    // verify vaults are different
    assert_ne!(vault1, vault2, "each user should have unique vault pda");

    let deposit_amount: u64 = 1_000_000_000;

    // user1 deposits
    let deposit_ix = create_deposit_ix(&user1.pubkey(), &vault1, deposit_amount);
    let tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user1.pubkey()),
        &[&user1],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).expect("user1 deposit failed");

    // user2 deposits
    let deposit_ix = create_deposit_ix(&user2.pubkey(), &vault2, deposit_amount);
    let tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user2.pubkey()),
        &[&user2],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).expect("user2 deposit failed");

    // verify both vaults have correct balance
    let vault1_account = svm.get_account(&vault1).expect("vault1 not found");
    let vault2_account = svm.get_account(&vault2).expect("vault2 not found");

    assert_eq!(vault1_account.lamports, deposit_amount, "vault1 balance mismatch");
    assert_eq!(vault2_account.lamports, deposit_amount, "vault2 balance mismatch");

    println!("multiple users test passed - both vaults have {} lamports", deposit_amount);
}
