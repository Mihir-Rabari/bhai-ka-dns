# 🚀 Bhai Ka DNS - AI-Powered DNS Server

An intelligent DNS server with AI-powered features including threat detection, smart caching, typo correction, and real-time analytics.

![Bhai Ka DNS](https://img.shields.io/badge/Bhai%20Ka%20DNS-AI%20Powered-purple)
![Python](https://img.shields.io/badge/Python-3.8+-blue)
![Flask](https://img.shields.io/badge/Flask-Web%20Interface-green)
![DNS](https://img.shields.io/badge/DNS-Server-orange)

## ✨ Features

### 🧠 AI-Powered Capabilities
- **Smart Threat Detection**: AI algorithms detect malicious domains and phishing attempts
- **Intelligent Caching**: Optimized DNS caching with AI-driven TTL management
- **Typo Correction**: Automatic detection and correction of common domain misspellings
- **Domain Analysis**: Real-time security scoring and trust level assessment
- **Smart Suggestions**: AI-powered domain name suggestions and alternatives

### 🛡️ Security Features
- Real-time threat blocking
- Pattern-based malware detection
- Suspicious TLD filtering
- Brand impersonation protection
- Configurable blacklists

### 📊 Analytics & Monitoring
- Live DNS query statistics
- Threat detection metrics
- Cache performance monitoring
- AI suggestion tracking
- Real-time dashboard

### 🌐 Web Interface
- Modern, responsive landing page
- Interactive DNS lookup tools
- Domain analysis dashboard
- Real-time analytics display
- Mobile-friendly design

## 🔧 Installation

### Prerequisites
- Python 3.8 or higher
- pip package manager
- Network access for DNS resolution

### Quick Start

1. **Clone or download the project files**
2. **Install dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

3. **Start the DNS server:**
   ```bash
   python dns_server.py
   ```

4. **Start the web interface (in a new terminal):**
   ```bash
   python web_app.py
   ```

## 🚀 Usage

### DNS Server
The DNS server runs on port `5353` by default. You can use it with any DNS client:

```bash
# Using dig
dig @localhost -p 5353 google.com

# Using nslookup
nslookup google.com localhost:5353

# Using Python
import dns.resolver
resolver = dns.resolver.Resolver()
resolver.nameservers = ['127.0.0.1']
resolver.port = 5353
result = resolver.resolve('google.com', 'A')
```

### Web Interface
Access the landing page at: **http://localhost:8080**

#### Features Available:
1. **DNS Lookup Tool**: Enter any domain to get DNS records + AI security analysis
2. **Domain Suggestions**: Get AI-powered domain name suggestions
3. **Live Analytics**: Monitor server performance and statistics
4. **Interactive Dashboard**: Real-time threat detection and caching metrics

## 🧠 AI Features Explained

### 1. Threat Detection
The AI analyzes domains using multiple heuristics:
- **Pattern Recognition**: Detects suspicious domain patterns
- **Blacklist Matching**: Checks against known malicious domains
- **TLD Analysis**: Flags suspicious top-level domains
- **Brand Protection**: Identifies potential impersonation attempts

### 2. Smart Caching
- **Adaptive TTL**: AI adjusts cache timing based on domain popularity
- **Predictive Preloading**: Anticipates frequently requested domains
- **Memory Optimization**: Intelligent cache eviction strategies

### 3. Typo Correction
- **Common Mistakes**: Automatically fixes frequent typos (e.g., "gogle.com" → "google.com")
- **Similarity Matching**: Suggests correct domains for misspelled queries
- **Learning Algorithm**: Adapts to new typo patterns over time

### 4. Domain Analysis
Each domain gets a comprehensive AI analysis:
- **Security Score**: 0-100 rating based on multiple factors
- **Trust Level**: High/Medium/Low/Very Low classification
- **Category Detection**: Identifies domain type (financial, tech, etc.)
- **Risk Flags**: Specific warnings about potential threats

## 📊 API Endpoints

### DNS Lookup with AI Analysis
```http
POST /api/lookup
Content-Type: application/json

{
  "domain": "example.com"
}
```

### Domain Suggestions
```http
POST /api/suggest
Content-Type: application/json

{
  "domain": "exampl.com"
}
```

### Server Statistics
```http
GET /api/stats
```

### Health Check
```http
GET /api/health
```

## ⚙️ Configuration

### DNS Server Configuration
Edit `dns_server.py` to customize:
- Port number (default: 5353)
- Host address (default: 0.0.0.0)
- Cache TTL settings
- Threat detection patterns
- Upstream DNS servers

### Web Interface Configuration
Edit `web_app.py` to customize:
- Web server port (default: 8080)
- API endpoints
- Analytics refresh rate
- UI customizations

## 🔒 Security Considerations

1. **Run with appropriate privileges**: DNS servers typically need elevated permissions
2. **Network security**: Consider firewall rules for DNS and web ports
3. **Rate limiting**: Implement rate limiting for production use
4. **Logging**: Enable comprehensive logging for security monitoring
5. **Updates**: Regularly update threat intelligence databases

## 🎯 Use Cases

### Home/Small Office
- **Ad Blocking**: Block advertising and tracking domains
- **Parental Controls**: Filter inappropriate content
- **Performance**: Speed up browsing with intelligent caching

### Enterprise
- **Security**: Detect and block malicious domains
- **Compliance**: Monitor and log DNS requests
- **Performance**: Optimize network performance with smart caching

### Development
- **Testing**: Custom DNS responses for development environments
- **Debugging**: Detailed DNS query analysis and logging
- **Integration**: API-based DNS management

## 🔧 Advanced Usage

### Custom Threat Intelligence
Add your own threat domains in `dns_server.py`:
```python
def load_threat_intelligence(self):
    custom_threats = ['badsite.com', 'malware.net']
    self.threat_domains.update(custom_threats)
```

### Custom Typo Patterns
Extend typo correction in `web_app.py`:
```python
typo_fixes = {
    'your_typo': 'correct_domain',
    # Add more patterns
}
```

### Performance Tuning
Optimize for your environment:
- Adjust cache sizes
- Tune AI thresholds
- Configure upstream servers
- Set appropriate timeouts

## 📈 Monitoring & Metrics

The server provides comprehensive metrics:
- **Total Queries**: Number of DNS requests processed
- **Blocked Threats**: Malicious domains blocked
- **Cache Hits**: Successful cache retrievals
- **AI Suggestions**: Typo corrections made
- **Response Times**: Query processing speed
- **Error Rates**: Failed resolutions

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new features
5. Submit a pull request

## 📝 License

This project is open source. Feel free to use, modify, and distribute according to your needs.

## 🆘 Troubleshooting

### Common Issues

**DNS server won't start:**
- Check if port 5353 is available
- Ensure proper permissions
- Verify network connectivity

**Web interface not accessible:**
- Confirm Flask is running on port 8080
- Check firewall settings
- Verify all dependencies are installed

**DNS queries timing out:**
- Check upstream DNS configuration
- Verify network connectivity
- Review server logs

**AI features not working:**
- Ensure all Python dependencies are installed
- Check for proper DNS resolution
- Review analytics for errors

### Debug Mode
Enable debug mode for detailed logging:
```python
# In dns_server.py
logging.basicConfig(level=logging.DEBUG)

# In web_app.py  
app.run(debug=True)
```

## 🌟 Future Enhancements

- **Machine Learning**: Enhanced AI models for threat detection
- **Clustering**: Multi-server DNS cluster support
- **APIs**: Extended REST API functionality
- **Plugins**: Modular plugin architecture
- **GUI**: Desktop application interface
- **Cloud**: Cloud deployment templates

---

**Made with ❤️ and AI** | **Bhai Ka DNS** - Your intelligent DNS companion!
