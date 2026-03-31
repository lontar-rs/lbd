#!/bin/bash

# Lontar Balinese Dictionary - Development Setup
# This script sets up the complete development environment for first-time users

set -e

echo "🌱 Lontar Balinese Dictionary - Development Setup"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_step() {
    echo -e "${BLUE}📋 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

# Function to detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install package based on OS
install_package() {
    local package=$1
    local os=$(detect_os)

    case $os in
        "linux")
            if command_exists apt-get; then
                sudo apt-get update && sudo apt-get install -y "$package"
            elif command_exists yum; then
                sudo yum install -y "$package"
            elif command_exists dnf; then
                sudo dnf install -y "$package"
            else
                print_error "Package manager not detected. Please install $package manually."
                exit 1
            fi
            ;;
        "macos")
            if command_exists brew; then
                brew install "$package"
            else
                print_error "Homebrew not found. Please install Homebrew first."
                exit 1
            fi
            ;;
        "windows")
            print_error "Windows setup not automated. Please install $package manually."
            exit 1
            ;;
        *)
            print_error "Unsupported OS. Please install $package manually."
            exit 1
            ;;
    esac
}

# Function to check and install prerequisites
check_prerequisites() {
    print_step "Checking system prerequisites..."

    local os=$(detect_os)
    print_info "Detected OS: $os"

    # Check Rust
    if command_exists rustc && command_exists cargo; then
        print_success "Rust is installed"
        rust_version=$(rustc --version | cut -d' ' -f2)
        print_info "Rust version: $rust_version"
    else
        print_warning "Rust not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust installed"
    fi

    # Check Node.js and pnpm
    if command_exists node && command_exists pnpm; then
        print_success "Node.js and pnpm are installed"
        node_version=$(node --version)
        pnpm_version=$(pnpm --version)
        print_info "Node.js version: $node_version"
        print_info "pnpm version: $pnpm_version"
    else
        print_warning "Node.js or pnpm not found. Installing..."

        # Install Node.js
        if ! command_exists node; then
            case $os in
                "linux")
                    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
                    sudo apt-get install -y nodejs
                    ;;
                "macos")
                    brew install node
                    ;;
                *)
                    print_error "Please install Node.js manually from https://nodejs.org/"
                    exit 1
                    ;;
            esac
        fi

        # Install pnpm
        npm install -g pnpm
        print_success "Node.js and pnpm installed"
    fi

    # Check PostgreSQL
    if command_exists psql && command_exists pg_isready; then
        print_success "PostgreSQL is installed"
        pg_version=$(psql --version | cut -d' ' -f3)
        print_info "PostgreSQL version: $pg_version"
    else
        print_warning "PostgreSQL not found. Installing..."
        case $os in
            "linux")
                sudo apt-get update
                sudo apt-get install -y postgresql postgresql-contrib
                ;;
            "macos")
                brew install postgresql
                brew services start postgresql
                ;;
            *)
                print_error "Please install PostgreSQL manually from https://postgresql.org/"
                exit 1
                ;;
        esac
        print_success "PostgreSQL installed"
    fi

    # Check curl
    if command_exists curl; then
        print_success "curl is available"
    else
        print_error "curl is required but not found. Please install curl."
        exit 1
    fi
}

# Function to setup database
setup_database() {
    print_step "Setting up PostgreSQL database..."

    # Start PostgreSQL service
    case $(detect_os) in
        "linux")
            sudo systemctl start postgresql || sudo service postgresql start
            sudo systemctl enable postgresql || print_warning "Could not enable PostgreSQL service"
            ;;
        "macos")
            brew services start postgresql
            ;;
    esac

    # Wait for PostgreSQL to be ready
    local max_attempts=10
    local attempt=1
    while [ $attempt -le $max_attempts ]; do
        if pg_isready -h localhost -p 5432 >/dev/null 2>&1; then
            print_success "PostgreSQL is ready"
            break
        fi
        print_info "Waiting for PostgreSQL... (attempt $attempt/$max_attempts)"
        sleep 2
        attempt=$((attempt + 1))
    done

    if [ $attempt -gt $max_attempts ]; then
        print_error "PostgreSQL failed to start"
        exit 1
    fi

    # Create database and user
    print_info "Creating database and user..."

    # Check if user exists, create if not
    if ! sudo -u postgres psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='lbd_app'" | grep -q 1; then
        sudo -u postgres createuser -s lbd_app
        print_success "Created database user: lbd_app"
    else
        print_info "Database user 'lbd_app' already exists"
    fi

    # Set password for user
    sudo -u postgres psql -c "ALTER USER lbd_app PASSWORD 'lbd_app_password_20260317';"

    # Create database
    if ! sudo -u postgres psql -lqt | cut -d \| -f 1 | grep -qw lbd; then
        sudo -u postgres createdb -O lbd_app lbd
        print_success "Created database: lbd"
    else
        print_info "Database 'lbd' already exists"
    fi

    print_success "Database setup complete"
}

