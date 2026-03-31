# Corpus Seeding Plan

Goal: seed the `corpus` table with initial catalogue records for key sources: prasasti, Gedong Kirtya holdings, Van der Tuuk KBNDW, and Internet Archive lontar images.

## Schema Reminder (from migrations/001_initial_schema.sql)
`corpus` fields: id (UUID), title, type, date_min, date_max, date_cert, period, script, location, call_number, dig_status, license, created_at.

## Sources & Strategy
- **Prasasti (inscriptions)**
  - Data needed: title/name, date range, certainty (certain/probable/disputed), location, script (aksara_bali), period (old_balinese/middle_balinese), type=prasasti.
  - Acquisition: manual CSV from published catalogues.
- **Gedong Kirtya holdings**
  - Data needed: title, call_number, type=lontar, script=aksara_bali, period=middle/modern_balinese, location=gedong_kirtya, dig_status=image_only/transliterated/verified.
  - Acquisition: published catalogue; enter high-priority items first.
- **Van der Tuuk KBNDW**
  - Treat as corpus source for Kawi forms; type=literary, location=leiden, script=aksara_bali/latin.
  - Call numbers from KBNDW volumes.
- **Internet Archive lontar (~130k images)**
  - Type=lontar, location=internet_archive, script=image_only, dig_status=image_only, license per IA item.
  - Start with a stub aggregate record noting volume/collection link.

## Minimal CSV Template (to create `data/corpus_seed.csv`)
```
title,type,date_min,date_max,date_cert,period,script,location,call_number,dig_status,license
Prasasti Blanjong,prasasti,914,914,certain,old_balinese,aksara_bali,denpasar,,image_only,public_domain
GK Aji Caka,lontar,, ,unknown,middle_balinese,aksara_bali,gedong_kirtya,GK-001,image_only,public_domain
KBNDW Vol 1,literary,1897,1897,certain,modern_balinese,latin,leiden,KBNDW-1,verified,public_domain
Internet Archive Lontar Collection,lontar,,,,unknown,aksara_bali,internet_archive,,image_only,varies
```

## Import Approach
Use the existing import CLI pattern; add a simple Rust/SQL script later. For now, seed via SQL:
```sql
\copy corpus(title,type,date_min,date_max,date_cert,period,script,location,call_number,dig_status,license)
FROM 'data/corpus_seed.csv' WITH CSV HEADER;
```

## Blockers
- Need actual catalog data for prasasti and Gedong Kirtya call numbers.
- Need IA collection list or at least a stub link.
- Licensing per source must be recorded accurately.

## Next Actions
1) Collect minimal CSV rows for each source (start with 10–20 items, including stubs).
2) Run `\copy` to seed `corpus`.
3) Add per-source notes in `docs/DATA_IMPORT_SUMMARY.md` after seeding.
