#!/bin/bash

# Lontar Balinese Dictionary - Service Status
# This script checks the status of all LBD services

set -e

# Load environment variables from .env file
if [ -f ".env" ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "❌ .env file not found. Please create it from .env.example"
    exit 1
fi

echo "📊 Lontar Balinese Dictionary Service Status"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to check service status
check_service() {
    local service_name=$1
    local port=$2
    local url=$3

    if curl -s "$url" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ $service_name${NC} - Running on ${BLUE}http://localhost:$port${NC}"
        return 0
    else
        echo -e "${RED}❌ $service_name${NC} - Not running on port $port"
        return 1
    fi
}

# Function to check process by PID file
check_pid_file() {
    local service_name=$1
    local pid_file=$2

    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            echo -e "   ${BLUE}📋 PID file: $pid (running)${NC}"
        else
            echo -e "   ${YELLOW}📋 PID file: $pid (not running)${NC}"
        fi
    else
        echo -e "   ${YELLOW}📋 PID file: Not found${NC}"
    fi
}

# Extract database connection details from DATABASE_URL
DB_HOST=$(echo $DATABASE_URL | sed -n 's/.*@\([^:]*\):.*/\1/p')
DB_PORT=$(echo $DATABASE_URL | sed -n 's/.*:\([0-9]*\)\/.*/\1/p')
DB_USER=$(echo $DATABASE_URL | sed -n 's/.*\/\/\([^:]*\):.*/\1/p')
DB_NAME=$(echo $DATABASE_URL | sed -n 's/.*\/\([^?]*\).*/\1/p')
DB_PASSWORD=$(echo $DATABASE_URL | sed -n 's/.*:\([^@]*\)@.*/\1/p')

echo ""

# Check PostgreSQL
echo -e "${BLUE}🗄️  Database${NC}"
if pg_isready -h $DB_HOST -p $DB_PORT >/dev/null 2>&1; then
    echo -e "${GREEN}✅ PostgreSQL${NC} - Running on ${BLUE}$DB_HOST:$DB_PORT${NC}"

    # Check database exists and has entries
    if PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -d $DB_NAME -c "SELECT 1;" >/dev/null 2>&1; then
        entry_count=$(PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -d $DB_NAME -t -c "SELECT COUNT(*) FROM entry;" 2>/dev/null || echo "0")
        published_count=$(PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -d $DB_NAME -t -c "SELECT COUNT(*) FROM entry WHERE status = 'published';" 2>/dev/null || echo "0")
        echo -e "   ${BLUE}📊 Entries: $entry_count total, $published_count published${NC}"
    else
        echo -e "   ${YELLOW}⚠️  Database connection failed${NC}"
    fi
else
    echo -e "${RED}❌ PostgreSQL${NC} - Not running on port $DB_PORT"
fi

echo ""

# Check Application Services
echo -e "${BLUE}🚀 Application Services${NC}"

# Check Backend
backend_running=false
if check_service "Backend" $SERVER_PORT "http://localhost:$SERVER_PORT/health"; then
    backend_running=true
    check_pid_file "Backend" ".backend.pid"
fi

# Check Frontend
frontend_running=false
if check_service "Frontend" 5173 "http://localhost:5173"; then
    frontend_running=true
    check_pid_file "Frontend" ".frontend.pid"
fi

# Check Meilisearch
meilisearch_running=false
if check_service "Meilisearch" 7700 "$MEILISEARCH_URL/health"; then
    meilisearch_running=true
    check_pid_file "Meilisearch" ".meilisearch.pid"

    # Check search index stats
    if command -v curl >/dev/null 2>&1; then
        docs=$(curl -s $MEILISEARCH_URL/indexes/entries/stats -H "Authorization: Bearer $MEILISEARCH_KEY" 2>/dev/null | grep -o '"numberOfDocuments":[0-9]*' | cut -d':' -f2 2>/dev/null || echo "0")
        echo -e "   ${BLUE}📚 Indexed documents: $docs${NC}"
    fi
fi

echo ""

# Summary
echo -e "${BLUE}📋 Summary${NC}"
all_running=true

if [ "$backend_running" = true ] && [ "$frontend_running" = true ] && [ "$meilisearch_running" = true ]; then
    echo -e "${GREEN}🎉 All services are running!${NC}"
    echo ""
    echo -e "${BLUE}📱 Access URLs:${NC}"
    echo -e "   • Frontend: ${GREEN}http://localhost:5173${NC}"
    echo -e "   • Backend API: ${GREEN}http://localhost:$SERVER_PORT${NC}"
    echo -e "   • Meilisearch: ${GREEN}$MEILISEARCH_URL${NC}"
else
    echo -e "${YELLOW}⚠️  Some services are not running${NC}"
    all_running=false
fi

echo ""
echo -e "${BLUE}🔧 Management Commands:${NC}"
echo -e "   • Start all: ${GREEN}./scripts/start.sh${NC}"
echo -e "   • Stop all: ${GREEN}./scripts/stop.sh${NC}"
echo -e "   • Check status: ${GREEN}./scripts/status.sh${NC}"
echo -e "   • View logs: ${GREEN}tail -f backend.log frontend.log meilisearch.log${NC}"

if [ "$all_running" = false ]; then
    echo ""
    echo -e "${YELLOW}💡 To start missing services, run: ./scripts/start.sh${NC}"
    exit 1
fi
