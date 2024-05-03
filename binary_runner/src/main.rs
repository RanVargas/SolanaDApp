use solana_client::SolanaClient;
use solana_sdk::pubkey::Pubkey;
use walletconnect_client::{WalletConnect, Metadata};


async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = Metadata {
        name: "CLI DApp".into(),
        description: "CLI-based Solana DApp".into(),
        url: "localhost".into(),
        icons: vec!["https://nordicperspective.com/wp-content/uploads/2022/09/fenrir-aleksi-briclot.jpg".into()],
    };

    let project_id = "5f7d164c2cd0149e268c40021fe004f0";

    let client = WalletConnect::new(project_id, metadata);

    let session = client.create_session().await?;
    let uri = session.uri();
    println!("Scan this QR code with your wallet app to connect: {}", uri);

    let solana_client = SolanaClient::new("https://api.devnet.solana.com");

    let pubke0y = Pubkey::new_unique();

    let balance = solana_client.get_balance(&pubkey).await?;
    println!("Balance: {}", balance);

    let address = solana_client.get_address(&pubkey);
    println!("Address: {}", address);


    Ok(())
}
