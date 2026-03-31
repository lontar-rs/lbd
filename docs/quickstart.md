---
layout: default
title: "Quick Start"
description: "Get the Lontar Balinese Dictionary running in 5 minutes"
prev:
  title: "Installation"
  url: "/docs/installation/"
next:
  title: "Installation"
  url: "/docs/installation/"
---

# 🚀 Quick Start Guide

Get the Lontar Balinese Dictionary running in 5 minutes!

## 🌱 One-Command Setup

For the fastest setup, use our automated installer:

```bash
git clone https://github.com/your-org/lontar-rs/lbd.git
cd lbd
./scripts/setup.sh
```

That's it! The setup script handles everything automatically.

---

## ⚡ Manual Quick Start

If you prefer manual setup, here's the fastest path:

### Prerequisites
- **Git** - `git --version`
- **Rust** - `rustc --version` (1.76+)
- **Node.js** - `node --version` (18+)
- **PostgreSQL** - `psql --version` (17+)

### Setup Commands

```bash
# 1. Clone and setup
git clone https://github.com/your-org/lontar-rs/lbd.git
cd lbd
cp .env.example .env

# 2. Setup database
sudo -u postgres createuser -s lbd_app
sudo -u postgres psql -c "ALTER USER lbd_app PASSWORD 'lbd_app_password_20260317';"
sudo -u postgres createdb -O lbd_app lbd

# 3. Build and run
cargo build
sqlx migrate run
cd frontend && pnpm install && cd ..
./meilisearch --master-key=masterKey --http-addr 127.0.0.1:7700 &
cargo run --bin lbd &
cd frontend && pnpm dev &

# 4. Import data
cargo run --bin import -- --file data/balai_bahasa_from_apk.csv --source balai-bahasa
./scripts/index_search.sh
```

---

## 🎯 Verify Installation

### Check Services

```bash
# Check all services
./scripts/status.sh
```

Expected output:
```
🗄️  Database
✅ PostgreSQL - Running on localhost:5432
   📊 Entries: 45188 total, 100 published

🚀 Application Services
✅ Backend - Running on http://localhost:3000
✅ Frontend - Running on http://localhost:5173
✅ Meilisearch - Running on http://localhost:7700
   📚 Indexed documents: 100

📋 Summary
🎉 All services are running!
```

### Test in Browser

Open http://localhost:5173 in your browser and try searching for:
- `padem` - "to die (honorific)"
- `air` - "serangga yg lebih besar dp kumbang"
- `api` - "api"

### Test API

```bash
# Health check
curl http://localhost:3000/health

# Search test
curl "http://localhost:3000/entries/search?q=padem&lang=bali"
```

---

## 🔍 First Search

### Web Interface
1. Open http://localhost:5173
2. Type "padem" in the search box
3. Select language: Bali / Indonesia / English
4. Press Enter or click Search

### Expected Results
```
Results (1)
padem          padem
umum           ritual
meninggal dunia (untuk orang terhormat)
to die (honorific register)
```

### API Search
```bash
# Search via API
curl "http://localhost:3000/entries/search?q=padem&lang=bali"

# Response
{
  "hits": [
    {
      "id": "11111111-1111-1111-1111-111111111111",
      "lemma_latin": "padem",
      "register": "umum",
      "first_sense": "meninggal dunia (untuk orang terhormat)",
      "first_sense_en": "to die (honorific register)",
      "domain": "ritual"
    }
  ],
  "estimated_total_hits": 1
}
```

---

## 🎮 Basic Usage

### Search Features
- **Trilingual**: Search in Balinese, Indonesian, or English
- **Instant**: Results appear as you type
- **Faceted**: Filter by register and domain
- **Detailed**: Click entries for full information

### Entry Details
Click any search result to see:
- **Lemma**: The word itself
- **Part of speech**: verb, noun, adjective, etc.
- **Register**: alus singgih, alus sor, andap, kasar
- **Definitions**: Indonesian and English translations
- **Etymology**: Word origins and language history
- **Attestations**: Manuscript evidence (when available)

### Language Examples

#### Balinese Search
```bash
# Search Balinese words
padem    # "to die (honorific)"
air      # "insect larger than beetle"
api      # "fire"
bumi     # "earth"
mati     # "to die"
```

