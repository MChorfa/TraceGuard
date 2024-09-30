# TraceGuard Development Guide

This guide provides instructions for setting up the development environment and contributing to the TraceGuard project.

## Prerequisites

- Rust 1.55 or later
- PostgreSQL 13 or later
- Node.js 14 or later (for web UI)
- Go 1.16 or later (for CLI)

## Setting Up the Development Environment

1. Clone the repository:
   ```
   git clone https://github.com/your-org/traceguard.git
   cd traceguard
   ```

2. Set up the database:
   ```
   createdb traceguard
   sqlx database create
   sqlx migrate run
   ```

3. Set up environment variables:
   ```
   cp .env.example .env
   # Edit .env with your local configuration
   ```

4. Build the project:
   ```
   cargo build
   ```

5. Run the tests:
   ```
   cargo test
   ```

6. Start the development server:
   ```
   cargo run
   ```

## Project Structure

- `src/`: Rust source code
  - `api/`: API endpoints
  - `models/`: Database models
  - `security/`: Security-related code
  - `storage/`: Storage implementations
- `migrations/`: Database migrations
- `proto/`: Protocol buffer definitions
- `web_ui/`: Web UI source code
- `cli/`: Command-line interface source code
- `tests/`: Integration tests
- `docs/`: Project documentation

## Contributing

1. Create a new branch for your feature or bug fix.
2. Make your changes and add tests if applicable.
3. Ensure all tests pass by running `cargo test`.
4. Update documentation if necessary.
5. Submit a pull request with a clear description of your changes.

## Code Style

We follow the Rust style guide. Please run `rustfmt` on your code before submitting a pull request:

## Database Migrations

To run database migrations:
