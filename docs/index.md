---
layout: default
title: "Lontar Balinese Dictionary"
description: "The historical dictionary of the Balinese language — Balinese · Indonesian · English"
---

# 📜 Lontar Balinese Dictionary (LBD)

**The historical dictionary of the Balinese language — Balinese · Indonesian · English**

*Named after the palm leaf manuscripts of Bali — the primary medium through which Balinese knowledge, literature, and language have been preserved for over a millennium. LBD is the first attempt at an OED-grade historical dictionary of the Balinese language.*

---

## 🚀 Quick Start

Get the Lontar Balinese Dictionary running in minutes:

```bash
# Clone the repository
git clone https://github.com/lontar-rs/lbd.git
cd lbd

# Run the automated setup (installs everything!)
./scripts/setup.sh

# Start all services
./scripts/start.sh

# Open your browser and start exploring!
open http://localhost:5173
```

**That's it!** You'll have a complete trilingual dictionary with 45,000+ Balinese words ready to search.

---

## 🎯 What Makes LBD Special

### 📚 **Comprehensive Dictionary**
- **45,188 entries** from Balai Bahasa corpus
- **Trilingual access**: Balinese → Indonesian → English
- **Register stratification**: Complete unda-usuk system
- **Etymological depth**: Word origins and language history

### ⚡ **Modern Technology**
- **Instant search**: Meilisearch-powered with <100ms response
- **Responsive design**: Works on all devices
- **Type-safe backend**: Rust with SQLx for reliability
- **Modern frontend**: SvelteKit with TypeScript

### 🏛️ **Academic Rigor**
- **OED-grade approach**: Historical dictionary methodology
- **Corpus-backed**: Evidence from manuscripts and sources
- **Wikipedia-style editorial process**: Community curation
- **Scholarly standards**: Proper citation and attribution

---

## 🌟 Current Status

### ✅ **Working Features**
- **🔍 Trilingual Search**: Instant search across all languages
- **📚 45K+ Entries**: Complete Balai Bahasa dictionary imported
- **🎯 Register System**: alus singgih, alus sor, andap, kasar
- **⚡ Fast Performance**: Sub-100ms search response times
- **🌐 Modern UI**: Clean, responsive web interface
- **🔧 Developer Tools**: Automated setup and management

### 🚧 **In Development**
- **📝 Editorial Interface**: Phase 4A admin tools (Next Sprint)
- **📜 Aksara Bali**: Unicode Balinese script rendering
- **📋 Corpus Attestations**: Manuscript evidence linking
- **🤝 Community Features**: User contributions and discussions

---

## 📖 Documentation

### � **Getting Started**
- **[🌱 Quick Start](./quickstart/)** - 5-minute guide to get running
- **[📦 Installation](./installation/)** - Detailed installation instructions
- **[📋 Project Overview](./)** - This page

### 🛠️ **Development**
- **[🤝 Contributing Guide](./contributing/)** - How to contribute
- **[🛠️ Development Guide](./development/)** - Detailed development setup
- **[📡 API Reference](./api/)** - Complete API documentation
- **[📋 Changelog](./changelog/)** - Version history and changes

### 📚 **Project Information**
- **[📋 Roadmap](./todo/)** - Development roadmap and tasks

---

## 🎮 Try It Now

### Search Examples

| Language | Query | Result |
|----------|-------|--------|
| **Balinese** | `padem` | "to die (honorific)" |
| **Indonesian** | `meninggal dunia` | finds "padem" |
| **English** | `to die` | finds "padem", "mati" |

### Features to Explore
- **Instant Search**: Type and see results immediately
- **Language Filters**: Search in specific languages
- **Entry Details**: Click any result for full information
- **Register Information**: See speech levels and context
- **Etymology**: Word origins and language history

---

## 🏗️ Architecture

