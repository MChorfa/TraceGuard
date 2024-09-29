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

## Project Structure

## Getting Started

1. Clone the repository
2. Set up the database:
   ```
   psql -d your_database_name -f migrations/001_create_sboms_table.sql
   psql -d your_database_name -f migrations/002_create_provenance_records_table.sql
   ```
3. Set the `DATABASE_URL` environment variable
4. Run the backend:
   ```
   cargo run
   ```
5. Run the web UI:
   ```
   cd web_ui && npm start
   ```

## Development

- Run tests: `cargo test`
- Format code: `cargo fmt`
- Lint code: `cargo clippy`

## Deployment

The project uses GitHub Actions for continuous integration and deployment. See `.github/workflows/cd.yml` for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.