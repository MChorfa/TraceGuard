# Variables
PROJECT_NAME = traceguard
DOCKER_IMAGE = traceguard:latest
DOCKER_COMPOSE_FILE = docker-compose.yml

# Build Rust code
build:
	cargo build --release

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Lint the code
lint:
	cargo clippy

# Build the Docker image
docker-build:
	docker build -t $(DOCKER_IMAGE) .

# Run the app with Docker Compose
up:
	docker-compose -f $(DOCKER_COMPOSE_FILE) up

# Stop the Docker Compose services
down:
	docker-compose -f $(DOCKER_COMPOSE_FILE) down

# Clean up
clean:
	cargo clean

.PHONY: buf-lint buf-generate buf-breaking

buf-lint:
	cd proto && buf lint

buf-generate:
	cd proto && buf generate

buf-breaking:
	cd proto && buf breaking --against '.git#branch=main'
