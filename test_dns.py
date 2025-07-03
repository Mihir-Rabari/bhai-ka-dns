#!/usr/bin/env python3
"""
Test script for Bhai Ka DNS Server
"""

import requests
import time
import subprocess
import json

def test_web_interface():
    """Test the web interface endpoints"""
    base_url = "http://localhost:8080"
    
    try:
        # Test health endpoint
        response = requests.get(f"{base_url}/api/health", timeout=5)
        if response.status_code == 200:
            print("✅ Web interface health check: OK")
            print(f"   Response: {response.json()}")
        else:
            print("❌ Web interface health check: Failed")
        
        # Test stats endpoint
        response = requests.get(f"{base_url}/api/stats", timeout=5)
        if response.status_code == 200:
            print("✅ Stats endpoint: OK")
            print(f"   Stats: {response.json()}")
        else:
            print("❌ Stats endpoint: Failed")
        
        # Test DNS lookup endpoint
        test_data = {"domain": "google.com"}
        response = requests.post(f"{base_url}/api/lookup", 
                               json=test_data, 
                               headers={'Content-Type': 'application/json'},
                               timeout=10)
        if response.status_code == 200:
            print("✅ DNS lookup endpoint: OK")
            result = response.json()
            print(f"   Domain: {result.get('domain')}")
            print(f"   Security Score: {result.get('ai_analysis', {}).get('security_score', 'N/A')}")
        else:
            print("❌ DNS lookup endpoint: Failed")
            
        # Test domain suggestions endpoint
        test_data = {"domain": "gogle.com"}  # Typo
        response = requests.post(f"{base_url}/api/suggest", 
                               json=test_data, 
                               headers={'Content-Type': 'application/json'},
                               timeout=5)
        if response.status_code == 200:
            print("✅ Domain suggestions endpoint: OK")
            result = response.json()
            print(f"   Suggestions count: {len(result.get('suggestions', []))}")
        else:
            print("❌ Domain suggestions endpoint: Failed")
            
    except requests.exceptions.ConnectionError:
        print("❌ Could not connect to web interface (localhost:8080)")
        print("   Make sure the server is running")
    except Exception as e:
        print(f"❌ Web interface test failed: {e}")

def test_dns_server():
    """Test the DNS server functionality"""
    try:
        # Test using dig command if available
        result = subprocess.run([
            'dig', '@localhost', '-p', '5353', 'google.com', '+short'
        ], capture_output=True, text=True, timeout=10)
        
        if result.returncode == 0 and result.stdout.strip():
            print("✅ DNS server test with dig: OK")
            print(f"   Result: {result.stdout.strip()}")
        else:
            print("❌ DNS server test with dig: Failed")
            if result.stderr:
                print(f"   Error: {result.stderr}")
                
    except FileNotFoundError:
        print("⚠️  'dig' command not found, skipping DNS server test")
        print("   Install dnsutils package to test DNS functionality")
    except subprocess.TimeoutExpired:
        print("❌ DNS server test: Timeout")
    except Exception as e:
        print(f"❌ DNS server test failed: {e}")

def main():
    """Run all tests"""
    print("🚀 Testing Bhai Ka DNS Server\n")
    
    print("📋 Web Interface Tests:")
    test_web_interface()
    
    print("\n📋 DNS Server Tests:")
    test_dns_server()
    
    print("\n🎉 Test Summary:")
    print("   If all tests pass, your Bhai Ka DNS server is working correctly!")
    print("   Access the web interface at: http://localhost:8080")
    print("   Use DNS server with: dig @localhost -p 5353 example.com")

if __name__ == "__main__":
    main()