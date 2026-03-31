# 📜 Lontar Balinese Dictionary (LBD)

**The historical dictionary of the Balinese language — Balinese · Indonesian · English**

Named after the palm leaf manuscripts of Bali — the primary medium through which Balinese knowledge, literature, and language have been preserved for over a millennium. LBD is the first attempt at an OED-grade historical dictionary of the Balinese language.

---

## 🚀 Quick Start

```bash
git clone https://github.com/lontar-rs/lbd.git
cd lbd
./scripts/setup.sh
./scripts/start.sh
open http://localhost:5173
```

That's it! You'll have a complete trilingual dictionary with 45,000+ Balinese words ready to search.

---

## ✨ Features

- **🔍 Trilingual Search**: Balinese → Indonesian → English
- **📚 45K+ Entries**: Complete Balai Bahasa dictionary corpus
- **🎯 Register System**: alus singgih, alus sor, andap, kasar
- **⚡ Instant Search**: Meilisearch-powered with <100ms response
- **🌐 Modern UI**: Responsive SvelteKit frontend
- **🔧 Developer Tools**: Automated setup and management

---

## 📚 Documentation

Complete documentation is available in the [`docs/`](./docs) folder:

- **[📖 Documentation Site](https://lontar-rs.github.io/lbd/)** - Full documentation
- **[🌱 Quick Start](./docs/quickstart/)** - 5-minute guide
- **[📦 Installation](./docs/installation/)** - Detailed setup
- **[🤝 Contributing](./docs/contributing/)** - Contribution guidelines
- **[🛠️ Development](./docs/development/)** - Technical details
- **[📡 API Reference](./docs/api/)** - Complete API docs

---

## 🏗️ Architecture

- **Backend**: Rust + Axum + SQLx + PostgreSQL
- **Frontend**: SvelteKit + TypeScript + TailwindCSS
- **Search**: Meilisearch for fast full-text search
- **Database**: PostgreSQL with 82,194 senses

---

## 🤝 Contributing

We welcome contributions! See the [Contributing Guide](./docs/contributing/) for details.

### 🚀 High Priority
- **📝 Editorial Interface**: Build admin tools (Phase 4A)
- **🔐 Authentication**: User login and role-based access
- **📋 Entry Management**: Draft → Review → Publish workflow

### 🎨 Areas to Help
- **📱 Mobile UI**: Improve mobile experience
- **🌐 Internationalization**: Add more languages
- **📚 Content**: Add more dictionary entries
- **🔍 Search**: Enhance algorithms

---

## 📊 What Makes LBD Different

| Feature | Modern Kamus | BASAbali Wiki | LBD |
|---|---|---|---|
| Trilingual (Bali–ID–EN) | Bali–ID only | Bali–ID–EN | ✅ |
| Etymology | ❌ | ❌ | ✅ Full chain |
| Earliest attestation | ❌ | ❌ | ✅ Corpus-backed |
| Register cross-links | Partial | Partial | ✅ Structured |
| Aksara Bali per entry | ❌ | Partial | ✅ |
| IPA transcription | ❌ | ❌ | ✅ |

---

## 🛠️ Service Management

```bash
# Start all services
./scripts/start.sh

# Check status
./scripts/status.sh

# Stop services
./scripts/stop.sh
```

---

## 📞 Getting Help

- **[GitHub Issues](https://github.com/lontar-rs/lbd/issues)** - Bug reports
- **[GitHub Discussions](https://github.com/lontar-rs/lbd/discussions)** - Questions
- **[Documentation](./docs/)** - Complete reference

---

## 🌟 Acknowledgments

- **Balai Bahasa Provinsi Bali** for foundational dictionary data
- **Gedong Kirtya** for manuscript preservation efforts
- **Balinese language community** for cultural stewardship

---

## 📜 License

- **Software**: Apache-2.0
- **Dictionary Data**: CC BY-SA 4.0

---

**Built with ❤️ for the Balinese language community**

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
