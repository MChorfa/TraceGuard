# TraceGuard API Documentation

This document outlines the main API endpoints provided by TraceGuard for managing SBOMs, provenance, and compliance information in a cloud-agnostic, multi-tenant environment.

## Authentication

TraceGuard uses OpenID Connect (OIDC) for authentication. After successful authentication, you'll receive a JWT token. Use this token in the Authorization header for all API requests:

## SBOM Management

### Upload SBOM

json
{
"name": "example-sbom",
"version": "1.0",
"format": "CycloneDX",
"content": "{ ... SBOM content ... }"
}

### List SBOMs

GET /api/sboms

Returns a list of all SBOMs in the system.

### Get SBOM

GET /api/sboms/{sbom_id}

Retrieve a specific SBOM by its ID.

## Provenance Management

### Create Provenance Record

POST /api/provenance

Create a new provenance record.

Request Body:

json
{
"artifact_id": "example-artifact",
"slsa_level": 2,
"metadata": { ... additional metadata ... }
}

### Verify Provenance

GET /api/provenance/verify/{artifact_id}

Verify the provenance of a specific artifact.

## Compliance Reporting

### Generate Compliance Report

POST /api/compliance/report

Generate a compliance report for a specific SBOM.

Request Body:
json
{
"tenant_id": "example-tenant",
"sbom_id": "example-sbom-id",
"framework": "NIST-800-53"
}


### Get Compliance Report

GET /api/compliance/report/{report_id}

Retrieve a specific compliance report by its ID.


## CLI Usage


TraceGuard provides a command-line interface for common operations.


### Upload SBOM
traceguard-cli upload-sbom <file_path>

### Create Provenance Record
traceguard-cli create-provenance <artifact_id> <slsa_level>

### Generate Compliance Report
traceguard-cli generate-compliance-report <tenant_id> <sbom_id> <framework>


### SBOM Operations

Parse an SBOM file:

## SBOM Endpoints

### List SBOMs

GET /api/sboms

Returns a list of all SBOMs in the system.

### Create SBOM

POST /api/sboms

Create a new SBOM.

Request Body:

### Get SBOM Relationships

GET /api/sboms/relationships

Retrieves the relationships between SBOMs.

Response:

## Provenance Endpoints

### List Provenance Records

GET /api/provenance

Retrieves a list of all provenance records.

Response Body:

### Create Provenance Record

POST /api/provenance

Create a new provenance record.

Request Body:

## Compliance Endpoints

### Generate Compliance Report

POST /api/compliance/report

Generates a compliance report for a specific tenant.

Request Body:

## Authentication

TraceGuard supports OpenID Connect (OIDC) authentication with multiple providers:

- Azure
- GitHub
- Google
- GitLab

### OIDC Login

GET /auth/:provider/login

Initiates the OIDC authentication flow for the specified provider.

### OIDC Callback

GET /auth/:provider/callback

Handles the OIDC authentication callback and completes the authentication process.