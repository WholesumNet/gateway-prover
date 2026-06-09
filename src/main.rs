use tokio;
use sp1_sdk::{Elf, ProveRequest, Prover, ProverClient, SP1Stdin,
    network::NetworkMode,
    SP1ProofWithPublicValues,
    ProvingKey
};

use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let elf: Elf = fs::read("blobs/subblock.bin")?.into();
    let stdin = SP1Stdin::from(&fs::read("blobs/0.bin")?);
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
    // verify().await?;

    Ok(())
}

pub async fn verify() -> anyhow::Result<()> {
    println!("Initializing SP1.");
    let cpu_client = ProverClient::builder()
        .cpu()
        .build()
        .await;
    let elf: Elf = fs::read("blobs/subblock.bin").unwrap().into();
    let pk = cpu_client
        .setup(elf)
        .await?;        
    let proof = SP1ProofWithPublicValues::load("blobs/proof.bin")?;
    println!("verifying...");
    cpu_client.verify(&proof, pk.verifying_key(), None)?;
    println!("WOW, verified!");
    Ok(())
}
