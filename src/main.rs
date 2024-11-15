use std::str::FromStr;

use solana_sdk::{
    commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction, instruction::{Instruction, InstructionError}, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signature}, signer::Signer, system_program, transaction::{Transaction, TransactionError}
};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};

fn main() {
    let rpc_client = RpcClient::new_with_commitment(
        "http://api.mainnet-beta.solana.com",
        CommitmentConfig::processed(),
    );

    let pool_state_address = Pubkey::from_str("3SsRkGV9SuaT4NfgkB5jAQfoGXgFMCkNFEZC14uqoGJk").unwrap();

    let account_data = rpc_client.get_account_data(&pool_state_address).unwrap();
    let pool_state = raydium::accounts::PoolStateAccount::deserialize(&account_data).unwrap();
    println!("{:?}", pool_state);
}
