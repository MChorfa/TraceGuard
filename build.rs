use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/traceguard.proto")?;

    // Run buf generate command
    let output = Command::new("buf")
        .args(&["generate", "proto"])
        .output()?;

    if !output.status.success() {
        return Err(format!("buf generate failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}