#### Indonesian Search
```bash
# Search Indonesian definitions
"meninggal dunia"  # finds "padem"
"api"              # finds "api"
"bumi"             # finds "bumi"
```

#### English Search
```bash
# Search English translations
"to die"           # finds "padem", "mati"
"fire"             # finds "api"
"earth"            # finds "bumi"
```

---

## 🔧 Service Management

### Start Services
```bash
./scripts/start.sh
```

### Stop Services
```bash
./scripts/stop.sh
```

### Check Status
```bash
./scripts/status.sh
```

### View Logs
```bash
# All logs
tail -f backend.log frontend.log meilisearch.log

# Specific service
tail -f backend.log
```

---

## 📊 What You Have

### Dictionary Content
- **45,188 entries** from Balai Bahasa dictionary
- **82,194 senses** (multiple definitions per word)
- **Trilingual**: Balinese → Indonesian → English
- **Register stratification**: alus singgih, alus sor, andap, kasar
- **Domain categorization**: ritual, general, medical, etc.

### Technical Features
- **⚡ Fast search**: Meilisearch-powered instant search
- **🌐 Modern UI**: Responsive SvelteKit frontend
- **🔒 Secure**: JWT authentication and proper credential management
- **📱 Mobile-friendly**: Works on all devices
- **🔧 Type-safe**: Rust backend with SQLx

### Search Capabilities
- **Full-text search**: Search any part of the word or definition
- **Language filtering**: Search in specific languages
- **Register filtering**: Filter by speech register
- **Domain filtering**: Filter by usage context
- **Instant results**: Real-time search as you type

---

## 🎯 Next Steps

### Explore the Dictionary
1. **Try different searches**: "seda", "kirukang", "ngames"
2. **Browse entries**: Click results to see full details
3. **Test languages**: Switch between Bali/ID/EN search
4. **Check registers**: Look for alus singgih vs andap words

### Development
1. **Read [Contributing Guide](./CONTRIBUTING.md)**
2. **Check [TODO](./TODO.md)** for current priorities
3. **Review [API Documentation](./API.md)**
4. **Join the community** on GitHub Discussions

### Contribute
1. **Phase 4A**: Help build editorial interface
2. **Content**: Add more dictionary entries
3. **Frontend**: Improve UI/UX
4. **Backend**: Enhance search and performance

---

## 🆘 Troubleshooting

### Common Issues

#### Services Not Starting
```bash
# Check what's running
./scripts/status.sh

# Restart services
./scripts/stop.sh
./scripts/start.sh
```

#### Search Not Working
```bash
# Rebuild search index
./scripts/index_search.sh

# Check Meilisearch
curl http://localhost:7700/health
```

#### Database Issues
```bash
# Test database connection
psql -h localhost -U lbd_app -d lbd -c "SELECT 1;"

# Check entry count
psql -h localhost -U lbd_app -d lbd -c "SELECT COUNT(*) FROM entry;"
```

#### Frontend Issues
```bash
# Rebuild frontend
cd frontend
rm -rf node_modules pnpm-lock.yaml
pnpm install
cd ..
./scripts/stop.sh
./scripts/start.sh
```

### Get Help
- **[Installation Guide](./installation.md)** - Detailed setup
- **[Development Guide](./DEVELOPMENT.md)** - Development setup
- **[GitHub Issues](https://github.com/your-org/lontar-rs/lbd/issues)** - Report problems
- **[GitHub Discussions](https://github.com/your-org/lontar-rs/lbd/discussions)** - Ask questions

---

## 🌟 Success!

🎉 **Congratulations!** You now have a fully functional Lontar Balinese Dictionary with:

- ✅ **45,000+ Balinese words** ready to search
- ✅ **Trilingual search** (Bali → Indonesian → English)
- ✅ **Modern web interface** for easy exploration
- ✅ **Fast, instant search** results
- ✅ **Complete development environment**

### What to Explore Next
- **Search for different words** and see the results
- **Click on entries** to view detailed information
- **Try different languages** in the search interface
- **Read the documentation** to learn more
- **Consider contributing** to help improve the dictionary

**Welcome to the Lontar Balinese Dictionary community! 🌟**

---

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
