#!/bin/bash

# Lontar Balinese Dictionary - Stop All Services
# This script stops the backend, frontend, and search engine

set -e

echo "🛑 Stopping Lontar Balinese Dictionary Services..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to stop service by PID file
stop_service() {
    local service_name=$1
    local pid_file=$2

    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            echo -e "${BLUE}🔄 Stopping $service_name (PID: $pid)...${NC}"
            kill "$pid"

            # Wait for process to stop
            local count=0
            while kill -0 "$pid" 2>/dev/null && [ $count -lt 10 ]; do
                sleep 1
                count=$((count + 1))
            done

            if kill -0 "$pid" 2>/dev/null; then
                echo -e "${YELLOW}⚠️  Force stopping $service_name...${NC}"
                kill -9 "$pid"
            fi

            echo -e "${GREEN}✅ $service_name stopped${NC}"
        else
            echo -e "${YELLOW}⚠️  $service_name PID $pid not running${NC}"
        fi
        rm -f "$pid_file"
    else
        echo -e "${YELLOW}⚠️  No PID file found for $service_name${NC}"
    fi
}

# Function to stop service by process name
stop_by_name() {
    local service_name=$1
    local process_pattern=$2

    local pids=$(pgrep -f "$process_pattern" 2>/dev/null || true)
    if [ -n "$pids" ]; then
        echo -e "${BLUE}🔄 Stopping $service_name...${NC}"
        echo "$pids" | xargs kill 2>/dev/null || true

        # Wait and force kill if needed
        sleep 2
        pids=$(pgrep -f "$process_pattern" 2>/dev/null || true)
        if [ -n "$pids" ]; then
            echo -e "${YELLOW}⚠️  Force stopping $service_name...${NC}"
            echo "$pids" | xargs kill -9 2>/dev/null || true
        fi

        echo -e "${GREEN}✅ $service_name stopped${NC}"
    else
        echo -e "${YELLOW}⚠️  No $service_name processes found${NC}"
    fi
}

# Stop services using PID files (preferred method)
echo -e "${BLUE}🔄 Stopping services with PID files...${NC}"
stop_service "Backend" ".backend.pid"
stop_service "Frontend" ".frontend.pid"
stop_service "Meilisearch" ".meilisearch.pid"

# Also try to stop by process name (fallback method)
echo -e "${BLUE}🔄 Checking for remaining processes...${NC}"
stop_by_name "Backend" "cargo run --bin lbd"
stop_by_name "Frontend" "pnpm dev"
stop_by_name "Frontend" "vite.*dev"
stop_by_name "Meilisearch" "meilisearch"
stop_by_name "Frontend" "esbuild"

# Clean up any remaining processes on specific ports
echo -e "${BLUE}🔄 Checking for processes on LBD ports...${NC}"

for port in 3000 5173 7700; do
    local pid=$(lsof -ti:$port 2>/dev/null || true)
    if [ -n "$pid" ]; then
        echo -e "${BLUE}🔄 Killing process on port $port (PID: $pid)...${NC}"
        kill "$pid" 2>/dev/null || true
        sleep 1
        # Force kill if still running
        pid=$(lsof -ti:$port 2>/dev/null || true)
        if [ -n "$pid" ]; then
            kill -9 "$pid" 2>/dev/null || true
        fi
    fi
done

# Clean up PID files
echo -e "${BLUE}🧹 Cleaning up PID files...${NC}"
rm -f .backend.pid .frontend.pid .meilisearch.pid

# Verify services are stopped
echo -e "${BLUE}🔍 Verifying services are stopped...${NC}"

services_stopped=true
for service in "Backend" "Frontend" "Meilisearch"; do
    case $service in
        "Backend")
            if curl -s http://localhost:3000/health >/dev/null 2>&1; then
                echo -e "${RED}❌ Backend is still running${NC}"
                services_stopped=false
            else
                echo -e "${GREEN}✅ Backend stopped${NC}"
            fi
            ;;
        "Frontend")
            if curl -s http://localhost:5173 >/dev/null 2>&1; then
                echo -e "${RED}❌ Frontend is still running${NC}"
                services_stopped=false
            else
                echo -e "${GREEN}✅ Frontend stopped${NC}"
            fi
            ;;
        "Meilisearch")
            if curl -s http://localhost:7700/health >/dev/null 2>&1; then
                echo -e "${RED}❌ Meilisearch is still running${NC}"
                services_stopped=false
            else
                echo -e "${GREEN}✅ Meilisearch stopped${NC}"
            fi
            ;;
    esac
done

echo ""
if [ "$services_stopped" = true ]; then
    echo -e "${GREEN}🎉 All LBD services stopped successfully!${NC}"
else
    echo -e "${YELLOW}⚠️  Some services may still be running. Check manually:${NC}"
    echo -e "${YELLOW}   • Backend: curl http://localhost:3000/health${NC}"
    echo -e "${YELLOW}   • Frontend: curl http://localhost:5173${NC}"
    echo -e "${YELLOW}   • Meilisearch: curl http://localhost:7700/health${NC}"
fi

echo ""
echo -e "${BLUE}📝 Logs are preserved in:${NC}"
echo -e "   • backend.log"
echo -e "   • frontend.log"
echo -e "   • meilisearch.log"
echo ""
echo -e "${YELLOW}💡 To start services again, run: ./scripts/start.sh${NC}"
