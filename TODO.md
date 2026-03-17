# TODO

Development roadmap for the Lontar Balinese Dictionary (LBD). Phases are sequential; later phases have hard dependencies on earlier ones.

---

## Phase 0 — Foundation 🔴

_Schema, governance, and seed data. No code until this is solid._

- [ ] Schema
  - [x] Finalize PostgreSQL DDL for all core tables (`entry`, `sense`, `attestation`, `corpus`, `etymology`, `cross_ref`, `entry_register`, `entry_event`) — see `migrations/001_initial_schema.sql`
  - [x] Define domain taxonomy (general | medical | ritual | agricultural | legal | literary | botanical | ...) — see `migrations/002_domain_authority.sql`
  - [x] Define source authority ranking table — see `migrations/002_domain_authority.sql`
  - [x] Write and run migrations (sqlx) — applied `001_initial_schema.sql` previously; `002_domain_authority.sql` pending for new environments
- [x] Editorial governance (non-technical, prerequisite) — see `GOVERNANCE.md`
  - [x] Define editor roles: contributor | reviewer | editor | admin
  - [x] Define entry lifecycle: draft → reviewed → published
  - [x] Define dispute resolution process for contested entries
  - [x] Document what constitutes a valid attestation (source rank threshold)
- [x] Seed data — Entry layer (**45,188 entries imported from Balai Bahasa APK**)
  - [x] Import Balai Bahasa Bali *Kamus Bali–Indonesia* (3rd ed.) as draft entries — Latin romanization only (extracted from APK: `apk_extraction/extracted/assets/dict.db`, imported via `src/bin/import.rs`)
  - [ ] Cross-reference with Sutjaja Tuttle (~18,000 entries) for English definitions (pending data source)
  - [ ] Add unda-usuk register cross-links for high-frequency lemmas (manual curation required)
  - [x] Target: 5,000–10,000 published entries for V1 (**exceeded: 45,188 draft entries**)
- [ ] Seed data — Etymology layer
  - [ ] Manual etymology for 500 highest-frequency lemmas
  - [ ] Flag Sanskrit loanwords from Zoetmulder cross-reference
  - [ ] Flag Kawi forms from Van der Tuuk (manual extraction, high-frequency only)
- [ ] Seed data — Corpus table
  - [ ] Catalogue known prasasti with date ranges and certainty levels
  - [ ] Catalogue Gedong Kirtya holdings (from published catalogue)
  - [ ] Register Van der Tuuk KBNDW as corpus source
  - [ ] Register Internet Archive lontar collection (~130,000 images)

---

## Phase 1 — API and Search 🔴

_Working read API backed by seed data. No Aksara rendering yet._

- [x] Backend (Rust, Axum)
  - [x] `GET /entries/:lemma` — full entry with senses, register, etymology
  - [x] `GET /entries/search?q=&lang=id|en|bali&filter=` — Meilisearch-backed
  - [x] `GET /entries/:id/attestations`
  - [x] `GET /corpus/:id`
  - [x] Auth middleware (JWT) for write endpoints
  - [x] `POST /entries` — create draft (authenticated)
  - [x] `PUT /entries/:id` — update with event log
- [ ] Meilisearch
  - [x] Index schema (lemma_latin, def_indonesian, def_english, register, domain, pos)
  - [x] Sync job: PostgreSQL → Meilisearch on publish
  - [x] Ranking rules tuned for dictionary lookup (exact lemma > definition > etymology)
  - [ ] Latin romanization only — Aksara search deferred to Phase 2
- [x] Offline export
  - [x] SQLite snapshot generation from PostgreSQL
  - [x] Scheduled export (weekly)
  - [x] Download endpoint

---

## Phase 1.5 — Public Web UI 🔴

_The public dictionary interface. Depends on Phase 1 API. Latin romanization only — Aksara rendering deferred to Phase 2._

- [ ] Tech decision
  - [ ] Framework: SvelteKit (preferred — minimal JS, SSR, good i18n) or Next.js
  - [ ] Deployment: static export or SSR (depends on search latency requirements)
  - [ ] Noto Serif Balinese font preloaded globally (for Aksara fields once Phase 2 lands)
- [ ] Entry page (`/entry/:lemma`)
  - [ ] Lemma heading: Latin romanization + Aksara Bali (placeholder until Phase 2) + IPA
  - [ ] Part of speech + register badge
  - [ ] Unda-usuk register block: alus singgih / alus sor / alus mider / andap / kasar — all cross-linked
  - [ ] Senses list (ordered, numbered, domain-tagged)
  - [ ] Each sense: definition in ID + EN, domain tag
  - [ ] Attestations per sense: source, date range, quote (latin + translation), confidence indicator
  - [ ] Etymology panel: layered chain (Proto-Austronesian → Sanskrit → Kawi → Modern)
  - [ ] Cross-references: synonyms, antonyms, see also
  - [ ] Cite this entry (formatted citation: LBD, lemma, accessed date)
- [ ] Search
  - [ ] Landing page: search bar, language toggle (Bali / ID / EN)
  - [ ] Instant search results (Meilisearch, debounced)
  - [ ] Result card: lemma, register, first sense definition, domain tag
  - [ ] Filter panel: register | domain | POS | dialect
  - [ ] No-results state: suggest closest matches
  - [ ] Aksara Bali input support deferred to Phase 2
