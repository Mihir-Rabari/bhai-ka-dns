#!/bin/bash

# Bhai Ka DNS - Dataset Loader
# This script downloads and prepares datasets for enhanced DNS functionality

set -e

echo "📊 Loading datasets for Bhai Ka DNS"
echo "===================================="

# Create data directory
mkdir -p data/{threats,typos,popular,geo}

# Function to download with progress
download_file() {
    local url=$1
    local output=$2
    local description=$3
    
    echo "📥 Downloading $description..."
    if command -v wget >/dev/null 2>&1; then
        wget -q --show-progress -O "$output" "$url"
    elif command -v curl >/dev/null 2>&1; then
        curl -L --progress-bar -o "$output" "$url"
    else
        echo "❌ Neither wget nor curl found. Please install one of them."
        exit 1
    fi
    echo "✅ Downloaded $description"
}

# Load threat intelligence feeds
load_threat_intelligence() {
    echo "🛡️ Loading threat intelligence..."
    
    # Malware domains
    download_file \
        "https://malware-filter.gitlab.io/malware-filter/urlhaus-filter-hosts.txt" \
        "data/threats/malware-domains.txt" \
        "Malware domains list"
    
    # Phishing domains
    download_file \
        "https://raw.githubusercontent.com/mitchellkrogza/Phishing.Database/master/phishing-domains/output/domains/ACTIVE/list" \
        "data/threats/phishing-domains.txt" \
        "Phishing domains list"
    
    # Ad/tracker domains
    download_file \
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts" \
        "data/threats/ad-tracker-domains.txt" \
        "Ad and tracker domains list"
    
    # Ransomware domains
    download_file \
        "https://raw.githubusercontent.com/Hestat/dhp/master/block.txt" \
        "data/threats/ransomware-domains.txt" \
        "Ransomware domains list"
    
    # Create consolidated threat list
    echo "🔗 Creating consolidated threat list..."
    cat data/threats/*.txt | grep -E '^[^#].*\.[a-z]{2,}$' | sort -u > data/threats/consolidated-threats.txt
    echo "✅ Consolidated $(wc -l < data/threats/consolidated-threats.txt) threat domains"
}

# Load popular domains for caching optimization
load_popular_domains() {
    echo "🌟 Loading popular domains for cache optimization..."
    
    # Alexa/Tranco top domains
    download_file \
        "https://tranco-list.eu/download/GN9Z4/full" \
        "data/popular/tranco-top-domains.csv" \
        "Tranco top domains list"
    
    # Extract just the domains (remove ranking)
    if [ -f "data/popular/tranco-top-domains.csv" ]; then
        cut -d',' -f2 data/popular/tranco-top-domains.csv | head -10000 > data/popular/top-domains.txt
        echo "✅ Loaded $(wc -l < data/popular/top-domains.txt) popular domains"
    fi
}

# Load common typo patterns
load_typo_patterns() {
    echo "✏️ Creating typo correction patterns..."
    
    cat > data/typos/common-typos.json << 'EOF'
{
  "google.com": ["gogle.com", "googel.com", "goggle.com", "gooogle.com"],
  "facebook.com": ["facbook.com", "facebok.com", "facebook.co", "facbeook.com"],
  "youtube.com": ["youtub.com", "youtubee.com", "youube.com", "yotube.com"],
  "amazon.com": ["amazom.com", "amzon.com", "amazon.co", "ammazon.com"],
  "twitter.com": ["twiter.com", "twittr.com", "twiter.co", "twitter.co"],
  "instagram.com": ["instgram.com", "instragram.com", "instagra.com", "instagram.co"],
  "linkedin.com": ["linkedn.com", "linkdin.com", "linkedin.co", "linkeind.com"],
  "github.com": ["gihub.com", "github.co", "gitub.com", "gihtub.com"],
  "stackoverflow.com": ["stackoveflow.com", "stackoverflow.co", "stakoverflow.com"],
  "reddit.com": ["redit.com", "reddit.co", "redditt.com", "reditt.com"],
  "wikipedia.org": ["wikipeda.org", "wikipedia.com", "wikpedia.org", "wikeipedia.org"],
  "apple.com": ["aple.com", "apple.co", "appel.com", "aplle.com"],
  "microsoft.com": ["micrsoft.com", "microsoft.co", "microsft.com", "mircosoft.com"],
  "netflix.com": ["netfix.com", "netflix.co", "netflex.com", "netfilx.com"],
  "paypal.com": ["payp4l.com", "paypal.co", "paypall.com", "paypaI.com"],
  "ebay.com": ["ebay.co", "eaby.com", "ebay.cm", "ebayy.com"]
}
EOF
    echo "✅ Created typo patterns for popular domains"
}

# Load GeoIP database (optional)
load_geoip_data() {
    echo "🌍 Loading GeoIP database..."
    
    # Check if MaxMind license key is available
    if [ -n "${MAXMIND_LICENSE_KEY:-}" ]; then
        download_file \
            "https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City&license_key=${MAXMIND_LICENSE_KEY}&suffix=tar.gz" \
            "data/geo/GeoLite2-City.tar.gz" \
            "GeoLite2 City database"
        
        # Extract the database
        cd data/geo
        tar -xzf GeoLite2-City.tar.gz --strip-components=1
        rm GeoLite2-City.tar.gz
        cd ../..
        echo "✅ GeoIP database loaded"
    else
        echo "⚠️ MAXMIND_LICENSE_KEY not set, skipping GeoIP database"
        echo "   You can get a free license key from https://www.maxmind.com/en/geolite2/signup"
    fi
}

# Load DNS blocklists for different categories
load_dns_blocklists() {
    echo "🚫 Loading DNS blocklists..."
    
    # Create category-specific blocklists
    mkdir -p data/blocklists/{ads,malware,phishing,trackers,social,porn}
    
    # Ad blockers
    download_file \
        "https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/master/BaseFilter/sections/adservers.txt" \
        "data/blocklists/ads/adguard-ads.txt" \
        "AdGuard ad servers list"
    
    # Social media blockers
    download_file \
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/social/hosts" \
        "data/blocklists/social/social-networks.txt" \
        "Social media blocklist"
    
    # Porn/adult content blockers
    download_file \
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn/hosts" \
        "data/blocklists/porn/adult-content.txt" \
        "Adult content blocklist"
    
    echo "✅ DNS blocklists loaded"
}

# Create sample configuration for AI models
create_ai_config() {
    echo "🧠 Creating AI model configuration..."
    
    cat > data/ai-config.json << 'EOF'
{
  "threat_detection": {
    "models": [
      {
        "name": "domain_classifier",
        "type": "ml",
        "confidence_threshold": 0.7,
        "features": ["domain_length", "entropy", "tld", "subdomain_count", "special_chars"]
      }
    ],
    "rules": [
      {
        "pattern": ".*-secure-.*\\.com$",
        "threat_type": "phishing",
        "confidence": 0.9
      },
      {
        "pattern": ".*paypal.*\\.(tk|ml|cf|ga)$",
        "threat_type": "phishing",
        "confidence": 0.95
      }
    ]
  },
  "typo_correction": {
    "algorithms": ["levenshtein", "jaro_winkler", "soundex"],
    "max_distance": 2,
    "min_confidence": 0.8
  },
  "domain_analysis": {
    "features": {
      "reputation": true,
      "age": true,
      "ssl_cert": true,
      "registration_info": true,
      "dns_records": true
    }
  }
}
EOF
    echo "✅ AI configuration created"
}

# Create initialization SQL for the database
create_db_init() {
    echo "💾 Creating database initialization scripts..."
    
    cat > data/init-db.js << 'EOF'
// MongoDB initialization script for Bhai Ka DNS
db = db.getSiblingDB('bhai_dns');

// Create collections with indexes
db.createCollection('dns_queries');
db.dns_queries.createIndex({ domain: 1, timestamp: -1 });
db.dns_queries.createIndex({ timestamp: -1 });
db.dns_queries.createIndex({ client_ip: 1 });
db.dns_queries.createIndex({ threat_detected: 1 });

db.createCollection('analytics');
db.analytics.createIndex({ date: -1 });
db.analytics.createIndex({ metric_type: 1, date: -1 });

db.createCollection('threat_intelligence');
db.threat_intelligence.createIndex({ domain: 1 });
db.threat_intelligence.createIndex({ threat_type: 1 });
db.threat_intelligence.createIndex({ updated_at: -1 });

db.createCollection('users');
db.users.createIndex({ email: 1 }, { unique: true });
db.users.createIndex({ username: 1 }, { unique: true });

// Insert initial data
print('Database initialized successfully');
EOF
    echo "✅ Database initialization script created"
}

# Create sample threat intelligence data
create_sample_data() {
    echo "📝 Creating sample data..."
    
    # Create sample threat domains
    cat > data/threats/sample-threats.json << 'EOF'
[
  {
    "domain": "malware-example.com",
    "threat_type": "malware",
    "confidence": 0.95,
    "source": "sample_data",
    "description": "Known malware distribution site"
  },
  {
    "domain": "phishing-bank.net",
    "threat_type": "phishing",
    "confidence": 0.98,
    "source": "sample_data",
    "description": "Phishing site targeting banking credentials"
  },
  {
    "domain": "fake-paypal.tk",
    "threat_type": "phishing",
    "confidence": 0.99,
    "source": "sample_data",
    "description": "PayPal phishing site"
  }
]
EOF
    
    # Create sample popular domains for testing
    cat > data/popular/sample-popular.txt << 'EOF'
google.com
youtube.com
facebook.com
amazon.com
wikipedia.org
reddit.com
twitter.com
instagram.com
linkedin.com
github.com
stackoverflow.com
apple.com
microsoft.com
netflix.com
zoom.us
EOF
    
    echo "✅ Sample data created"
}

# Main execution
main() {
    echo "Starting dataset loading process..."
    
    load_threat_intelligence
    load_popular_domains
    load_typo_patterns
    load_geoip_data
    load_dns_blocklists
    create_ai_config
    create_db_init
    create_sample_data
    
    echo ""
    echo "🎉 Dataset loading complete!"
    echo "=========================="
    echo ""
    echo "📊 Data Summary:"
    echo "  Threat domains: $(wc -l < data/threats/consolidated-threats.txt 2>/dev/null || echo 'Loading failed')"
    echo "  Popular domains: $(wc -l < data/popular/top-domains.txt 2>/dev/null || echo 'Loading failed')"
    echo "  Typo patterns: $(jq 'keys | length' data/typos/common-typos.json 2>/dev/null || echo 'Not available')"
    echo ""
    echo "🔧 Next Steps:"
    echo "  1. Copy data/init-db.js to your MongoDB container"
    echo "  2. Set MAXMIND_LICENSE_KEY for GeoIP features"
    echo "  3. Configure threat feeds in your .env file"
    echo "  4. Run the DNS server with: cargo run"
    echo ""
}

# Parse command line arguments
case "${1:-}" in
    "threats")
        load_threat_intelligence
        ;;
    "popular")
        load_popular_domains
        ;;
    "typos")
        load_typo_patterns
        ;;
    "geoip")
        load_geoip_data
        ;;
    "blocklists")
        load_dns_blocklists
        ;;
    "clean")
        echo "🧹 Cleaning data directory..."
        rm -rf data/
        echo "✅ Data directory cleaned"
        ;;
    *)
        main
        ;;
esac