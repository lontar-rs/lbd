#!/bin/bash

# Lontar Balinese Dictionary - Start All Services
# This script starts the backend, frontend, and search engine

set -e

# Load environment variables from .env file
if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "❌ .env file not found. Please create it from .env.example"
    exit 1
fi

echo "🚀 Starting Lontar Balinese Dictionary Services..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Extract database connection details from DATABASE_URL
DB_HOST=$(echo $DATABASE_URL | sed -n 's/.*@\([^:]*\):.*/\1/p')
DB_PORT=$(echo $DATABASE_URL | sed -n 's/.*:\([0-9]*\)\/.*/\1/p')

# Function to check if a service is already running
check_service() {
    local service_name=$1
    local port=$2
    local check_command=$3

    if $check_command >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  $service_name is already running on port $port${NC}"
        return 0
    else
        return 1
    fi
}

# Function to wait for service to be ready
wait_for_service() {
    local service_name=$1
    local port=$2
    local max_attempts=30
    local attempt=1

    echo -e "${BLUE}⏳ Waiting for $service_name to be ready...${NC}"

    while [ $attempt -le $max_attempts ]; do
        if curl -s "http://localhost:$port" >/dev/null 2>&1; then
            echo -e "${GREEN}✅ $service_name is ready!${NC}"
            return 0
        fi

        echo -e "${BLUE}   Attempt $attempt/$max_attempts...${NC}"
        sleep 1
        attempt=$((attempt + 1))
    done

    echo -e "${RED}❌ $service_name failed to start within $max_attempts seconds${NC}"
    return 1
}

# Check if PostgreSQL is running
echo -e "${BLUE}🔍 Checking PostgreSQL...${NC}"
if ! pg_isready -h $DB_HOST -p $DB_PORT >/dev/null 2>&1; then
    echo -e "${RED}❌ PostgreSQL is not running. Please start PostgreSQL first.${NC}"
    echo -e "${YELLOW}💡 On Ubuntu/Debian: sudo systemctl start postgresql${NC}"
    exit 1
else
    echo -e "${GREEN}✅ PostgreSQL is running${NC}"
fi

# Start Meilisearch if not already running
echo -e "${BLUE}🔍 Checking Meilisearch...${NC}"
if ! check_service "Meilisearch" 7700 "curl -s $MEILISEARCH_URL/health"; then
    echo -e "${BLUE}🔍 Starting Meilisearch...${NC}"

    # Check if meilisearch binary exists
    if [ ! -f "./meilisearch" ]; then
        echo -e "${YELLOW}⚠️  Meilisearch binary not found. Installing...${NC}"
        curl -L https://install.meilisearch.com | sh
    fi

    # Extract host and port from MEILISEARCH_URL
    MEILISEARCH_HOST=$(echo $MEILISEARCH_URL | sed -n 's/.*\/\/\([^:]*\):.*/\1/p')
    MEILISEARCH_PORT=$(echo $MEILISEARCH_URL | sed -n 's/.*:\([0-9]*\)\/.*/\1/p')

    # Start Meilisearch in background
    nohup ./meilisearch --master-key=$MEILISEARCH_KEY --http-addr $MEILISEARCH_HOST:$MEILISEARCH_PORT > meilisearch.log 2>&1 &
    MEILISEARCH_PID=$!
    echo $MEILISEARCH_PID > .meilisearch.pid

    if wait_for_service "Meilisearch" $MEILISEARCH_PORT; then
        echo -e "${GREEN}✅ Meilisearch started (PID: $MEILISEARCH_PID)${NC}"
    else
        echo -e "${RED}❌ Failed to start Meilisearch${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✅ Meilisearch is already running${NC}"
fi

# Start Backend if not already running
echo -e "${BLUE}🔍 Checking Backend...${NC}"
if ! check_service "Backend" $SERVER_PORT "curl -s http://localhost:$SERVER_PORT/health"; then
    echo -e "${BLUE}🔍 Starting Backend...${NC}"
    cargo run --bin lbd > backend.log 2>&1 &
    BACKEND_PID=$!
    echo $BACKEND_PID > .backend.pid

    if wait_for_service "Backend" $SERVER_PORT; then
        echo -e "${GREEN}✅ Backend started (PID: $BACKEND_PID)${NC}"
    else
        echo -e "${RED}❌ Failed to start Backend${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✅ Backend is already running${NC}"
fi

# Start Frontend if not already running
echo -e "${BLUE}🔍 Checking Frontend...${NC}"
if ! check_service "Frontend" 5173 "curl -s http://localhost:5173"; then
    echo -e "${BLUE}🔍 Starting Frontend...${NC}"
    cd frontend
    pnpm dev > ../frontend.log 2>&1 &
    FRONTEND_PID=$!
    echo $FRONTEND_PID > ../.frontend.pid
    cd ..

    if wait_for_service "Frontend" 5173; then
        echo -e "${GREEN}✅ Frontend started (PID: $FRONTEND_PID)${NC}"
    else
        echo -e "${RED}❌ Failed to start Frontend${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✅ Frontend is already running${NC}"
fi

echo ""
echo -e "${GREEN}🎉 All LBD services are running!${NC}"
echo ""
echo -e "${BLUE}📱 Services:${NC}"
echo -e "   • Frontend: ${GREEN}http://localhost:5173${NC}"
echo -e "   • Backend API: ${GREEN}http://localhost:$SERVER_PORT${NC}"
echo -e "   • Meilisearch: ${GREEN}$MEILISEARCH_URL${NC}"
echo -e "   • Database: ${GREEN}$DATABASE_URL${NC}"
echo ""
echo -e "${YELLOW}💡 To stop all services, run: ./scripts/stop.sh${NC}"
echo -e "${YELLOW}💡 To view logs: tail -f backend.log frontend.log meilisearch.log${NC}"
