# TraceGuard

TraceGuard is a robust, enterprise-grade platform designed for secure, traceable, and compliant management of Software Bill of Materials (SBOMs), AI models, and data pipelines.

## DISCLAIMER ❗❗❗

<span style="color:red;">❗ This project is a fictional example created for educational purposes. It does not represent a real company or product. ❗</span>

## Features

- Multi-SBOM Support (CycloneDX, SPDX, SWID)
- SLSA Provenance
- Provenance API
- Compliance (OSCAL, OPA)
- Multi-Tenancy
- Flexible Deployment
- Observability (OpenTelemetry)
- Web UI for SBOM and Provenance Management
- CLI for CI/CD pipelines
- gRPC Service for efficient communication between the backend and frontend

## Getting Started

### Prerequisites

- Rust 1.55 or later
- Node.js 14 or later
- PostgreSQL 13 or later
- Go 1.16 or later (for CLI)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-org/traceguard.git
   cd traceguard
   ```

2. Set up the backend:
   ```
   cargo build
   ```

3. Set up the frontend:
   ```
   cd web_ui
   npm install
   ```

4. Set up the database:
   ```
   createdb traceguard
   sqlx database create
   sqlx migrate run
   ```

5. Start the development server:
   ```
   cargo run
   ```

6. In a separate terminal, start the frontend development server:
   ```
   cd web_ui
   npm start
   ```

## Provenance Management

TraceGuard now supports SLSA (Supply-chain Levels for Software Artifacts) provenance management. You can create, verify, and manage provenance records using the API, CLI, or Web UI.

For more information on how to use the provenance features, please refer to the API documentation and the CLI usage guide.

## Usage

Visit `http://localhost:3000` in your web browser to access the TraceGuard web interface. You can now upload SBOMs, view provenance information, and generate compliance reports.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
