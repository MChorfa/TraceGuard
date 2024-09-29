# TraceGuard API Documentation

This document outlines the main API endpoints provided by TraceGuard for managing SBOMs, provenance, and compliance information.

## Authentication

All API endpoints require authentication. Use the following header in your requests:

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

## Provenance Endpoints

### List Provenance Records

GET /api/provenance

Returns a list of all provenance records in the system.

### Create Provenance Record

POST /api/provenance

Create a new provenance record.

Request Body:

## Compliance Endpoints

### Generate Compliance Report

POST /api/compliance/report

Generates a compliance report for a specific tenant.

Request Body: