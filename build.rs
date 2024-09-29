use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=proto");
    
    let output = Command::new("buf")
        .args(&["generate"])
        .current_dir("proto")  // Set the working directory to the proto folder
        .output()
        .expect("Failed to execute buf generate");

    if !output.status.success() {
        panic!("buf generate failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}