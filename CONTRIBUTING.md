# Contributing to TraceGuard

Thank you for your interest in contributing to **TraceGuard**! We welcome contributions from the community to make the project even better. Please read through the following guidelines to get started.

## How to Contribute

1. **Fork the Repository**  
   - Fork the project to your own GitHub account and clone it locally.
   - Create a new branch for your feature, bugfix, or documentation update.

   ```bash
   git clone https://github.com/MChorfa/TraceGuard.git
   git checkout -b feature/your-feature
   ```

2. **Submit a Pull Request (PR)**
   - Make sure your changes are tested and linted before submitting.
   - Once you're ready, submit a pull request to the main repository.

3. **Code of Conduct**
   - We follow the [Contributor Covenant](https://www.contributor-covenant.org/) to ensure a welcoming and inclusive environment.

4. **Issues and Bug Reports**
   - If you find a bug or have a feature request, open an issue in the [Issues](https://github.com/MChorfa/TraceGuard/issues) tab.

## Coding Guidelines

- **Rust Code Style**: Follow the official [Rust style guide](https://doc.rust-lang.org/).
- **Go SDK**: For contributions to the Dagger pipeline, use Go conventions.
- **Testing**: All contributions must include tests to ensure functionality and prevent regressions.

## Development Workflow

1. **Run Tests**  
   - Before submitting a PR, make sure all tests pass:
     ```bash
     cargo test
     ```
   - For Go-related changes:
     ```bash
     go test ./...
     ```

2. **Linting**  
   - Run linting tools to ensure code quality:
     ```bash
     cargo fmt -- --check
     ```
     For Go:
     ```bash
     gofmt -l .
     ```

## Feature Requests

If you have a suggestion for a new feature, open an issue with the following details:
- **Feature Description**: A clear and concise explanation of the feature.
- **Use Case**: Why the feature is necessary.
- **Implementation Details**: (Optional) Ideas on how the feature can be implemented.

## Contact
For any questions or further discussion, feel free to open an issue or reach out to the project maintainers.

