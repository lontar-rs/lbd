# 📦 Installation Guide

Complete installation instructions for the Lontar Balinese Dictionary.

## 🌱 Automated Installation (Recommended)

For most users, the automated setup handles everything:

```bash
git clone https://github.com/your-org/lontar-rs/lbd.git
cd lbd
./scripts/setup.sh
```

This installs:
- ✅ All dependencies (Rust, Node.js, PostgreSQL, Meilisearch)
- ✅ Database setup and migrations
- ✅ 45,000+ Balinese words imported
- ✅ Development environment ready

---

## 🔧 Manual Installation

### System Requirements

#### **Supported Operating Systems**
- ✅ **Linux** (Ubuntu 20.04+, Debian 11+, CentOS 8+, Fedora 35+)
- ✅ **macOS** (Intel, Apple Silicon)
- ⚠️ **Windows** (WSL2 recommended)

#### **Hardware Requirements**
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB free space
- **CPU**: Modern 64-bit processor

#### **Software Prerequisites**
- **Git** 2.0+
- **Curl** (for downloads)
- **Terminal/shell** access

---

## 📋 Step-by-Step Installation

### Step 1: Install Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH (or restart terminal)
source "$HOME/.cargo/env"

# Verify installation
rustc --version  # Should be 1.76+
cargo --version
```

### Step 2: Install Node.js and pnpm

#### **Linux (Ubuntu/Debian)**
```bash
# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install pnpm
npm install -g pnpm
```

#### **Linux (CentOS/Fedora)**
```bash
# Install Node.js
curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash -
sudo yum install -y nodejs

# Install pnpm
npm install -g pnpm
```

#### **macOS**
```bash
# Install Node.js
brew install node

# Install pnpm
npm install -g pnpm
```

#### **Windows (WSL2)**
```bash
# Inside WSL2 Ubuntu
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs
npm install -g pnpm
```

**Verify installation:**
```bash
node --version  # Should be 18+
pnpm --version
```

### Step 3: Install PostgreSQL

#### **Linux (Ubuntu/Debian)**
```bash
# Install PostgreSQL
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib

# Start and enable service
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

#### **Linux (CentOS/Fedora)**
```bash
# Install PostgreSQL
sudo dnf install -y postgresql-server postgresql-contrib

# Initialize and start
sudo postgresql-setup --initdb
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

#### **macOS**
```bash
# Install PostgreSQL
brew install postgresql
brew services start postgresql
```

#### **Windows (WSL2)**
```bash
# Inside WSL2 Ubuntu
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

**Verify installation:**
```bash
psql --version  # Should be 17+
```

### Step 4: Setup Database

```bash
# Create database user
sudo -u postgres createuser -s lbd_app

# Set password (replace with secure password)
sudo -u postgres psql -c "ALTER USER lbd_app PASSWORD 'lbd_app_password_20260317';"

# Create database
sudo -u postgres createdb -O lbd_app lbd

# Test connection
psql -h localhost -U lbd_app -d lbd -c "SELECT 1;"
```

### Step 5: Clone and Setup Project

```bash
# Clone repository
git clone https://github.com/your-org/lontar-rs/lbd.git
cd lbd

# Create environment file
cp .env.example .env

# Edit environment file
nano .env
```

**Environment configuration:**
```bash
# Database
DATABASE_URL=postgresql://lbd_app:lbd_app_password_20260317@localhost:5432/lbd

# Meilisearch
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=masterKey

# JWT
JWT_SECRET=lbd_jwt_secret_$(date +%s)

# Server
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Logging
RUST_LOG=info
```

### Step 6: Setup Meilisearch

```bash
# Download Meilisearch
curl -L https://install.meilisearch.com | sh

# Make executable
chmod +x meilisearch

# Test installation
./meilisearch --version
```

### Step 7: Build and Setup Project

```bash
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
cargo run --bin import -- --file data/balai_bahasa_from_apk.csv --source balai-bahasa --database-url "postgresql://lbd_app:lbd_app_password_20260317@localhost:5432/lbd"

# Create search index
./scripts/index_search.sh
```

---

## 🚀 Verification

### Test Services

```bash
# Check service status
./scripts/status.sh

# Start services
./scripts/start.sh

# Test in browser
open http://localhost:5173
```

