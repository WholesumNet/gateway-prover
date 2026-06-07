use tokio;
use sp1_sdk::{Elf, ProveRequest, Prover, ProverClient, SP1Stdin,
    network::NetworkMode,
};

use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let elf: Elf = fs::read("elf/subblock.bin")?.into();
    let stdin = SP1Stdin::from(&fs::read("elf/0.bin")?);
    let prover = ProverClient::builder()
        .network()
        .rpc_url("http://gateway.internal:50061")
        .hosted()
        .build()
        .await;
    let pk = prover
        .setup(elf)
        .await?;
    let proof = prover
        .prove(&pk, stdin)
        .compressed()
        .await?;

    Ok(())
}