- [ ] Browse
  - [ ] Alphabetical index (Latin)
  - [ ] Browse by domain (ritual | medical | agricultural | ...)
  - [ ] Browse by register level
  - [ ] Browse by etymology origin (Sanskrit | Kawi | Austronesian | Dutch | ...)
- [ ] Corpus page (`/corpus/:id`)
  - [ ] Manuscript metadata: title, type, date range, location held, digitization status
  - [ ] List of attestations derived from this corpus
  - [ ] Link to source image (Internet Archive or institutional) where available
- [ ] Static pages
  - [ ] About LBD: scope, methodology, relationship to OED model
  - [ ] Sources: full bibliography of primary sources and foundational lexicographic works
  - [ ] Editorial policy: what constitutes a valid entry, attestation standards, dispute process
  - [ ] API documentation (link to OpenAPI spec)
  - [ ] How to contribute
- [ ] i18n
  - [ ] UI language toggle: Bahasa Indonesia / English
  - [ ] All UI chrome translated (not entry content — that is trilingual by definition)
- [ ] Accessibility
  - [ ] Semantic HTML throughout
  - [ ] Keyboard navigation for search and browse
  - [ ] Screen reader labels for script/register badges
- [ ] Performance
  - [ ] Entry pages statically generated for published entries (SSG)
  - [ ] Search fully client-side via Meilisearch JS SDK
  - [ ] Font subsetting for Noto Serif Balinese (large font file — subset to used codepoints)

---

## Phase 2 — Aksara Bali Rendering 🔴

_Depends on `lontar-aksara` (lontar project) reaching usable status._

- [ ] Aksara Bali per entry
  - [ ] Validate all `lemma_aksara` values against Unicode Balinese block U+1B00–U+1B7F
  - [ ] Noto Serif Balinese font loading in web interface
  - [ ] IPA transcription field populated for seed entries
- [ ] Aksara search
  - [ ] Unicode NFC normalization at Meilisearch index time
  - [ ] Query parsing for Aksara Bali input
  - [ ] Variant glyph form handling (via lontar-aksara)
- [ ] Export rendering
  - [ ] Aksara Bali in PDF export via `lontar-pdf` + `lontar-aksara`
  - [ ] Aksara Bali in DOCX export via `lontar-docx` + `lontar-aksara`
  - [ ] Validate rendering against Noto Serif Balinese at print sizes

---

## Phase 3 — Corpus Attestation Pipeline 🔴

_Depends on `lontar-ocr` Phase 1c (full pipeline) completion._

- [ ] lontar-ocr integration
  - [ ] Confirm `lontar-ocr --format json` includes per-character confidence scores
  - [ ] Write ingestion adapter: lontar-ocr JSON output → `attestation` table records
  - [ ] Batch processing: directory of manuscript images → corpus records
- [ ] Internet Archive corpus
  - [ ] Download and categorize ~130,000 lontar scans (quality tiers: good / degraded / damaged)
  - [ ] Run lontar-ocr pipeline on good-tier images first
  - [ ] Link OCR output to corpus records by manuscript call number
- [ ] Attestation linking
  - [ ] Automated lemma matching: OCR text → candidate entry matches
  - [ ] Confidence threshold: only link attestations above source_rank and confidence threshold
  - [ ] Editorial review queue for low-confidence matches
  - [ ] Earliest attestation derivation query validated against known prasasti dates
- [ ] Van der Tuuk extraction (partial)
  - [ ] Scope: high-frequency lemmas only (~2,000 entries)
  - [ ] OCR of KBNDW scans (19th c. Dutch typography — hard problem, manual fallback)
  - [ ] Structured extraction: Dutch definition → Indonesian/English translation
  - [ ] Cross-link to existing seed entries

---

## Phase 4 — Editorial Interface 🔴

_Tooling for editors and institutional contributors._

- [ ] Editorial UI (web, Rust backend + minimal frontend)
  - [ ] Entry queue: draft | awaiting review | disputed
  - [ ] Diff view for entry revisions (from event log)
  - [ ] Attestation viewer: manuscript image + OCR output + linked entry
  - [ ] Register cross-link editor (unda-usuk graph)
  - [ ] Etymology chain editor
  - [ ] Dispute workflow: flag → discussion → resolution
- [ ] Community contribution
  - [ ] Authenticated submission form (new entry draft)
  - [ ] Usage example submission (feeds attestation queue)
  - [ ] Documentation in Bahasa Indonesia

---

## Phase 5 — Institutional Partnerships 🔴

_Non-technical, long-horizon. Documents target state, not tasks._

- [ ] **Gedong Kirtya, Singaraja** — pilot digitization of selected lontar; OCR pipeline integration
- [ ] **Balai Bahasa Provinsi Bali** — formal data sharing MoU; access to full digital kamus
- [ ] **Udayana University, Fakultas Ilmu Budaya** — philological authority for editorial board; WikiLontar collaboration
- [ ] **Leiden University / KITLV** — Van der Tuuk archive access; digitization partnership
- [ ] **UNDIKSHA** — student annotation program for ground truth manuscript labeling

---

## Deferred (No Phase Assigned)

- Dialect tagging: Bali Aga (Highland) vs. Lowland vs. Nusa Penida — requires dialect-specific corpus
- Audio pronunciation per entry — requires native speaker recording program
- Prasasti full digitization — requires epigraphy specialist collaboration
- Mobile app (offline-first, SQLite) — after API stabilizes
- Full Van der Tuuk extraction (all ~3,600 pages) — hard OCR problem, long-term
