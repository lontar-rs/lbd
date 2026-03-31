# 🛠️ Development Guide

Detailed development setup and technical documentation for the Lontar Balinese Dictionary.

## 🌱 Quick Setup (Recommended)

For most contributors, use the automated setup:

```bash
./scripts/setup.sh
```

This handles everything: dependencies, database, data import, and configuration.

---

## 🔧 Manual Setup

If you prefer manual setup or need to understand the process:

### Prerequisites

#### Required Software
- **Rust** (stable 1.76+)
- **Node.js** (18+)
- **pnpm** (latest)
- **PostgreSQL** (17+)
- **curl** (for downloads)
- **Git** (for version control)

#### OS Support
- ✅ **Linux** (Ubuntu, Debian, CentOS, etc.)
- ✅ **macOS** (Intel, Apple Silicon)
- ⚠️ **Windows** (WSL2 recommended)

### Step 1: Install Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH (or restart terminal)
source "$HOME/.cargo/env"

# Verify installation
rustc --version
cargo --version
```

### Step 2: Install Node.js and pnpm

```bash
# Install Node.js (Linux)
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Node.js (macOS)
brew install node

# Install pnpm globally
npm install -g pnpm

# Verify installation
node --version
pnpm --version
```

### Step 3: Install PostgreSQL

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql

# macOS
brew install postgresql
brew services start postgresql

# Verify installation
psql --version
```

### Step 4: Setup Database

```bash
# Start PostgreSQL service
sudo systemctl start postgresql  # Linux
brew services start postgresql   # macOS

# Create database user
sudo -u postgres createuser -s lbd_app

# Set password
sudo -u postgres psql -c "ALTER USER lbd_app PASSWORD 'lbd_app_password_20260317';"

# Create database
sudo -u postgres createdb -O lbd_app lbd

# Test connection
psql -h localhost -U lbd_app -d lbd -c "SELECT 1;"
```

### Step 5: Setup Environment

```bash
# Create environment file
cp .env.example .env

# Edit .env with your configuration
nano .env
```

Example `.env` file:
```bash
DATABASE_URL=postgresql://lbd_app:lbd_app_password_20260317@localhost:5432/lbd
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=masterKey
JWT_SECRET=lbd_jwt_secret_$(date +%s)
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
RUST_LOG=info
```

### Step 6: Setup Meilisearch

```bash
# Download Meilisearch
curl -L https://install.meilisearch.com | sh

# Make executable
chmod +x meilisearch

# Start Meilisearch (for setup)
./meilisearch --master-key=masterKey --http-addr 127.0.0.1:7700 &
MEILISEARCH_PID=$!

# Wait for it to be ready
sleep 3

# Create search index
curl -X POST 'http://localhost:7700/indexes' \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Bearer masterKey' \
     --data-binary '{"uid": "entries", "primaryKey": "id"}'

# Stop Meilisearch
kill $MEILISEARCH_PID
```

### Step 7: Build and Setup Project

```bash
# Clone repository (if not already done)
git clone https://github.com/your-org/lontar-rs/lbd.git
cd lbd

# Build Rust project
cargo build

# Run database migrations
sqlx migrate run

# Install frontend dependencies
cd frontend
pnpm install
cd ..
```

### Step 8: Import Data

```bash
# Import seed data
psql -h localhost -U lbd_app -d lbd -f scripts/seed_data.sql

# Import Balai Bahasa dictionary (45K+ words)
cargo run --bin import -- --file data/balai_bahasa_from_apk.csv --source balai-bahasa

# Update search index
./scripts/index_search.sh
```

---

## 🚀 Running the Application

### Development Mode

```bash
# Start all services
./scripts/start.sh

# Or start individually:
# Backend
cargo run --bin lbd

# Frontend (in another terminal)
cd frontend && pnpm dev

# Meilisearch (in another terminal)
./meilisearch --master-key=masterKey --http-addr 127.0.0.1:7700
```

### Service URLs

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:3000
- **Health Check**: http://localhost:3000/health
- **Meilisearch**: http://localhost:7700
- **Database**: postgresql://localhost:5432/lbd

---

## 🛠️ Development Tools

### Code Quality

```bash
# Format Rust code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test

# Run pre-commit hooks
pre-commit run --all-files
```

### Database Operations

```bash
# Create new migration
sqlx migrate add migration_name

# Run migrations
sqlx migrate run

# Rollback migration
sqlx migrate revert

# Access database
psql $DATABASE_URL

# Check database status
./scripts/status.sh
```

### Data Management

```bash
# Import CSV data
cargo run --bin import -- --file data/words.csv --source custom

# Export database
pg_dump $DATABASE_URL > backup.sql

# Update search index
./scripts/index_search.sh

# Check entry counts
psql $DATABASE_URL -c "SELECT COUNT(*) FROM entry;"
psql $DATABASE_URL -c "SELECT COUNT(*) FROM entry WHERE status = 'published';"
```

---

## 🏗️ Architecture Overview

