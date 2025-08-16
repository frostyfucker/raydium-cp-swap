#[tokio::test]
async fn test_integer_overflow_in_swap() {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::InstructionError,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
        transaction::TransactionError,
    };
    use raydium_cp_swap::{instruction as raydium_instruction, error::ErrorCode};

    let rpc_client = RpcClient::new("http://localhost:8899".to_string());
    let payer = Keypair::new();

    let sig = rpc_client.request_airdrop(&payer.pubkey(), 2_000_000_000).unwrap();
    rpc_client.confirm_transaction_with_commitment(&sig, CommitmentConfig::confirmed()).unwrap();

    let program_id = Pubkey::new_from_string("<YOUR_RAYDIUM_CP_SWAP_PROGRAM_ID>").unwrap();
    let amm_id = Pubkey::new_from_string("<YOUR_AMM_ACCOUNT_PUBKEY>").unwrap();
    let payer_token_a_account = Pubkey::new_from_string("<YOUR_PAYER_TOKEN_A_ACCOUNT_PUBKEY>").unwrap();
    let pool_token_a_vault = Pubkey::new_from_string("<YOUR_POOL_TOKEN_A_VAULT_PUBKEY>").unwrap();
    let pool_token_b_vault = Pubkey::new_from_string("<YOUR_POOL_TOKEN_B_VAULT_PUBKEY>").unwrap();
    let payer_token_b_account = Pubkey::new_from_string("<YOUR_PAYER_TOKEN_B_ACCOUNT_PUBKEY>").unwrap();
    let pool_lp_mint = Pubkey::new_from_string("<YOUR_POOL_LP_MINT_PUBKEY>").unwrap();
    let pool_lp_vault = Pubkey::new_from_string("<YOUR_POOL_LP_VAULT_PUBKEY>").unwrap();
    let pool_state_account = Pubkey::new_from_string("<YOUR_POOL_STATE_ACCOUNT_PUBKEY>").unwrap();

    let in_amount: u64 = u64::MAX - 100;
    let minimum_out_amount: u64 = 0;

    let swap_ix = raydium_instruction::swap(
        program_id,
        amm_id,
        payer_token_a_account,
        pool_token_a_vault,
        pool_token_b_vault,
        payer_token_b_account,
        pool_lp_mint,
        pool_lp_vault,
        pool_state_account,
        in_amount,
        minimum_out_amount,
    ).unwrap();

    let mut transaction = Transaction::new_with_payer(&[swap_ix], Some(&payer.pubkey()));
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    transaction.sign(&[&payer], recent_blockhash);

    let simulation_result = rpc_client.simulate_transaction(&transaction).unwrap();

    match simulation_result.value.err {
        Some(TransactionError::InstructionError(_, InstructionError::Custom(code))) => {
            assert_eq!(code, ErrorCode::MathOverflow as u32);
            println!("✅ SUCCESS: MathOverflow triggered as expected.");
        }
        _ => panic!("❌ FAIL: Expected MathOverflow error not triggered."),
    }
}

