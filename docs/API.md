# TraceGuard API Documentation

This document outlines the main API endpoints provided by TraceGuard for managing SBOMs, provenance, and compliance information in a cloud-agnostic, multi-tenant environment.

## Authentication

TraceGuard uses OpenID Connect (OIDC) for authentication. After successful authentication, you'll receive a JWT token. Use this token in the Authorization header for all API requests:

## SBOM Management

### Upload SBOM

## CLI Usage

TraceGuard provides a command-line interface for common operations.

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