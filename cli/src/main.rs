use clap::{App, Arg, SubCommand};
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("TraceGuard CLI")
        .version("1.0")
        .author("Your Name")
        .about("CLI for TraceGuard")
        .subcommand(SubCommand::with_name("upload-sbom")
            .about("Upload an SBOM")
            .arg(Arg::with_name("file")
                .help("The SBOM file to upload")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("create-provenance")
            .about("Create a provenance record")
            .arg(Arg::with_name("artifact-id")
                .help("The artifact ID")
                .required(true)
                .index(1))
            .arg(Arg::with_name("slsa-level")
                .help("The SLSA level")
                .required(true)
                .index(2)))
        .subcommand(SubCommand::with_name("generate-compliance-report")
            .about("Generate a compliance report")
            .arg(Arg::with_name("tenant-id")
                .help("The tenant ID")
                .required(true)
                .index(1))
            .arg(Arg::with_name("sbom-id")
                .help("The SBOM ID")
                .required(true)
                .index(2))
            .arg(Arg::with_name("framework")
                .help("The compliance framework")
                .required(true)
                .index(3)))
        .subcommand(SubCommand::with_name("get-provenance")
            .about("Get a provenance record")
            .arg(Arg::with_name("id")
                .help("The provenance record ID")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("verify-provenance")
            .about("Verify a provenance record")
            .arg(Arg::with_name("id")
                .help("The provenance record ID")
                .required(true)
                .index(1)))
        .get_matches();

    let client = reqwest::Client::new();

    match matches.subcommand() {
        ("upload-sbom", Some(sub_m)) => {
            let file_path = sub_m.value_of("file").unwrap();
            let content = std::fs::read_to_string(file_path)?;
            
            let response = client.post("http://localhost:8080/api/sboms")
                .json(&serde_json::json!({
                    "name": file_path,
                    "version": "1.0",
                    "format": "CycloneDX",
                    "content": content
                }))
                .send()
                .await?;

            println!("SBOM uploaded successfully. Response: {:?}", response);
        },
        ("create-provenance", Some(sub_m)) => {
            let artifact_id = sub_m.value_of("artifact-id").unwrap();
            let slsa_level = sub_m.value_of("slsa-level").unwrap().parse::<i32>()?;

            let response = client.post("http://localhost:8080/api/provenance")
                .json(&serde_json::json!({
                    "artifact_id": artifact_id,
                    "slsa_level": slsa_level,
                }))
                .send()
                .await?;

            println!("Provenance record created successfully. Response: {:?}", response);
        },
        ("generate-compliance-report", Some(sub_m)) => {
            let tenant_id = sub_m.value_of("tenant-id").unwrap();
            let sbom_id = sub_m.value_of("sbom-id").unwrap();
            let framework = sub_m.value_of("framework").unwrap();

            let response = client.post("http://localhost:8080/api/compliance/report")
                .json(&serde_json::json!({
                    "tenant_id": tenant_id,
                    "sbom_id": sbom_id,
                    "framework": framework,
                }))
                .send()
                .await?;

            println!("Compliance report generated successfully. Response: {:?}", response);
        },
        ("get-provenance", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            let response = client.get(&format!("http://localhost:8080/api/provenance/{}", id))
                .send()
                .await?;
            println!("Provenance record: {:?}", response.json::<serde_json::Value>().await?);
        },
        ("verify-provenance", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            let response = client.get(&format!("http://localhost:8080/api/provenance/{}/verify", id))
                .send()
                .await?;
            println!("Provenance verification result: {:?}", response.json::<bool>().await?);
        },
        _ => println!("Invalid command. Use --help for usage information."),
    }

    Ok(())
}