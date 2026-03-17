# Sutjaja Cross-Reference Plan

We do **not** have Sutjaja data yet. This document defines how to ingest and align it once provided.

## Expected Input
- Format: CSV/TSV or JSON
- Required columns: `lemma_latin`, `def_english`
- Optional: `pos`, `register`, `domain`, `sense_order` (if available)

## Mapping to LBD Schema
- Match on `lemma_latin` against existing `entry.lemma_latin`
- Sense mapping:
  - If the Sutjaja dataset has **one definition per lemma** (likely), map to `sense_order = 1`.
  - If multiple definitions per lemma are present, use provided `sense_order`/domain; otherwise flag for manual review.
- If entry missing: insert `entry` with `status='draft'`
- Insert/merge `sense.def_english` if missing; keep `def_indonesian` from Balai Bahasa
- Tag source in `entry_event` as `sutjaja`
- Document assumption in import CLI: default `sense_order = 1` when not provided.

## CLI Usage (once data available)
```bash
cargo run --bin import -- \
  --file data/sutjaja.csv \
  --source sutjaja \
  --database-url $DATABASE_URL
```

## Conflict Handling
- If English definition exists: keep existing, log conflict to review list
- If POS differs: prefer existing, append note in `entry_event`
- If lemma not found: create new draft entry with English definition only

## Review Queue
- Lemmas missing any English definition (covers no senses and senses with def_english NULL):
```sql
SELECT e.lemma_latin
FROM entry e
WHERE NOT EXISTS (
    SELECT 1 FROM sense s
    WHERE s.entry_id = e.id
      AND s.def_english IS NOT NULL
);
```

## Next Actions (blocked until data)
1) Obtain Sutjaja dataset (CSV/JSON)
2) Normalize to expected columns
3) Run import CLI with `--source sutjaja`
4) Review conflict report
