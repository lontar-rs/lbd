# Contributing to LBD

Thank you for helping build the Lontar Balinese Dictionary.

## Ground rules
- Be respectful of the philological and linguistic context (Balinese language, unda-usuk register system).
- All changes must be non-destructive: append-only event logs, no history rewrites.
- Follow the Apache-2.0 license (see LICENSE).

## Project status
- Phase 0/1 scaffold: Rust (Axum), PostgreSQL, Meilisearch placeholders, pre-commit hooks.

## Development setup
1. Install Rust (stable), PostgreSQL, Meilisearch.
2. Copy env: `cp .env.example .env` and set `DATABASE_URL`, `MEILISEARCH_URL`, `MEILISEARCH_KEY`.
3. Create DB and run migrations: `sqlx migrate run`.
4. Seed sample data (optional): `psql -d lbd -f scripts/seed_data.sql`.
5. Run server: `cargo run`.

## Tooling
- Pre-commit: `pre-commit install` (already configured). Run all hooks: `pre-commit run --all-files`.
- Formatting: `cargo fmt -- --check` (also via pre-commit).
- Lints: `cargo clippy -- -D warnings` (also via pre-commit).
- Tests: `cargo test`.

## Branch/commit
- Use descriptive branches.
- Commits should be scoped and message should summarize the change.
- Keep README/ARCHITECTURE/TODO updates in sync with code changes when relevant.

## Adding database changes
- Add migrations under `migrations/` using `sqlx migrate add <name>`.
- Keep schema aligned with ARCHITECTURE.md.

## Reporting issues
- Use the GitHub issue template; include repro steps, logs, and environment info.

## Security
- See SECURITY.md for vulnerability reporting.