# Function to setup environment file
setup_environment() {
    print_step "Setting up environment configuration..."

    if [ -f ".env" ]; then
        print_warning ".env file already exists. Skipping creation."
        print_info "You can manually update .env if needed."
    else
        print_info "Creating .env file from template..."

        cat > .env << EOF
# Lontar Balinese Dictionary - Environment Configuration
# Generated by setup.sh on $(date)

# Database Configuration
DATABASE_URL=postgresql://lbd_app:lbd_app_password_20260317@localhost:5432/lbd

# Meilisearch Configuration
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=masterKey

# JWT Configuration
JWT_SECRET=lbd_jwt_secret_$(date +%s)

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Logging Configuration
RUST_LOG=info

# Frontend Configuration
VITE_API_BASE_URL=http://localhost:3000
EOF

        print_success "Created .env file"
        print_warning "Please review .env file and update values if needed"
    fi
}

# Function to setup Rust dependencies
setup_rust_deps() {
    print_step "Setting up Rust dependencies..."

    print_info "Building Rust project..."
    cargo build
    print_success "Rust dependencies installed and project built"

    # Run database migrations
    print_info "Running database migrations..."
    if cargo run --bin lbd -- migrate; then
        print_success "Database migrations completed"
    else
        print_warning "Migration command not found. Running sqlx migrate directly..."
        sqlx migrate run
        print_success "Database migrations completed"
    fi
}

# Function to setup frontend dependencies
setup_frontend_deps() {
    print_step "Setting up frontend dependencies..."

    cd frontend
    print_info "Installing frontend dependencies with pnpm..."
    pnpm install
    cd ..

    print_success "Frontend dependencies installed"
}

# Function to setup Meilisearch
setup_meilisearch() {
    print_step "Setting up Meilisearch..."

    # Download Meilisearch if not exists
    if [ ! -f "./meilisearch" ]; then
        print_info "Downloading Meilisearch..."
        curl -L https://install.meilisearch.com | sh
        print_success "Meilisearch downloaded"
    else
        print_info "Meilisearch already exists"
    fi

    # Create search index
    print_info "Creating search index..."

    # Start Meilisearch temporarily
    ./meilisearch --master-key=masterKey --http-addr 127.0.0.1:7700 > meilisearch_setup.log 2>&1 &
    MEILISEARCH_PID=$!

    # Wait for Meilisearch to be ready
    local max_attempts=10
    local attempt=1
    while [ $attempt -le $max_attempts ]; do
        if curl -s http://localhost:7700/health >/dev/null 2>&1; then
            print_success "Meilisearch is ready"
            break
        fi
        print_info "Waiting for Meilisearch... (attempt $attempt/$max_attempts)"
        sleep 2
        attempt=$((attempt + 1))
    done

    if [ $attempt -le $max_attempts ]; then
        # Create search index
        curl -X POST 'http://localhost:7700/indexes' \
             -H 'Content-Type: application/json' \
             -H 'Authorization: Bearer masterKey' \
             --data-binary '{"uid": "entries", "primaryKey": "id"}' \
             >/dev/null 2>&1
        print_success "Search index created"
    else
        print_warning "Could not create search index. You can create it later with the management scripts."
    fi

    # Stop Meilisearch
    kill $MEILISEARCH_PID 2>/dev/null || true
    wait $MEILISEARCH_PID 2>/dev/null || true
    rm -f meilisearch_setup.log
}

# Function to import sample data
import_sample_data() {
    print_step "Importing sample data..."

    # Check if seed data exists
    if [ -f "scripts/seed_data.sql" ]; then
        print_info "Importing seed data..."
        PGPASSWORD=lbd_app_password_20260317 psql -h localhost -U lbd_app -d lbd -f scripts/seed_data.sql
        print_success "Sample data imported"
    else
        print_warning "Seed data file not found. Skipping sample data import."
    fi

    # Import Balai Bahasa data if available
    if [ -f "data/balai_bahasa_from_apk.csv" ]; then
        print_info "Importing Balai Bahasa dictionary data..."
        if cargo run --bin import -- --file data/balai_bahasa_from_apk.csv --source balai-bahasa --database-url "postgresql://lbd_app:lbd_app_password_20260317@localhost:5432/lbd"; then
            print_success "Balai Bahasa data imported successfully"
        else
            print_warning "Could not import Balai Bahasa data. You can import it later with:"
            print_info "  cargo run --bin import -- --file data/balai_bahasa_from_apk.csv --source balai-bahasa"
        fi
    else
        print_warning "Balai Bahasa data file not found. Skipping dictionary import."
    fi
}

