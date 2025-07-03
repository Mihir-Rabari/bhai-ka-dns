#!/bin/bash

# Bhai Ka DNS - Quick Setup Script
# This script sets up the entire Bhai Ka DNS application

set -e

echo "🚀 Setting up Bhai Ka DNS - AI-Powered DNS Server"
echo "=================================================="

# Check prerequisites
check_prerequisites() {
    echo "📋 Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker is required but not installed. Please install Docker first."
        exit 1
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        echo "❌ Docker Compose is required but not installed. Please install Docker Compose first."
        exit 1
    fi
    
    # Check if ports are available
    if lsof -Pi :5353 -sTCP:LISTEN -t >/dev/null ; then
        echo "❌ Port 5353 is already in use. Please stop the service using this port."
        exit 1
    fi
    
    if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null ; then
        echo "❌ Port 8080 is already in use. Please stop the service using this port."
        exit 1
    fi
    
    echo "✅ Prerequisites check passed!"
}

# Setup environment
setup_environment() {
    echo "🔧 Setting up environment..."
    
    # Create .env file if it doesn't exist
    if [ ! -f .env ]; then
        cat > .env << EOF
# Database Configuration
DATABASE_URI=mongodb://admin:password@mongodb:27017/bhai_dns?authSource=admin

# Application Configuration
RUST_LOG=info
ENABLE_AI_FEATURES=true
ENABLE_ANALYTICS=true

# Security (CHANGE IN PRODUCTION!)
JWT_SECRET=your-super-secret-key-change-in-production

# Frontend
REACT_APP_API_URL=http://localhost:8080
EOF
        echo "✅ Created .env file"
    fi
    
    # Create monitoring directories
    mkdir -p monitoring/prometheus
    mkdir -p monitoring/grafana/dashboards
    mkdir -p monitoring/grafana/datasources
    
    # Create Prometheus config
    cat > monitoring/prometheus.yml << EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'bhai-dns'
    static_configs:
      - targets: ['bhai-dns-backend:9090']
  - job_name: 'redis'
    static_configs:
      - targets: ['redis:6379']
EOF

    # Create Redis config
    cat > redis.conf << EOF
# Redis configuration for Bhai Ka DNS
bind 0.0.0.0
port 6379
daemonize no
timeout 300
tcp-keepalive 60
loglevel notice
databases 16
save 900 1
save 300 10
save 60 10000
stop-writes-on-bgsave-error yes
rdbcompression yes
dbfilename dump.rdb
dir /data
maxmemory 512mb
maxmemory-policy allkeys-lru
appendonly yes
appendfilename "appendonly.aof"
appendfsync everysec
EOF
    
    echo "✅ Environment setup complete!"
}

# Load datasets
load_datasets() {
    echo "📊 Loading datasets for enhanced DNS functionality..."
    
    # Run the dataset loader script
    if [ -f "scripts/load-datasets.sh" ]; then
        chmod +x scripts/load-datasets.sh
        ./scripts/load-datasets.sh
    else
        echo "⚠️ Dataset loader script not found, creating sample data..."
        mkdir -p data/{threats,typos,popular}
        
        # Create minimal sample data
        echo "malware.com" > data/threats/sample-threats.txt
        echo "phishing.net" >> data/threats/sample-threats.txt
        echo "spam.org" >> data/threats/sample-threats.txt
        
        echo '{"google.com": ["gogle.com", "googel.com"]}' > data/typos/common-typos.json
        
        echo "google.com" > data/popular/top-domains.txt
        echo "youtube.com" >> data/popular/top-domains.txt
        echo "facebook.com" >> data/popular/top-domains.txt
        
        echo "✅ Sample datasets created"
    fi
}

# Build and start services
start_services() {
    echo "🏗️ Building and starting services..."
    
    # Build and start with Docker Compose
    docker-compose up -d --build
    
    echo "⏳ Waiting for services to start..."
    sleep 30
    
    # Check if services are running
    if docker-compose ps | grep -q "Up"; then
        echo "✅ Services started successfully!"
    else
        echo "❌ Some services failed to start. Check logs with: docker-compose logs"
        exit 1
    fi
}

# Verify installation
verify_installation() {
    echo "🔍 Verifying installation..."
    
    # Test API health
    if curl -f -s http://localhost:8080/health > /dev/null; then
        echo "✅ Backend API is responding"
    else
        echo "❌ Backend API is not responding"
    fi
    
    # Test DNS server
    if dig @localhost -p 5353 google.com +short > /dev/null; then
        echo "✅ DNS server is responding"
    else
        echo "❌ DNS server is not responding"
    fi
    
    # Test frontend
    if curl -f -s http://localhost:3000 > /dev/null; then
        echo "✅ Frontend is accessible"
    else
        echo "❌ Frontend is not accessible"
    fi
}

# Show access information
show_access_info() {
    echo ""
    echo "🎉 Bhai Ka DNS setup complete!"
    echo "============================="
    echo ""
    echo "🌐 Access Points:"
    echo "  Frontend:  http://localhost:3000"
    echo "  API:       http://localhost:8080"
    echo "  Grafana:   http://localhost:3001 (admin/admin)"
    echo "  DNS:       localhost:5353"
    echo ""
    echo "🔧 Useful Commands:"
    echo "  View logs:     docker-compose logs -f"
    echo "  Stop services: docker-compose down"
    echo "  Restart:       docker-compose restart"
    echo ""
    echo "📚 Documentation:"
    echo "  API Docs:      http://localhost:8080/docs"
    echo "  GitHub:        https://github.com/your-username/bhai-ka-dns"
    echo ""
    echo "🧪 Test DNS Resolution:"
    echo "  dig @localhost -p 5353 google.com"
    echo "  curl -X POST http://localhost:8080/api/dns/lookup -H 'Content-Type: application/json' -d '{\"domain\": \"example.com\"}'"
    echo ""
}

# Main execution
main() {
    check_prerequisites
    setup_environment
    load_datasets
    start_services
    verify_installation
    show_access_info
}

# Parse command line arguments
case "${1:-}" in
    "clean")
        echo "🧹 Cleaning up..."
        docker-compose down -v --remove-orphans
        docker system prune -f
        echo "✅ Cleanup complete!"
        ;;
    "logs")
        docker-compose logs -f
        ;;
    "restart")
        echo "🔄 Restarting services..."
        docker-compose restart
        echo "✅ Services restarted!"
        ;;
    "stop")
        echo "🛑 Stopping services..."
        docker-compose down
        echo "✅ Services stopped!"
        ;;
    *)
        main
        ;;
esac