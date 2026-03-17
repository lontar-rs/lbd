# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

LBD (Lontar Balinese Dictionary) is an OED-model historical dictionary of the Balinese language — trilingual (Balinese, Indonesian, English), corpus-backed, with full register (unda-usuk) stratification and etymological depth. Currently in **Phase 0 (Design)** — no application code exists yet.

## Planned Tech Stack

- **Language:** Rust
- **Web framework:** Axum
- **Database:** PostgreSQL 17 with sqlx for migrations
- **Search:** Meilisearch (trilingual index)
- **Offline export:** SQLite snapshots
- **External dependencies:** `lontar-ocr` (corpus ingestion), `lontar` (rendering/export), `lontar-aksara` (Aksara Bali shaping via rustybuzz)

## Architecture (Four Layers)

1. **Corpus** — manuscript images → Unicode text via lontar-ocr; `corpus` table stores sourced text records with date ranges and authority ranking
2. **Data** — PostgreSQL core: `entry` → `sense` → `attestation`, plus `etymology`, `entry_register` (unda-usuk), `cross_ref`, `entry_event` (append-only audit log)
3. **Search** — Meilisearch with three indices (one per language); Latin romanization only in V1, Aksara Bali search deferred
4. **Interface** — Axum REST API + editorial workflow (draft → reviewed → published); export via lontar crates

Dependencies flow strictly upward (no layer may depend on a higher layer).

## Domain-Specific Concepts

- **Unda-usuk:** Balinese speech register system (alus_singgih, alus_sor, alus_mider, andap, kasar). Structurally central — register is a dimension of the lexicon, not an annotation. A single concept has distinct word forms per register that are cross-linked.
- **Aksara Bali:** Traditional Balinese script, Unicode block U+1B00–U+1B7F. All entries carry both Latin romanization and Aksara form.
- **Attestation model:** Every sense is backed by dated textual evidence from manuscripts/inscriptions. Earliest attestation is always derived (never stored). Source authority ranking is explicit (prasasti > van_der_tuuk > zoetmulder > balai_bahasa > sutjaja > community).
- **Entry events:** All changes are append-only. No destructive updates.

## Key Design Files

- `ARCHITECTURE.md` — full schema DDL, layer descriptions, tech stack, search index design
- `TODO.md` — phased roadmap with dependencies between phases
- `README.md` — project context, source bibliography, comparison with existing Balinese dictionaries

## License

Data: CC BY-SA 4.0. Software: MIT or Apache-2.0.
