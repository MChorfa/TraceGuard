# TraceGuard

TraceGuard is a robust, enterprise-grade platform designed for secure, traceable, and compliant management of Software Bill of Materials (SBOMs), AI models, and data pipelines.

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
- OpenID Connect (OIDC) Authentication

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

4. Set up the CLI:
   ```
   cd cli
   go build
   ```

5. Set up the database:
   ```
   psql -c "CREATE DATABASE traceguard"
   sqlx database create
   sqlx migrate run
   ```

6. Configure OIDC providers:
   Create a `.env` file in the root directory and add your OIDC provider credentials:
   ```
   AZURE_CLIENT_ID=your_azure_client_id
   AZURE_CLIENT_SECRET=your_azure_client_secret
   GITHUB_CLIENT_ID=your_github_client_id
   GITHUB_CLIENT_SECRET=your_github_client_secret
   GOOGLE_CLIENT_ID=your_google_client_id
   GOOGLE_CLIENT_SECRET=your_google_client_secret
   GITLAB_CLIENT_ID=your_gitlab_client_id
   GITLAB_CLIENT_SECRET=your_gitlab_client_secret
   ```

7. Start the development servers:
   ```
   # In one terminal
   cargo run

   # In another terminal
   cd web_ui
   npm start
   ```

## Usage

### CLI

TraceGuard CLI provides various commands for managing SBOMs, provenance, and compliance:


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.