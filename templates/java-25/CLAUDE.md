# Project Guidelines

## First-Time Setup

When starting a new session in this project for the first time, run `/project-memory` to initialize the project memory
system. This sets up `docs/project_notes/` for tracking bugs, architectural decisions, key facts, and work history.

## Stack

- **Language**: Java 25
- **Framework**: Spring Boot 4.0
- **Build**: Gradle 9.3 (Groovy DSL, version catalog)
- **Formatting**: Spotless with Google Java Format

## Conventions

- Multi-module Gradle project with dynamic module discovery in `settings.gradle`
- Modules are prefixed with the project name (e.g., `myapp-core`, `myapp-rest`)
- Add a new module: create a directory with a `build.gradle` — it's auto-discovered
- **platform/**: BOM module — add version constraints here
- **core/**: Shared utilities and base classes
- **rest/**: Spring Boot application with REST endpoints
- **test/**: Shared test infrastructure (annotations, base classes)
- REST endpoints go in `rest/src/main/java/com/example/rest/controller/`
- Shared utilities go in `core/src/main/java/com/example/core/`
- All modules depend on the platform BOM via `implementation platform(project(":projectname-platform"))`
- Run formatting: `./gradlew spotlessApply`
- Run tests: `./gradlew test`
