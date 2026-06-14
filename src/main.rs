use tokio;
use sp1_sdk::{Elf, ProveRequest, Prover, ProverClient, SP1Stdin,
    network::NetworkMode,
    SP1ProofWithPublicValues,
    ProvingKey
};

use std::fs;
// use md5;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    // just_execute().await?;
    // prove_on_cluster()?;
    prove_on_cuda().await?;
    // verify().await?;

    Ok(())
}
pub async fn just_execute() -> anyhow::Result<()> {
    let elf: Elf = fs::read("blobs/subblock.bin")?.into();
    // let stdin_blob = fs::read("blobs/1.bin")?;
    let stdin: SP1Stdin = bincode::deserialize(&fs::read("blobs/0.bin")?)?;
    let prover = ProverClient::from_env().await;
    let _ = prover
        .setup(elf.clone())
        .await?;
    println!("prover is loaded.");
    let (_public_values, report) = prover
        .execute(elf, stdin)
        .await?;
    println!("execution is done.");
    println!("{:#?} ", report);
    println!("cycles: {}", report.total_instruction_count());
    Ok(())
}

pub async fn prove_on_cuda() -> anyhow::Result<()> {
    let elf: Elf = fs::read("blobs/subblock.bin")?.into();
    let stdin: SP1Stdin = bincode::deserialize(&fs::read("blobs/0.bin")?)?;
    let prover = ProverClient::from_env().await;
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

pub async fn prove_on_cluster() -> anyhow::Result<()> {
    let elf: Elf = fs::read("blobs/subblock.bin")?.into();
    let stdin: SP1Stdin = bincode::deserialize(&fs::read("blobs/0.bin")?)?;
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
    let proof = SP1ProofWithPublicValues::load("proof1-4090.bin")?;
    println!("verifying...");
    cpu_client.verify(&proof, pk.verifying_key(), None)?;
    println!("WOW, verified!");
    Ok(())
}
