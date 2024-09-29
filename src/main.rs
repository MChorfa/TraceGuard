mod sbom;
mod provenance;
mod storage;
mod metadata;
mod observability;
mod compliance;
mod plugins;
mod lifecycle;
mod security;

use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    observability::telemetry::init_telemetry()?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: traceguard <command> [<args>]");
        process::exit(1);
    }

    match args[1].as_str() {
        "sbom" => handle_sbom_command(&args[2..]).await?,
        "provenance" => handle_provenance_command(&args[2..]).await?,
        "compliance" => handle_compliance_command(&args[2..]).await?,
        "lifecycle" => handle_lifecycle_command(&args[2..]).await?,
        _ => {
            println!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }

    Ok(())
}

async fn handle_sbom_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        println!("Usage: traceguard sbom <file_path>");
        process::exit(1);
    }

    let file_path = &args[0];
    let sbom = sbom::sbom_parser::parse_sbom(file_path)?;
    println!("SBOM parsed successfully:");
    println!("{:#?}", sbom);

    Ok(())
}

async fn handle_provenance_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        println!("Usage: traceguard provenance <artifact_id> <slsa_level>");
        process::exit(1);
    }

    let artifact_id = &args[0];
    let slsa_level: u8 = args[1].parse()?;

    let record = provenance::provenance_api::record_provenance(artifact_id, slsa_level, None).await?;
    println!("Provenance record created successfully:");
    println!("{:#?}", record);

    Ok(())
}

async fn handle_compliance_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        println!("Usage: traceguard compliance <system_name>");
        process::exit(1);
    }

    let system_name = &args[0];
    let components = vec![]; // Add sample components here
    let report = compliance::oscal::generate_oscal_report(system_name, components);

    let json = compliance::oscal::export_oscal_json(&report)?;
    println!("OSCAL report generated successfully:");
    println!("{}", json);

    Ok(())
}

async fn handle_lifecycle_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        println!("Usage: traceguard lifecycle <tenant_id> <artifact_type>");
        process::exit(1);
    }

    let tenant_id = uuid::Uuid::parse_str(&args[0])?;
    let artifact_type = &args[1];

    // This is a placeholder implementation. In a real-world scenario, you would:
    // 1. Fetch the lifecycle policy for the given tenant and artifact type
    // 2. Initialize the LifecycleManager with proper IcebergMetadata and BlobStorage
    // 3. Apply the lifecycle policy

    println!("Lifecycle management for tenant {} and artifact type {} initiated", tenant_id, artifact_type);

    Ok(())
}