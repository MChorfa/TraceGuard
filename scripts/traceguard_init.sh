#!/bin/bash

# Create core directories
mkdir -p src/sbom src/provenance src/data src/models src/compliance src/security
mkdir -p dagger cli tests docs config

# Create core Rust source files
touch src/sbom/sbom_parser.rs
touch src/sbom/slsa_provenance.rs
touch src/provenance/provenance_api.rs
touch src/data/data_provenance.rs
touch src/models/model_registry.rs
touch src/compliance/oscal_integration.rs
touch src/security/encryption.rs
touch src/main.rs

# Create Go-based Dagger pipeline and CLI
touch dagger/pipeline.go
touch cli/traceguard_cli.go

# Create test directory and sample file
touch tests/integration_tests.rs

# Documentation and config files
touch docs/README.md
touch docs/architecture_diagram.puml
touch config/dagger_pipeline.go
touch config/porter_bundle.yml
touch config/sbom_policy.rego

# Create Cargo.toml
touch Cargo.toml

# Create Go mod file
touch go.mod

# Print directory structure
echo "Project structure initialized:"
tree .

# Initialize Go module
go mod init github.com/MChorfa/TraceGuard

# Install Go dependencies
go get dagger.io/dagger
go get github.com/spf13/cobra

echo "Go dependencies installed"

# Initialize Rust project
cargo init --bin

echo "Rust project initialized"

# Print final message
echo "TraceGuard project initialized successfully!"