### Backend (Rust)

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root
├── handlers/            # API route handlers
│   ├── mod.rs          # Handler module exports
│   ├── entries.rs      # Dictionary entries API
│   ├── search.rs       # Search functionality
│   ├── events.rs       # Entry events/audit log
│   └── corpus.rs       # Corpus/attestation API
├── models/              # Data models
│   ├── entry.rs        # Entry and related models
│   ├── sense.rs        # Definition senses
│   ├── etymology.rs    # Etymology chains
│   ├── event.rs        # Event logging
│   └── register.rs     # Register levels
├── search/              # Search integration
│   ├── client.rs       # Meilisearch client
│   └── indexing.rs     # Search indexing
├── auth/                # Authentication
├── error.rs            # Error handling
└── state.rs            # Application state
```

### Frontend (SvelteKit)

```
frontend/
├── src/
│   ├── lib/             # Shared utilities
│   │   ├── api.ts       # API client
│   │   ├── types.ts     # TypeScript types
│   │   └── stores.ts    # Svelte stores
│   ├── routes/          # Page components
│   │   ├── +page.svelte # Main search page
│   │   ├── entry/       # Entry detail pages
│   │   └── browse/      # Browse pages
│   └── app.html         # App template
├── package.json         # Dependencies
└── vite.config.ts       # Vite configuration
```

### Database Schema

```sql
-- Core tables
entry          # Dictionary entries
sense          # Definitions/senses
etymology      # Etymology chains
attestation    # Manuscript attestations
entry_event    # Audit trail
entry_register # Register classifications
corpus         # Source manuscripts
```

---

## 🔍 Search System

### Meilisearch Integration

The search system uses Meilisearch for fast, full-text search:

```rust
// Search client setup
let client = SearchClient::init(
    &env::var("MEILISEARCH_URL")?,
    &env::var("MEILISEARCH_KEY")?
)?;

// Search query
let results = client.search(
    "entries",
    "padem",
    Some("status = 'published'")
).await?;
```

### Indexing

Search index includes:
- **Lemma** (word)
- **Definitions** (Indonesian + English)
- **Register** (speech level)
- **Domain** (usage context)

### Updating Index

```bash
# Rebuild search index
./scripts/index_search.sh

# Or manually:
curl -X DELETE 'http://localhost:7700/indexes/entries' \
  -H 'Authorization: Bearer masterKey'
# Then re-run indexing script
```

---

## 📝 API Documentation

### Core Endpoints

```bash
# Health check
GET /health

# Search entries
GET /entries/search?q=<query>&lang=<lang>

# Get specific entry
GET /entries/<lemma>

# Get entry events
GET /entries/<id>/events

# Get attestations
GET /entries/<id>/attestations
```

### Response Format

```json
{
  "hits": [
    {
      "id": "uuid",
      "lemma_latin": "padem",
      "register": "umum",
      "first_sense": "meninggal dunia (untuk orang terhormat)",
      "first_sense_en": "to die (honorific)",
      "domain": "ritual"
    }
  ],
  "estimated_total_hits": 1,
  "processing_time_ms": 0
}
```

---

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test search::tests

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_search_functionality() {
        // Setup test database
        let pool = PgPool::connect(&test_db_url()).await.unwrap();

        // Test search
        let results = search_entries(&pool, "padem").await.unwrap();
        assert!(!results.is_empty());
    }
}
```

---

## 🐛 Debugging

### Common Issues

#### Database Connection
```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Test connection
psql -h localhost -U lbd_app -d lbd

# Check environment variables
echo $DATABASE_URL
```

#### Search Issues
```bash
# Check Meilisearch status
curl http://localhost:7700/health

# Check index status
curl http://localhost:7700/indexes/entries/stats \
  -H "Authorization: Bearer masterKey"

# Rebuild index
./scripts/index_search.sh
```

#### Build Issues
```bash
# Clean build
cargo clean && cargo build

# Check dependencies
cargo tree

# Update dependencies
cargo update
```

### Logging

```bash
# Enable debug logging
export RUST_LOG=debug

# View logs
tail -f backend.log
tail -f frontend.log
tail -f meilisearch.log
```

---

## 🚀 Deployment

### Environment Setup

Production requires:
- **PostgreSQL** (configured for production)
- **Meilisearch** (with persistent storage)
- **Reverse proxy** (nginx/caddy)
- **SSL certificates**

### Configuration

Production `.env`:
```bash
DATABASE_URL=postgresql://user:pass@prod-db:5432/lbd
MEILISEARCH_URL=https://search.example.com
MEILISEARCH_KEY=production-key
JWT_SECRET=secure-jwt-secret
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info
```

### Docker (Future)

Docker configuration is planned for Phase 4B.

---

## 📚 Additional Resources

### Documentation
- **[API.md](./API.md)** - Complete API reference
- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - System architecture
- **[TODO.md](./TODO.md)** - Development roadmap

### External Resources
- **Rust Book** - https://doc.rust-lang.org/book/
- **SvelteKit Docs** - https://kit.svelte.dev/
- **Meilisearch Docs** - https://docs.meilisearch.com/
- **PostgreSQL Docs** - https://www.postgresql.org/docs/

### Community
- **GitHub Issues** - Bug reports and questions
- **GitHub Discussions** - General discussion
- **Discord** (future) - Real-time chat

---

## 🤝 Getting Help

If you encounter issues:

1. **Check this guide** for common solutions
2. **Search existing issues** on GitHub
3. **Ask in Discussions** for help
4. **Create an issue** for bugs

---

**Happy developing! 🌟**

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
