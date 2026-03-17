# LBD Architecture

## Overview

LBD is structured in four layers. Each layer has a clear boundary and dependency direction — no upward dependencies.

```
┌─────────────────────────────────────────────────────┐
│  4. Interface Layer                                 │
│     Public API · Editorial UI · Export              │
├─────────────────────────────────────────────────────┤
│  3. Search Layer                                    │
│     Meilisearch · Trilingual index                  │
├─────────────────────────────────────────────────────┤
│  2. Data Layer                                      │
│     PostgreSQL · Entry · Sense · Attestation        │
├─────────────────────────────────────────────────────┤
│  1. Corpus Layer                                    │
│     lontar-ocr · Manuscript pipeline                │
└─────────────────────────────────────────────────────┘
```

External dependencies flow upward from Layer 1:

```
Manuscript images
    → lontar-ocr (corpus ingestion)
        → PostgreSQL (structured storage)
            → Meilisearch (search index)
                → Axum API
                    → lontar (rendering/export)
```

---

## Layer 1 — Corpus

### Purpose

Convert manuscript images into dated, sourced Unicode text records that back attestation claims in dictionary entries.

### Components

**lontar-ocr** (external project, dependency)
- Input: manuscript image (lontar scan, prasasti photograph)
- Output: Unicode Aksara Bali text + per-character confidence scores
- Required output format for LBD: `--format json` with confidence per glyph

