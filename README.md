# **TraceGuard**

**TraceGuard** is a robust platform designed to ensure secure, traceable, and compliant management of SBOMs (Software Bill of Materials), AI models, and data pipelines. Built with a focus on supply chain security and provenance, TraceGuard integrates with modern standards like **SLSA** and handles multiple SBOM formats, including **CycloneDX**, **SPDX**, and **SWID Tags**. With a comprehensive compliance framework, the platform helps organizations meet security and regulatory standards.

## **Key Features**
- **Multi-SBOM Support**: Parse and validate **CycloneDX**, **SPDX**, and **SWID** formats.
- **SLSA Provenance**: Integrates **SLSA levels** for software attestation and end-to-end traceability.
- **Provenance API**: Tracks the full lifecycle of software, data, and AI models.
- **Compliance**: Automatically generates **OSCAL** artifacts and enforces policies via **OPA**.

## **Tech Stack**
- **Rust**: Core system logic and SBOM parsing.
- **Go**: Dagger CI/CD automation using Go SDK.
- **WebAssembly (Wasm)**: For efficient cross-platform execution.
- **Sigstore**: Cryptographic signing and verification.
- **MLflow/ZenML**: AI model lifecycle management. (API integration)
  
## **Getting Started**

1. Clone the repository:
   ```bash
   git clone https://github.com/MChorfa/TraceGuard.git
   cd traceguard
   ```

2. Run the initialization script to set up the project structure:
   ```bash
   ./init_project.sh
   ```

3. Follow the configuration steps in the [Documentation](./docs/README.md) for setup instructions.

## **Documentation**
Full documentation can be found in the [docs](./docs/README.md) folder, including architecture diagrams, configuration files, and deployment instructions.

## **Contribution**
We welcome contributions! Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

## **License**

This project is licensed under the **Apache License 2.0**. See the [LICENSE](./LICENSE) file for details.
