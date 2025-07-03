# 🚀 Bhai Ka DNS - Quick Start Guide

Get up and running with Bhai Ka DNS in minutes!

## 🏃‍♂️ Super Quick Start (Docker Compose)

```bash
# Clone the repository
git clone https://github.com/your-username/bhai-ka-dns.git
cd bhai-ka-dns

# Run the setup script
chmod +x scripts/setup.sh
./scripts/setup.sh

# That's it! 🎉
```

**Access Points:**
- 🌐 **Frontend**: http://localhost:3000
- 🔧 **API**: http://localhost:8080  
- 📊 **Grafana**: http://localhost:3001 (admin/admin)
- 🔍 **DNS Server**: localhost:5353

## 🧪 Quick Test

```bash
# Test DNS resolution
dig @localhost -p 5353 google.com

# Test API
curl http://localhost:8080/health

# Test domain analysis
curl -X POST http://localhost:8080/api/dns/analyze \
  -H "Content-Type: application/json" \
  -d '{"domain": "example.com"}'
```

## ☸️ Kubernetes Deployment

```bash
# Deploy to Kubernetes
chmod +x scripts/k8s-deploy.sh
./scripts/k8s-deploy.sh

# Access via port forwarding
kubectl port-forward -n bhai-dns svc/bhai-dns-backend-api 8080:8080
```

## 🛠️ Manual Development Setup

### Backend (Rust)
```bash
# Install Rust dependencies
cargo build --release

# Start MongoDB
docker run -d -p 27017:27017 mongo:7.0

# Run the backend
cargo run
```

### Frontend (React)
```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

## 📱 What You Get

### 🎨 Beautiful Frontend
- Modern React app with shadcn/ui components
- Smooth animations with Framer Motion
- Real-time DNS lookup tool
- Interactive analytics dashboard
- Dark/light theme support

### 🦀 Powerful Backend
- High-performance Rust DNS server
- AI-powered threat detection
- Intelligent caching system
- Real-time analytics
- RESTful API

### 🧠 AI Features
- **Threat Detection**: Blocks malicious domains automatically
- **Typo Correction**: Suggests corrections for misspelled domains
- **Security Scoring**: Analyzes domain safety in real-time
- **Smart Caching**: AI-optimized cache management

### 📊 Monitoring Stack
- **Prometheus**: Metrics collection
- **Grafana**: Beautiful dashboards
- **Real-time Stats**: Live performance monitoring
- **Historical Analysis**: Trend tracking

## 🎯 Common Use Cases

### 🏠 Home Network
```bash
# Block ads and trackers
echo "nameserver 127.0.0.1" > /etc/resolv.conf
echo "port 5353" >> /etc/resolv.conf
```

### 🏢 Enterprise
```bash
# Monitor DNS queries
curl http://localhost:8080/api/analytics/dashboard

# Add custom threat domains
curl -X POST http://localhost:8080/api/admin/threats \
  -H "Content-Type: application/json" \
  -d '{"domain": "malicious-site.com", "threat_type": "malware"}'
```

### 👩‍💻 Development
```bash
# Custom DNS responses for testing
curl -X POST http://localhost:8080/api/dns/lookup \
  -H "Content-Type: application/json" \
  -d '{"domain": "test.local"}'
```

## 🔧 Configuration

### Environment Variables
```bash
# Database
export DATABASE_URI="mongodb://localhost:27017/bhai_dns"

# Features
export ENABLE_AI_FEATURES=true
export ENABLE_ANALYTICS=true

# Security
export JWT_SECRET="your-secret-key"
```

### Config File (config.toml)
```toml
[dns]
port = 5353
upstream_servers = ["8.8.8.8:53", "1.1.1.1:53"]
enable_ai_features = true

[web]
port = 8080
cors_origins = ["*"]

[ai]
threat_detection = true
typo_correction = true
domain_analysis = true
```

## 🚨 Troubleshooting

### Service Won't Start
```bash
# Check logs
docker-compose logs -f

# Check port availability
lsof -i :5353
lsof -i :8080
```

### DNS Not Resolving
```bash
# Test direct connection
dig @127.0.0.1 -p 5353 google.com

# Check upstream servers
curl http://localhost:8080/api/analytics/stats
```

### Frontend Not Loading
```bash
# Check if backend is running
curl http://localhost:8080/health

# Restart frontend
docker-compose restart bhai-dns-frontend
```

## 🎮 Interactive Demo

Try these commands to see Bhai Ka DNS in action:

```bash
# 1. Lookup a domain with AI analysis
curl -X POST http://localhost:8080/api/dns/lookup \
  -H "Content-Type: application/json" \
  -d '{"domain": "github.com"}' | jq

# 2. Test threat detection
curl -X POST http://localhost:8080/api/dns/analyze \
  -H "Content-Type: application/json" \
  -d '{"domain": "malware.com"}' | jq

# 3. Get typo suggestions
curl -X POST http://localhost:8080/api/dns/suggest \
  -H "Content-Type: application/json" \
  -d '{"domain": "gogle.com"}' | jq

# 4. View real-time stats
curl http://localhost:8080/api/analytics/stats | jq
```

## 📚 Next Steps

1. **Explore the Frontend**: Visit http://localhost:3000 and try the interactive tools
2. **Check Analytics**: View Grafana dashboards at http://localhost:3001
3. **Read the Docs**: Check out the full [README.md](README.md) for detailed information
4. **Customize**: Modify `config.toml` for your specific needs
5. **Deploy**: Use the Kubernetes scripts for production deployment

## 🤝 Need Help?

- 📖 **Full Documentation**: [README.md](README.md)
- 🐛 **Issues**: [GitHub Issues](https://github.com/your-username/bhai-ka-dns/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/your-username/bhai-ka-dns/discussions)
- 🚀 **Examples**: Check the `examples/` directory

---

**Happy DNS resolving!** 🎉