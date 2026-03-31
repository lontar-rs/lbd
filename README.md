

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

## 🚀 Quick Start

Get the Lontar Balinese Dictionary running in minutes with our automated setup:

```bash
# Clone the repository
git clone https://github.com/your-org/lontar-rs/lbd.git
cd lbd

# Run the automated setup (installs everything)
./scripts/setup.sh

# Start all services
./scripts/start.sh

# Open your browser
open http://localhost:5173
```

**That's it!** The setup script handles:
- ✅ All dependencies (Rust, Node.js, PostgreSQL, Meilisearch)
- ✅ Database setup and migrations
- ✅ 45,000+ Balinese words imported
- ✅ Trilingual search configured
- ✅ Development environment ready

---

## 📋 Current Status

### ✅ **Working Features**
- **🔍 Trilingual Search**: Balinese → Indonesian → English
- **📚 45K+ Entries**: Imported from Balai Bahasa dictionary
- **🎯 Register Stratification**: alus singgih, alus sor, andap, kasar
- **🔧 Editorial Workflow**: Draft → Review → Publish (Wikipedia-style)
- **🌐 Modern Web UI**: SvelteKit frontend with responsive design
- **⚡ Fast Search**: Meilisearch-powered instant search
- **📖 Entry Pages**: Detailed word definitions with etymology
- **🔒 Security**: JWT authentication and proper credential management

### 🚧 **In Development**
- **📝 Editorial Interface**: Phase 4A - Basic admin UI (Next Sprint)
- **📜 Aksara Bali Rendering**: Unicode Balinese script display
- **📋 Corpus Attestations**: Manuscript evidence linking
- **🤝 Community Features**: User contributions and discussions

---

## 🛠️ Development Setup

### Prerequisites
- **Git** for version control
- **Terminal/shell** access
- **Internet connection** (for downloads)

### Automated Setup (Recommended)
```bash
./scripts/setup.sh
```

### Manual Setup
If you prefer manual setup, see [DEVELOPMENT.md](./DEVELOPMENT.md) for detailed instructions.

---

## 🔧 Service Management

All services are managed through simple scripts:

```bash
# Start all services
./scripts/start.sh

# Stop all services
./scripts/stop.sh

# Check service status
./scripts/status.sh
```

### Services
- **Frontend**: http://localhost:5173 (SvelteKit)
- **Backend API**: http://localhost:3000 (Rust/Axum)
- **Database**: PostgreSQL (localhost:5432)
- **Search**: Meilisearch (localhost:7700)

---

## 📚 Project Structure

```
lbd/
├── src/                    # Rust backend
│   ├── handlers/           # API endpoints
│   ├── models/            # Data models
│   └── search/            # Search integration
├── frontend/               # SvelteKit frontend
│   ├── src/lib/          # Components and utilities
│   └── src/routes/       # Page components
├── scripts/               # Management scripts
│   ├── setup.sh          # 🌱 First-time setup
│   ├── start.sh          # 🚀 Start services
│   ├── stop.sh           # 🛑 Stop services
│   └── status.sh         # 📊 Check status
├── data/                  # Dictionary data
│   └── balai_bahasa_*.csv # 45K+ word corpus
└── migrations/            # Database schema
```

---

## 🎯 Usage Examples

### Search for Words
```bash
# Search in the web interface
http://localhost:5173?q=padem

# Or via API
curl "http://localhost:3000/entries/search?q=padem&lang=bali"
```

### Import New Data
```bash
# Import from CSV
cargo run --bin import -- --file data/new_words.csv --source custom

# Update search index
./scripts/index_search.sh
```

### Database Operations
```bash
# Run migrations
sqlx migrate run

# Access database directly
psql $DATABASE_URL
```

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

### Quick Contribution Guide
1. **Fork** the repository
2. **Run setup**: `./scripts/setup.sh`
3. **Create feature branch**
4. **Make changes** and test
5. **Submit pull request**

### Areas for Contribution
- **📝 Editorial Interface**: Help build Phase 4A admin tools
- **📜 Aksara Bali**: Implement Balinese script rendering
- **🌐 Internationalization**: Add more languages
- **📱 Mobile UI**: Improve mobile experience
- **📚 Content**: Add more dictionary entries
- **🔍 Search**: Enhance search algorithms

---

## � Documentation

Complete documentation is available in the [`docs/`](./docs) folder:

### 🚀 **Getting Started**
- **[📖 Documentation Index](./docs/index.md)** - Complete documentation hub
- **[🌱 Quick Start](./docs/quickstart.md)** - 5-minute guide to get running
- **[📦 Installation](./docs/installation.md)** - Detailed installation instructions

### 🛠️ **Development**
- **[🤝 Contributing Guide](./docs/CONTRIBUTING.md)** - Contribution guidelines
- **[🛠️ Development Guide](./docs/DEVELOPMENT.md)** - Detailed development setup
- **[📡 API Reference](./docs/API.md)** - Complete API documentation

### 📋 **Project Information**
- **[📋 TODO/Roadmap](./docs/TODO.md)** - Development roadmap and tasks
- **[📋 Changelog](./docs/CHANGELOG.md)** - Version history and changes
- **[📜 License](./LICENSE)** - Project license information

### 🔧 **Technical Reference**
- **[🏗️ Architecture](./docs/ARCHITECTURE.md)** - System architecture and design
- **[🗄️ Database Schema](./docs/database.md)** - Database structure and relationships
- **[🔍 Search System](./docs/search.md)** - Search functionality and Meilisearch
- **[🚀 Deployment](./docs/deployment.md)** - Production deployment guide

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

Software: Licensed under Apache-2.0.

---

*ᬮᭀᬦ᭄ᬢᬭ᭄ ᬩᬲ ᬩᬮᬶ — Lontar Basa Bali*
