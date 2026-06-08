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
        .network_for(NetworkMode::Reserved)
        .rpc_url("http://localhost:50061")
        .build()
        .await;
    let pk = prover
        .setup(elf)
        .await?;
    println!("setup is complete.");
    let proof = prover
        .prove(&pk, stdin)
        .compressed()
        .await?;
    println!("proof is ready!");
    let _ = proof.save("./proof.bin");

    Ok(())
}
