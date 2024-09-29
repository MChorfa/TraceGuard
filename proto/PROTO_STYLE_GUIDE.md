# Protocol Buffer Style Guide

This document outlines the style guidelines for writing protocol buffer definitions in the TraceGuard project.

## Naming Conventions

- Use PascalCase for message names and enum names.
- Use snake_case for field names and enum value names.
- Prefix enum values with the enum name in all caps.

## Versioning

- Use a `v1`, `v2`, etc. subdirectory for each major version of the API.
- Include the major version in the package name, e.g., `package traceguard.v1`.

## Comments

- Use `//` for single-line comments.
- Use `/* */` for multi-line comments.
- Document all messages, fields, services, and methods.

## Organization

- Group related messages and enums together.
- Order fields logically, with required fields first.

## Best Practices

- Use singular names for repeated fields (e.g., `repeated string tag` instead of `repeated string tags`).
- Use enums for fields with a known, limited set of values.
- Use well-known types (e.g., `google.protobuf.Timestamp`) when appropriate.

Remember to run `buf lint` before committing changes to ensure adherence to these guidelines.