# {{project-name}}

Multi-module Java project with Spring Boot.

## Prerequisites

- [Java 25+](https://adoptium.net/)
- [Gradle 9.3+](https://gradle.org/install/) (only needed to generate the wrapper)

## Getting started

```bash
gradle wrapper
cp .env.example .env
./gradlew build
./gradlew :{{project-name}}-rest:bootRun
```

The server starts on `http://localhost:8080` by default.

## Project structure

```
├── platform/   BOM — version constraints
├── core/       Shared utilities
├── rest/       Spring Boot application
└── test/       Shared test infrastructure
```

## Endpoints

| Method | Path      | Description  |
|--------|-----------|--------------|
| GET    | `/health` | Health check |

## Adding a module

Create a directory with a `build.gradle` file. It will be automatically discovered by `settings.gradle` and named
`{{project-name}}-<dirname>`.

## Environment variables

| Variable                 | Default | Description           |
|--------------------------|---------|-----------------------|
| `PORT`                   | `8080`  | Server port           |
| `SPRING_PROFILES_ACTIVE` | —       | Active Spring profile |
