use anyhow::anyhow;
use clap::Parser;
use transmission_rpc::{self as tx, types::SessionSetArgs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv();

    let run_config = RunConfig::parse();

    let mut client = tx::TransClient::new(run_config.tx_rpc.as_str().try_into()?);

    println!(
        "Setting peer port to {} on {}",
        run_config.peer_port, run_config.tx_rpc
    );

    client
        .session_set(SessionSetArgs {
            peer_port: Some(run_config.peer_port as _),
            ..Default::default()
        })
        .await
        .map_err(|e| anyhow!("{e}"))?;

    // `transmission-remote` prints the following:
    // localhost:9091/transmission/rpc/ responded: success
    println!("Success.");

    Ok(())
}

#[derive(clap::Parser)]
struct RunConfig {
    /// Transmission RPC URL
    #[arg(long, env, default_value = "http://127.0.0.1:9091/transmission/rpc/")]
    tx_rpc: String,
    /// Peer port to set
    #[arg(long)]
    peer_port: u32,
}
