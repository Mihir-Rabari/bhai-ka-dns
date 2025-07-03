#!/usr/bin/env python3
"""
Bhai Ka DNS - Web Interface and Landing Page
"""

from flask import Flask, render_template, request, jsonify, redirect, url_for
import dns.resolver
import json
import time
import re
import socket
from datetime import datetime
import requests
import threading
from dns_server import BhaiDNSServer, AIDNSResolver
import validators

app = Flask(__name__)

# Global DNS server instance
dns_server_instance = None
resolver_instance = AIDNSResolver()

@app.route('/')
def index():
    """Landing page for Bhai Ka DNS"""
    return render_template('index.html')

@app.route('/api/lookup', methods=['POST'])
def dns_lookup():
    """API endpoint for DNS lookups with AI features"""
    data = request.get_json()
    domain = data.get('domain', '').strip()
    
    if not domain:
        return jsonify({'error': 'Domain is required'}), 400
        
    if not validators.domain(domain):
        return jsonify({'error': 'Invalid domain format'}), 400
    
    try:
        # Perform DNS lookup
        result = {
            'domain': domain,
            'timestamp': datetime.now().isoformat(),
            'records': {},
            'ai_analysis': {}
        }
        
        # Get different record types
        record_types = ['A', 'AAAA', 'CNAME', 'MX', 'TXT', 'NS']
        
        for record_type in record_types:
            try:
                answers = dns.resolver.resolve(domain, record_type)
                result['records'][record_type] = [str(rdata) for rdata in answers]
            except:
                result['records'][record_type] = []
        
        # AI Analysis
        result['ai_analysis'] = analyze_domain_with_ai(domain)
        
        return jsonify(result)
        
    except Exception as e:
        return jsonify({'error': f'DNS lookup failed: {str(e)}'}), 500

@app.route('/api/analyze', methods=['POST'])
def analyze_domain():
    """AI-powered domain analysis"""
    data = request.get_json()
    domain = data.get('domain', '').strip()
    
    if not domain:
        return jsonify({'error': 'Domain is required'}), 400
    
    analysis = analyze_domain_with_ai(domain)
    return jsonify(analysis)

@app.route('/api/suggest', methods=['POST'])
def suggest_domains():
    """AI-powered domain suggestions"""
    data = request.get_json()
    input_domain = data.get('domain', '').strip()
    
    if not input_domain:
        return jsonify({'error': 'Domain input is required'}), 400
    
    suggestions = generate_domain_suggestions(input_domain)
    return jsonify({'suggestions': suggestions})

@app.route('/api/stats')
def get_stats():
    """Get DNS server statistics"""
    if resolver_instance:
        stats = resolver_instance.get_analytics()
        stats['uptime'] = time.time() - app.start_time if hasattr(app, 'start_time') else 0
        return jsonify(stats)
    return jsonify({'error': 'DNS server not running'})

@app.route('/api/health')
def health_check():
    """Health check endpoint"""
    return jsonify({
        'status': 'healthy',
        'service': 'Bhai Ka DNS',
        'timestamp': datetime.now().isoformat()
    })

def analyze_domain_with_ai(domain):
    """AI-powered domain analysis"""
    analysis = {
        'security_score': 0,
        'trust_level': 'unknown',
        'category': 'general',
        'flags': [],
        'recommendations': []
    }
    
    domain_lower = domain.lower()
    
    # Security analysis
    security_score = 100
    
    # Check for suspicious patterns
    if re.search(r'[0-9]{5,}', domain):
        security_score -= 30
        analysis['flags'].append('Contains many numbers')
        
    if any(keyword in domain_lower for keyword in ['secure', 'login', 'verify', 'update']):
        security_score -= 20
        analysis['flags'].append('Contains security-related keywords')
        
    if domain.count('-') > 3:
        security_score -= 15
        analysis['flags'].append('Many hyphens detected')
        
    if len(domain) > 30:
        security_score -= 10
        analysis['flags'].append('Unusually long domain')
        
    # TLD analysis
    suspicious_tlds = ['.tk', '.ml', '.cf', '.ga']
    if any(domain.endswith(tld) for tld in suspicious_tlds):
        security_score -= 40
        analysis['flags'].append('Suspicious TLD')
        
    # Category detection
    if any(keyword in domain_lower for keyword in ['bank', 'paypal', 'amazon', 'google']):
        analysis['category'] = 'financial/tech'
        if not domain.endswith('.com'):
            security_score -= 50
            analysis['flags'].append('Suspicious brand impersonation')
            
    # Set trust level
    if security_score >= 80:
        analysis['trust_level'] = 'high'
    elif security_score >= 60:
        analysis['trust_level'] = 'medium'
    elif security_score >= 40:
        analysis['trust_level'] = 'low'
    else:
        analysis['trust_level'] = 'very_low'
        
    analysis['security_score'] = max(0, security_score)
    
    # Generate recommendations
    if security_score < 70:
        analysis['recommendations'].append('Exercise caution when visiting this domain')
    if 'Many hyphens detected' in analysis['flags']:
        analysis['recommendations'].append('Verify the legitimacy of this domain')
    if analysis['trust_level'] == 'very_low':
        analysis['recommendations'].append('Consider blocking this domain')
        
    return analysis

def generate_domain_suggestions(input_domain):
    """Generate AI-powered domain suggestions"""
    suggestions = []
    
    # Remove common typos
    typo_fixes = {
        'gogle': 'google',
        'yahho': 'yahoo',
        'facbook': 'facebook',
        'twiter': 'twitter',
        'amazom': 'amazon'
    }
    
    for typo, correct in typo_fixes.items():
        if typo in input_domain.lower():
            fixed = input_domain.lower().replace(typo, correct)
            suggestions.append({
                'domain': fixed,
                'reason': f'Corrected typo: {typo} → {correct}',
                'confidence': 0.9
            })
    
    # Generate variations
    base_name = input_domain.split('.')[0]
    
    # Add common TLDs
    common_tlds = ['.com', '.net', '.org', '.io', '.co']
    for tld in common_tlds:
        if not input_domain.endswith(tld):
            suggestions.append({
                'domain': base_name + tld,
                'reason': f'Alternative TLD: {tld}',
                'confidence': 0.7
            })
    
    # Add prefixes/suffixes
    variations = [
        f'www.{input_domain}',
        f'my{base_name}.com',
        f'{base_name}app.com',
        f'{base_name}online.com'
    ]
    
    for variation in variations:
        suggestions.append({
            'domain': variation,
            'reason': 'Domain variation',
            'confidence': 0.6
        })
    
    return suggestions[:10]  # Return top 10 suggestions

def start_dns_server_background():
    """Start DNS server in background thread"""
    global dns_server_instance
    try:
        dns_server_instance = BhaiDNSServer(port=5353)
        dns_server_instance.start()
    except Exception as e:
        print(f"Failed to start DNS server: {e}")

if __name__ == '__main__':
    app.start_time = time.time()
    
    # Start DNS server in background
    dns_thread = threading.Thread(target=start_dns_server_background, daemon=True)
    dns_thread.start()
    
    print("🚀 Starting Bhai Ka DNS Web Interface...")
    print("🌐 Access the landing page at: http://localhost:8080")
    print("🔧 DNS Server running on port 5353")
    
    app.run(host='0.0.0.0', port=8080, debug=True)