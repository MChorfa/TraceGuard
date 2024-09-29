# **TraceGuard**

**TraceGuard** is a robust, enterprise-grade platform designed for secure, traceable, and compliant management of Software Bill of Materials (SBOMs), AI models, and data pipelines. Built with a focus on supply chain security and provenance, TraceGuard integrates with modern standards like **SLSA** and handles multiple SBOM formats, including **CycloneDX**, **SPDX**, and **SWID Tags**.

## **Key Features**
- **Multi-SBOM Support**: Parse and validate **CycloneDX**, **SPDX**, and **SWID** formats.
- **SLSA Provenance**: Integrates **SLSA levels** for software attestation and end-to-end traceability.
- **Provenance API**: Tracks the full lifecycle of software, data, and AI models.
- **Compliance**: Automatically generates **OSCAL** artifacts and enforces policies via **OPA**.
- **Multi-Tenancy**: Secure isolation of tenant data and resources.
- **Flexible Deployment**: Support for cloud, on-premises, and off-grid environments.
- **Observability**: Comprehensive monitoring and logging with OpenTelemetry.

## **Tech Stack**
- **Rust**: Core system logic and SBOM parsing.
- **Go**: Dagger CI/CD automation using Go SDK.
- **WebAssembly (Wasm)**: For efficient cross-platform execution.
- **Kubernetes**: For orchestration and scaling.
- **Apache Iceberg**: For efficient metadata management.
- **Sigstore**: Cryptographic signing and verification.
- **OpenTelemetry**: For observability and monitoring.

## **Getting Started**

1. Clone the repository:
   ```bash
   git clone https://github.com/MChorfa/TraceGuard.git
   cd TraceGuard
   ```

2. Run the initialization script to set up the project structure:
   ```bash
   ./scripts/traceguard_init.sh
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. Run tests:
   ```bash
   cargo test
   ```

5. For deployment options, refer to the [Deployment Guide](./docs/DEPLOYMENT.md).

## **Documentation**
Full documentation can be found in the [docs](./docs/README.md) folder, including architecture diagrams, configuration files, and deployment instructions.

## **Contribution**
We welcome contributions! Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

## **License**

This project is licensed under the **Apache License 2.0**. See the [LICENSE](./LICENSE) file for details.
