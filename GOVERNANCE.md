# Editorial Governance

## Roles
- **Contributor**: submits drafts and suggestions.
- **Reviewer**: validates entries, senses, attestations, and etymology; can request changes.
- **Editor**: approves/publishes reviewed entries; manages dispute resolution.
- **Admin**: manages roles, system configuration, and escalations.

## Entry Lifecycle
```
draft → reviewed → published
  ↑          |
  └──────────┘ (revision cycle)
```
- **Draft**: created by contributor/editor. Not visible to public.
- **Reviewed**: validated by reviewer; can loop back to draft for changes.
- **Published**: approved by editor; public and versioned via event log.
- All changes are append-only via `entry_event`.

## Dispute Resolution
1. Flag entry/sense/attestation with reason.
2. Reviewer triage → either resolve or escalate to Editor.
3. Editor decision logged as `entry_event` (disputed → resolved) with diff.
4. If unresolved, Admin arbitrates and may lock the entry.

## Attestation Validity
- Must cite a corpus source with authority rank ≥ threshold (default: prasasti, van_der_tuuk, zoetmulder, balai_bahasa, sutjaja; community requires review).
- Include date range and certainty; earliest attestation is derived, not stored.
- OCR-derived attestations require confidence threshold; low-confidence → review queue.

## Registers & Cross-links
- Unda-usuk levels: alus_singgih | alus_sor | alus_mider | andap | kasar.
- Each concept links across registers via `entry_register.equivalent_id`.
- No destructive merges; use cross_refs and event log.

## Publishing Rules
- Minimum required: lemma (Latin), POS, at least one sense (ID/EN), register, status.
- Published entries must have reviewer approval and event log entry.
- Aksara fields optional until Phase 2.

## Security & Access
- Auth required for create/update endpoints.
- JWT/role mapping: contributor, reviewer, editor, admin (to be enforced in API middleware in Phase 1/4).