# Function to run final verification
verify_setup() {
    print_step "Verifying setup..."

    local all_good=true

    # Check database connection
    if PGPASSWORD=lbd_app_password_20260317 psql -h localhost -U lbd_app -d lbd -c "SELECT 1;" >/dev/null 2>&1; then
        print_success "Database connection verified"
        entry_count=$(PGPASSWORD=lbd_app_password_20260317 psql -h localhost -U lbd_app -d lbd -t -c "SELECT COUNT(*) FROM entry;" 2>/dev/null || echo "0")
        print_info "Entries in database: $entry_count"
    else
        print_error "Database connection failed"
        all_good=false
    fi

    # Check Rust build
    if cargo check >/dev/null 2>&1; then
        print_success "Rust project builds successfully"
    else
        print_error "Rust project has build errors"
        all_good=false
    fi

    # Check frontend dependencies
    if [ -d "frontend/node_modules" ]; then
        print_success "Frontend dependencies installed"
    else
        print_error "Frontend dependencies missing"
        all_good=false
    fi

    # Check Meilisearch
    if [ -f "./meilisearch" ]; then
        print_success "Meilisearch binary available"
    else
        print_error "Meilisearch binary missing"
        all_good=false
    fi

    return $([ "$all_good" = true ] && echo 0 || echo 1)
}

# Function to show next steps
show_next_steps() {
    echo ""
    echo -e "${GREEN}🎉 Setup completed successfully!${NC}"
    echo ""
    echo -e "${BLUE}🚀 Next Steps:${NC}"
    echo -e "   1. Start all services: ${GREEN}./scripts/start.sh${NC}"
    echo -e "   2. Check service status: ${GREEN}./scripts/status.sh${NC}"
    echo -e "   3. Open your browser: ${GREEN}http://localhost:5173${NC}"
    echo ""
    echo -e "${BLUE}📚 Development Commands:${NC}"
    echo -e "   • Start services: ${GREEN}./scripts/start.sh${NC}"
    echo -e "   • Stop services: ${GREEN}./scripts/stop.sh${NC}"
    echo -e "   • Check status: ${GREEN}./scripts/status.sh${NC}"
    echo -e "   • Import data: ${GREEN}./scripts/index_search.sh${NC}"
    echo ""
    echo -e "${BLUE}📖 Useful Links:${NC}"
    echo -e "   • Frontend: ${GREEN}http://localhost:5173${NC}"
    echo -e "   • Backend API: ${GREEN}http://localhost:3000${NC}"
    echo -e "   • API Health: ${GREEN}http://localhost:3000/health${NC}"
    echo -e "   • Meilisearch: ${GREEN}http://localhost:7700${NC}"
    echo ""
    echo -e "${CYAN}💡 Tips:${NC}"
    echo -e "   • All configuration is in ${YELLOW}.env${NC} file"
    echo -e "   • Logs are in ${YELLOW}backend.log${NC}, ${YELLOW}frontend.log${NC}, ${YELLOW}meilisearch.log${NC}"
    echo -e "   • Database contains ${YELLOW}45K+ Balinese words${NC} from Balai Bahasa"
    echo -e "   • Search is trilingual: Balinese → Indonesian → English"
    echo ""
    echo -e "${GREEN}Happy coding! 🌟${NC}"
}

# Main setup flow
main() {
    echo ""
    print_info "This script will set up the complete Lontar Balinese Dictionary development environment."
    print_info "It will install dependencies, configure the database, and import sample data."
    echo ""

    # Ask for confirmation
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Setup cancelled."
        exit 0
    fi

    # Run setup steps
    check_prerequisites
    setup_database
    setup_environment
    setup_rust_deps
    setup_frontend_deps
    setup_meilisearch
    import_sample_data

    # Verify everything works
    if verify_setup; then
        show_next_steps
    else
        echo ""
        print_error "Setup completed with errors. Please check the messages above."
        print_info "You can try running the setup again or fix the issues manually."
        exit 1
    fi
}

# Run main function
main "$@"
