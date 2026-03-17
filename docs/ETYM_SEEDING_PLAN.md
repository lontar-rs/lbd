# Etymology Seeding Plan (500 high-frequency lemmas)

Goal: seed etymology for the top 500 lemmas and flag Sanskrit/Kawi loans.

## Data Inputs
- Frequency list: derive top 500 lemmas from existing Balai Bahasa import (entry table).
- External references (not in repo):
  - Zoetmulder OJED (for Sanskrit loans; Kawi layer)
  - Van der Tuuk KBNDW (Kawi forms)
- Attestation: requires corpus references; placeholders until corpus seeds are added.

## Workflow
1) **Select top 500 lemmas**
```sql
SELECT lemma_latin
FROM entry
ORDER BY lemma_latin -- TODO: replace with frequency once available
LIMIT 500;
```
- Replace ORDER BY with frequency once we have corpus counts; for now alphabetical placeholder.

2) **Manual etymology entry** (seed placeholders)
- For each lemma:
  - Insert into `etymology` with `chain` as a JSON array of nodes (origin, language, form).
  - Link via `entry_event` with `event_type='etymology_seeded'` and `source='manual'`.

3) **Flag Sanskrit loanwords (Zoetmulder cross-ref)**
- Match lemma against Zoetmulder list (external).
- Add `entry_event` with `event_type='etymology_flag'`, diff: `{source:'zoetmulder', note:'sanskrit_loan_candidate'}`.
- Optional: add `etymology` chain node with `language='sanskrit'` and `form` if known.

4) **Flag Kawi forms (Van der Tuuk)**
- Match lemma against Van der Tuuk Kawi forms (external).
- Add `entry_event` with `event_type='etymology_flag'`, diff: `{source:'van_der_tuuk', note:'kawi_candidate'}`.

5) **Review queue**
```sql
-- Entries lacking any etymology
SELECT e.lemma_latin
FROM entry e
WHERE NOT EXISTS (
    SELECT 1 FROM etymology et
    WHERE et.root_entry_id = e.id
);
```

## Blockers / Assumptions
- No frequency data yet; need corpus counts to rank top 500. Placeholder uses alphabetical order.
- External datasets (Zoetmulder, Van der Tuuk) not available in repo; need CSV lists for matching.
- Attestation data not seeded; etymology assertions should be tagged as provisional.

## Next Actions
1) Obtain frequency list (from corpus or proxy list) to rank lemmas.
2) Acquire Zoetmulder Sanskrit loan list and Van der Tuuk Kawi forms (CSV).
3) Write a small helper to generate `etymology` inserts for the top 500 with placeholders.
4) Generate review report for missing etymology.