### **Technology Stack**
- **Backend**: Rust + Axum + SQLx + PostgreSQL
- **Frontend**: SvelteKit + TypeScript + TailwindCSS
- **Search**: Meilisearch for fast full-text search
- **Database**: PostgreSQL with 82,194 senses

### **Project Structure**
```
lbd/
├── src/                    # Rust backend
├── frontend/               # SvelteKit frontend
├── docs/                   # Documentation (this site)
├── scripts/                # Management scripts
├── data/                   # Dictionary data
└── migrations/             # Database schema
```

### **Service Management**
```bash
# Start all services
./scripts/start.sh

# Check status
./scripts/status.sh

# Stop services
./scripts/stop.sh
```

---

## 🤝 Contributing

We welcome contributions from developers, linguists, and Balinese language enthusiasts!

### 🚀 **High Priority (Phase 4A)**
- **📝 Editorial Interface**: Build admin tools for dictionary curation
- **🔐 Authentication**: User login and role-based access
- **📋 Entry Management**: Draft → Review → Publish workflow

### 🎨 **Frontend & UX**
- **📱 Mobile Responsiveness**: Optimize for mobile devices
- **🌐 Internationalization**: Add more interface languages
- **♿ Accessibility**: WCAG compliance and screen reader support

### 🔧 **Backend & Infrastructure**
- **⚡ Performance**: Optimize search and database queries
- **🔍 Advanced Search**: Filters, faceted search, suggestions
- **🧪 Testing**: Unit tests, integration tests, E2E tests

### 📚 **Content & Linguistics**
- **📖 Dictionary Entries**: Add more Balinese words and definitions
- **📜 Etymology**: Build etymological chains and language histories
- **🎭 Register System**: Improve unda-usuk classifications

See the [Contributing Guide](./contributing/) for detailed guidelines.

---

## 📊 Impact

### **Cultural Preservation**
- **45K+ Words**: Largest digital Balinese dictionary
- **Trilingual Access**: Balinese → Indonesian → English
- **Scholarly Rigor**: OED-grade historical approach
- **Open Access**: CC BY-SA licensing for broad use

### **Technical Innovation**
- **Modern Stack**: Rust + SvelteKit + PostgreSQL + Meilisearch
- **Type Safety**: Comprehensive type system prevents bugs
- **Performance**: Sub-100ms search response times
- **Scalability**: Architecture supports 100K+ entries

### **Community Building**
- **Open Source**: Collaborative development model
- **Professional Documentation**: Comprehensive guides and API reference
- **Contributor Friendly**: Clear guidelines and automated setup
- **Sustainable**: Automated maintenance and management tools

---

## 🌟 Acknowledgments

- **Balai Bahasa Provinsi Bali** for the foundational dictionary data
- **Gedong Kirtya** for manuscript preservation efforts
- **Van der Tuuk** for pioneering Balinese lexicography
- **Balinese language community** for continued cultural stewardship

---

## 📞 Getting Help

### **Resources**
- **[GitHub Issues](https://github.com/lontar-rs/lbd/issues)** - Bug reports and feature requests
- **[GitHub Discussions](https://github.com/lontar-rs/lbd/discussions)** - General questions and discussions
- **[Documentation](./)** - Complete documentation reference

---

**Last Updated: {{ site.time | date: "%Y-%m-%d" }}**

**Documentation Version: v0.1.0**

### **Quick Commands**
```bash
# Setup development environment
./scripts/setup.sh

# Start all services
./scripts/start.sh

# Check service status
./scripts/status.sh

# Stop all services
./scripts/stop.sh
```

---

## 🎉 Ready to Start?

1. **[🌱 Quick Start](./quickstart/)** - Get running in 5 minutes
2. **[📦 Installation](./installation/)** - Detailed setup instructions
3. **[🤝 Contributing](./contributing/)** - Join the community
4. **[🛠️ Development](./development/)** - Technical details

---

**Welcome to the Lontar Balinese Dictionary community! 🌟**

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
