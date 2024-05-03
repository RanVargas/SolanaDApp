use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Signature, Signer};
use solana_sdk::transaction::Transaction;

pub struct SolanaClient {
    rpc_client: RpcClient,
}

impl SolanaClient {
    pub fn new(url: &str) -> Self {
        SolanaClient {
            rpc_client: RpcClient::new(String::from(url)),
        }
    }

    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, Box<dyn std::error::Error>> {
        self.rpc_client.get_balance(pubkey).await.map_err(Into::into)
    }

    pub fn get_address(&self, pubkey: &Pubkey) -> String {
        pubkey.to_string()
    }

    pub async fn get_signature(&self, pubkey: &Pubkey, transaction: &Transaction) -> Result<Signature, Box<dyn std::error::Error>> {
        let recent_blockhash = self.rpc_client.get_recent_blockhash().await?.0;
        let mut transaction = transaction.clone();
        transaction.try_sign(&[pubkey], recent_blockhash)?;

        self.rpc_client.send_and_confirm_transaction(&transaction).await.map_err(Into::into)
    }
}