### Test API

```bash
# Health check
curl http://localhost:3000/health

# Search test
curl "http://localhost:3000/entries/search?q=padem&lang=bali"
```

### Expected Results

- ✅ **Frontend** loads at http://localhost:5173
- ✅ **Backend** responds at http://localhost:3000/health
- ✅ **Search** returns results for "padem"
- ✅ **Database** contains 45,000+ entries

---

## 🔧 Troubleshooting

### Common Issues

#### **PostgreSQL Connection Failed**
```bash
# Check PostgreSQL status
sudo systemctl status postgresql

# Start PostgreSQL
sudo systemctl start postgresql

# Check user exists
sudo -u postgres psql -c "\du"

# Test connection manually
psql -h localhost -U lbd_app -d lbd
```

#### **Rust Build Failed**
```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build

# Check dependencies
cargo tree
```

#### **Frontend Build Failed**
```bash
# Clear cache
cd frontend
rm -rf node_modules pnpm-lock.yaml
pnpm install
cd ..
```

#### **Meilisearch Issues**
```bash
# Check binary
./meilisearch --version

# Download fresh binary
rm -f meilisearch
curl -L https://install.meilisearch.com | sh
chmod +x meilisearch
```

#### **Search Not Working**
```bash
# Check Meilisearch is running
curl http://localhost:7700/health

# Rebuild search index
./scripts/index_search.sh

# Check index status
curl http://localhost:7700/indexes/entries/stats \
  -H "Authorization: Bearer masterKey"
```

### Platform-Specific Issues

#### **macOS Permission Issues**
```bash
# Give Meilisearch permissions
sudo xattr -d com.apple.quarantine meilisearch

# Or use System Preferences to allow
```

#### **Windows WSL2 Issues**
```bash
# Ensure WSL2 is properly configured
wsl --version

# Check networking
curl -v http://localhost:3000/health
```

#### **Linux Port Conflicts**
```bash
# Check what's using ports
sudo netstat -tulpn | grep :3000
sudo netstat -tulpn | grep :5173
sudo netstat -tulpn | grep :7700

# Kill conflicting processes
sudo kill -9 <PID>
```

---

## 🔄 Updates and Maintenance

### Update Project
```bash
# Pull latest changes
git pull origin main

# Update dependencies
cargo update
cd frontend && pnpm update && cd ..

# Rebuild
cargo build

# Restart services
./scripts/stop.sh
./scripts/start.sh
```

### Update Dependencies
```bash
# Rust dependencies
cargo update

# Node.js dependencies
cd frontend
pnpm update
cd ..

# Meilisearch (if needed)
rm -f meilisearch
curl -L https://install.meilisearch.com | sh
chmod +x meilisearch
```

### Database Maintenance
```bash
# Backup database
pg_dump $DATABASE_URL > backup_$(date +%Y%m%d).sql

# Vacuum database
psql $DATABASE_URL -c "VACUUM ANALYZE;"

# Check database size
psql $DATABASE_URL -c "SELECT pg_size_pretty(pg_database_size('lbd'));"
```

---

## 📦 Alternative Installation Methods

### Docker (Future)

Docker installation is planned for Phase 4B:

```bash
# Future command
docker-compose up -d
```

### Package Managers

Future package manager installations:
```bash
# Future commands
npm install -g lontar-dictionary
brew install lontar-dictionary
```

---

## 📞 Getting Help

If you encounter installation issues:

1. **Check this guide** for common solutions
2. **Search existing issues** on GitHub
3. **Check troubleshooting section** above
4. **Create an issue** with:
   - Operating system and version
   - Error messages
   - Steps taken
   - Environment details

---

## 🎯 Next Steps

After successful installation:

1. **Read [Quick Start](./quickstart.md)** for basic usage
2. **Review [Contributing Guide](./CONTRIBUTING.md)** for development
3. **Check [API Documentation](./API.md)** for integration
4. **Explore [TODO](./TODO.md)** for project priorities

---

**Installation complete! 🎉**

*ᬮᭀᬦ᭄ᬢᬭᬄ ᬩᬲ ᬩᬮᬮᬶ — Lontar Basa Bali*
