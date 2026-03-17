

---


---

# 📜 Lontar Balinese Dictionary (LBD)

**The historical dictionary of the Balinese language — Balinese · Indonesian · English**

*Named after the palm leaf manuscripts of Bali — the primary medium through which Balinese knowledge, literature, and language have been preserved for over a millennium. LBD is the first attempt at an OED-grade historical dictionary of the Balinese language.*

---

## What is LBD?

LBD is an open, corpus-backed, historically grounded dictionary of the Balinese language — trilingual across Balinese, Indonesian, and English. Each entry traces a word from its earliest attested form in lontar manuscripts and copper plate inscriptions (*prasasti*) through its modern usage, with full register (*unda-usuk*) stratification and etymological depth.

The OED is the reference model: not a translation dictionary, not a learner's dictionary, but a scholarly record of what words mean, what they have meant, and where they come from — grounded in primary evidence.

```
lema: padem
aksara: ᬧᬤᭂᬫ᭄
register: alus singgih
pos: verba
etymology: Kawi paḍem < Sanskrit padma (lotus) → euphemistic register shift
senses:
  1. [ritual/formal] meninggal dunia (of royalty, high caste) — to die (honorific)
attestations:
  - Prasasti Sukawana A1 (~882 CE): "...sang hyang paḍem..."
  - Lontar Usada Rare (undated, Gedong Kirtya LT 1247): "yan padem sapisan..."
register_equivalents:
  andap: mati
  alus_sor: seda
  alus_singgih: padem
```

---

## Relationship to the Lontar Ecosystem

LBD is built on two existing projects:

| Project | Role in LBD |
|---|---|
| [`lontar-ocr`](https://github.com/your-org/lontar-ocr) | Corpus ingestion — converts manuscript images to Unicode text, feeds the attestation layer |
| [`lontar`](https://github.com/your-org/lontar) | Rendering and export — Aksara Bali text shaping (`lontar-aksara`), scholarly output formats (DOCX, PDF, LaTeX) |

LBD is not a standalone project. It is the dictionary layer that makes the outputs of `lontar-ocr` and `lontar` meaningful as preservation infrastructure.

---

## What Makes LBD Different

| Feature | Modern Kamus (Balai Bahasa) | BASAbali Wiki | LBD |
|---|---|---|---|
| Trilingual (Bali–ID–EN) | Bali–ID only | Bali–ID–EN | ✅ |
| Etymology | ❌ | ❌ | ✅ Full chain |
| Earliest attestation | ❌ | ❌ | ✅ Corpus-backed |
| Unda-usuk register cross-links | Partial | Partial | ✅ Structured |
| Aksara Bali per entry | ❌ | Partial | ✅ |
| IPA transcription | ❌ | ❌ | ✅ |
| Dialect tagging | ❌ | ❌ | ✅ |
| Open structured data | ❌ | Partial | ✅ CC BY-SA |
| Historical depth | ❌ | ❌ | ✅ OED-model |

---

## Project Status

| Phase | Status | Goal |
|---|---|---|
| 0 | 🔴 Design | Schema, data model, editorial governance |
| 1 | 🔴 Design | Seed database (5,000–10,000 entries), REST + GraphQL API |
| 1.5 | 🔴 Design | Public web UI — entry pages, search, browse, corpus viewer |
| 2 | 🔴 Design | Aksara Bali rendering, `lontar-aksara` integration |
| 3 | 🔴 Design | Corpus attestation pipeline via `lontar-ocr` |
| 4 | 🔴 Design | Editorial interface — review queue, diff view, dispute workflow |
| 5 | 🔴 Design | Institutional partnerships, Van der Tuuk extraction |

**Status legend:** 🔴 Design · 🟡 In Progress · 🟢 Usable · ✅ Stable

---

## Primary Sources

### Manuscript Collections
- **Gedong Kirtya, Singaraja** — largest lontar collection in the world (~3,000+ texts)
- **Leiden University Library** — Van der Tuuk bequest, partially digitized
- **Udayana University** — active transliteration program
- **Museum Bali, Denpasar**

### Foundational Lexicographic Works
- **Van der Tuuk**, *Kawi-Balineesch-Nederlandsch Woordenboek* (1897–1912) — 4 vols, ~3,600 pp — the historical backbone, public domain
- **Zoetmulder**, *Old Javanese-English Dictionary* (1982) — Kawi layer reference
- **Balai Bahasa Provinsi Bali**, *Kamus Bali–Indonesia* (3rd ed.) — modern lemma base
- **Sutjaja**, *Tuttle Concise Balinese Dictionary* — ~18,000 entries, register-aware

---

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full technical design.

## Roadmap

See [TODO.md](./TODO.md) for the detailed task breakdown.

---
## Getting Started (Scaffold)

### Prerequisites
- Rust toolchain (1.76+ recommended)
- PostgreSQL 17
- Meilisearch running locally (default http://localhost:7700)

### Setup
```bash
cp .env.example .env
# edit DATABASE_URL, MEILISEARCH_URL, MEILISEARCH_KEY

# Run migrations (requires database exists)
sqlx migrate run

# Seed sample data
psql -d lbd -f scripts/seed_data.sql

# Run server
cargo run
```

### API Smoke Test
```bash
curl http://127.0.0.1:3000/health
curl http://127.0.0.1:3000/entries/padem
```

### Current Core Dependencies
- axum 0.7.5
- tokio 1.37.0
- sqlx 0.7.4
- meilisearch-sdk 0.27.0
- serde/serde_json 1.0.x

### Runtime Check (latest deps)
- `cargo run` boots successfully (health endpoint OK)
- `cargo check` clean on updated dependencies

## License

Dictionary data: [Creative Commons Attribution-ShareAlike 4.0 (CC BY-SA 4.0)](https://creativecommons.org/licenses/by-sa/4.0/)

Software: Licensed under either of MIT or Apache-2.0 at your option.

---

*ᬮᭀᬦ᭄ᬢᬭ᭄ ᬩᬲ ᬩᬮᬶ — Lontar Basa Bali*