**Corpus record** (LBD-owned)
```sql
corpus (
    id          UUID PRIMARY KEY,
    title       TEXT NOT NULL,
    type        TEXT NOT NULL,          -- lontar | prasasti | modern_text | oral_recorded
    date_min    INTEGER,                -- year CE, null if unknown
    date_max    INTEGER,
    date_cert   TEXT,                   -- certain | probable | disputed | unknown
    period      TEXT NOT NULL,          -- old_balinese | middle_balinese | modern_balinese
    script      TEXT NOT NULL,          -- aksara_bali | latin | mixed
    location    TEXT,                   -- gedong_kirtya | leiden | udayana | ...
    call_number TEXT,
    dig_status  TEXT NOT NULL,          -- none | image_only | transliterated | verified
    license     TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

### Source Authority Ranking

Trust is explicit, not implicit. Every attestation carries its source rank.

```
1  prasasti          -- datable, primary, archaeological
2  van_der_tuuk      -- philologically rigorous, 19th c. Dutch
3  zoetmulder        -- Kawi layer authority
4  balai_bahasa      -- modern official
5  sutjaja           -- modern scholarly
6  community         -- flagged, requires review
```

---

## Layer 2 — Data

### Core Schema

#### Entry

The fundamental unit. Maps to one lemma across all registers and senses.

```sql
entry (
    id              UUID PRIMARY KEY,
    lemma_latin     TEXT NOT NULL UNIQUE,
    lemma_aksara    TEXT,               -- Unicode Balinese block U+1B00–U+1B7F
    ipa             TEXT,
    pos             TEXT,               -- noun | verb | adj | adv | particle | ...
    root            UUID REFERENCES entry(id),
    status          TEXT NOT NULL,      -- draft | reviewed | published
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

#### Register

Unda-usuk is structurally central to Balinese — not an annotation but a dimension of the lexicon. A single concept (e.g. "die") has distinct forms across registers that are cross-linked, not synonymized.

```sql
entry_register (
    entry_id        UUID REFERENCES entry(id),
    level           TEXT NOT NULL,      -- alus_singgih | alus_sor | alus_mider | andap | kasar
    dialect         TEXT,               -- lowland | bali_aga | nusa_penida
    equivalent_id   UUID REFERENCES entry(id),  -- same concept, different register
    PRIMARY KEY (entry_id, level)
)
```

#### Etymology

Layered etymology chain reflecting Balinese's linguistic history.

```sql
etymology (
    id              UUID PRIMARY KEY,
    entry_id        UUID REFERENCES entry(id),
    proto_austronesian  TEXT,
    proto_mp            TEXT,           -- Proto-Malayo-Polynesian
    sanskrit            TEXT,
    kawi                TEXT,           -- Old Javanese
    old_balinese        TEXT,
    loan_source     TEXT,               -- javanese | malay | dutch | portuguese | english
    loan_form       TEXT,
    notes           TEXT,
    confidence      TEXT NOT NULL       -- certain | probable | disputed | unknown
)
```

#### Sense

A single entry can have multiple senses, each trilingual, each domain-tagged.

```sql
sense (
    id              UUID PRIMARY KEY,
    entry_id        UUID REFERENCES entry(id),
    sense_order     INTEGER NOT NULL,
    def_balinese    TEXT,
    def_indonesian  TEXT NOT NULL,
    def_english     TEXT,
    domain          TEXT,               -- general | medical | ritual | agricultural | legal | ...
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

#### Attestation

The evidentiary backbone. Each sense can have multiple attestations; earliest attestation date is derived, not stored.

```sql
attestation (
    id              UUID PRIMARY KEY,
    sense_id        UUID REFERENCES sense(id),
    corpus_id       UUID REFERENCES corpus(id),
    quote_aksara    TEXT,
    quote_latin     TEXT,
    quote_trans_id  TEXT,               -- Indonesian translation
    quote_trans_en  TEXT,               -- English translation
    confidence      NUMERIC(3,2),       -- per-character confidence from lontar-ocr
    source_rank     INTEGER NOT NULL,   -- from authority ranking
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

#### Cross-reference

```sql
cross_ref (
    entry_id        UUID REFERENCES entry(id),
    ref_entry_id    UUID REFERENCES entry(id),
    type            TEXT NOT NULL       -- synonym | antonym | see_also | register_equivalent
)
```

### Derived Queries

Earliest attestation per entry (never stored, always derived):

```sql
SELECT e.lemma_latin, MIN(c.date_min) AS earliest_attested
FROM entry e
JOIN sense s ON s.entry_id = e.id
JOIN attestation a ON a.sense_id = s.id
JOIN corpus c ON c.id = a.corpus_id
WHERE c.date_cert IN ('certain', 'probable')
GROUP BY e.id;
```

---

## Layer 3 — Search

### Meilisearch Index

Three indices, one per language direction. Each document is a flattened entry representation.

```json
{
  "id": "uuid",
  "lemma_latin": "padem",
  "lemma_aksara": "ᬧᬤᭂᬫ᭄",
  "def_indonesian": "meninggal dunia (untuk orang terhormat)",
  "def_english": "to die (honorific register)",
  "register": "alus_singgih",
  "domain": "ritual",
  "pos": "verb",
  "etymology_summary": "Kawi paḍem < Sanskrit padma"
}
```

**Searchable fields:** `lemma_latin`, `lemma_aksara`, `def_indonesian`, `def_english`

**Filterable:** `register`, `domain`, `pos`, `dialect`

**Ranking rules (in order):**
1. Exact lemma match
2. Definition match
3. Etymology match

### Aksara Search (V2)

V1 supports Latin romanization search only. Aksara Bali direct search requires:
- Unicode normalization (NFC) at index time
- Font-aware query parsing (variant glyph forms)
- Deferred to when `lontar-aksara` is stable

---

## Layer 4 — Interface

### Public Web UI

The primary user-facing surface. Statically generated for published entries (SSG), client-side search via Meilisearch JS SDK.

**Stack:** SvelteKit (SSR + SSG) · Meilisearch JS · Noto Serif Balinese (subsetted)

**Key surfaces:**

- **Entry page** (`/entry/:lemma`) — lemma in Latin + Aksara Bali + IPA, unda-usuk register block with all cross-linked equivalents, ordered senses (trilingual), attestations per sense with source and date range, etymology chain panel, citation formatter
- **Search** — instant trilingual search (Bali / ID / EN), filter by register / domain / POS / dialect
- **Browse** — alphabetical index, by domain, by register level, by etymology origin (Sanskrit / Kawi / Austronesian / Dutch / ...)
- **Corpus page** (`/corpus/:id`) — manuscript metadata, list of derived attestations, link to source image
- **Static pages** — About, Sources, Editorial policy, API docs, How to contribute

**i18n:** UI chrome in Bahasa Indonesia and English. Entry content is trilingual by definition.

**Phase dependency:** Latin romanization only until Phase 2 (`lontar-aksara`). Aksara Bali entry fields render as Unicode text with Noto Serif Balinese once font shaping is validated.

### API (Axum)

REST for simple lookups. GraphQL for complex queries (etymology trees, sense hierarchies, corpus-linked attestations).

```
GET  /entries/:lemma                    -- single entry, full detail
GET  /entries/search?q=&lang=&filter=   -- search
GET  /entries/:id/attestations          -- all attestations for an entry
GET  /corpus/:id                        -- corpus record detail
POST /entries                           -- create (editorial, authenticated)
PUT  /entries/:id                       -- update (editorial, authenticated)
```

### Editorial Workflow

Every entry moves through explicit states:

```
draft → reviewed → published
  ↑                    |
  └────────────────────┘  (revision cycle)
```

All changes are append-only (event log). No destructive updates. OED's revision history model.

```sql
entry_event (
    id          UUID PRIMARY KEY,
    entry_id    UUID REFERENCES entry(id),
    editor_id   UUID,
    event_type  TEXT,       -- created | updated | reviewed | published | disputed
    diff        JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

### Export (lontar integration)

`lontar` (external project, dependency) handles all rendered output:

| Format | Use case | lontar crate |
|---|---|---|
| PDF | Archival, print | `lontar-pdf` |
| DOCX | Scholarly collaboration | `lontar-docx` |
| LaTeX | Journal submission | `lontar-latex` |
| HTML | Web interface | `lontar-html` |

Aksara Bali rendering in all formats via `lontar-aksara` (rustybuzz-based OpenType shaping).

---

## Etymology Chain Model

Balinese's layered history requires explicit modeling of language strata:

```
Proto-Austronesian (~4000 BCE)
    └── Proto-Malayo-Polynesian
            └── Old Balinese (pre-12th c. CE)
                    ├── Sanskrit borrowings (Hindu-Buddhist, direct)
                    ├── Kawi / Old Javanese (literary register, 12th c.+)
                    └── Middle Balinese (Majapahit period, 12th–16th c.)
                            └── Modern Balinese (17th c.–present)
                                    ├── Malay / Indonesian
                                    ├── Dutch (colonial period)
                                    └── English (contemporary)
```

Each node in the chain is a structured record, not a free-text field. Reconstruction confidence is explicit per node.

---

## Tech Stack Summary

| Component | Technology |
|---|---|
| Database | PostgreSQL 17 |
| Search | Meilisearch |
| Backend | Rust (Axum) |
| Public web UI | SvelteKit (SSR + SSG) |
| Corpus ingestion | lontar-ocr |
| Rendering / export | lontar |
| Aksara shaping | lontar-aksara (rustybuzz) |
| Offline / mobile | SQLite export |
| Data license | CC BY-SA 4.0 |

---

## Hard Problems (Deferred)

In order of difficulty:

1. **Aksara Bali OCR** — no production model exists. lontar-ocr is solving this.
2. **Manuscript dating** — most lontar are undated. Palaeographic dating requires specialist philologists.
3. **Van der Tuuk extraction** — 3,600 pages of 19th c. Dutch typography. OCR + structured NLP extraction.
4. **Trilingual definition quality** — Balinese → English directly requires scarce expertise.
5. **Editorial governance** — who has authority to publish, dispute, or correct an entry. Must be defined before the system is built.
