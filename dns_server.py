#!/usr/bin/env python3
"""
Bhai Ka DNS - A Smart AI-Powered DNS Server
"""

import socket
import threading
import time
import json
import hashlib
import re
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Tuple
import dns.resolver
import dns.message
import dns.rdatatype
from dnslib import DNSRecord, DNSHeader, RR, A, AAAA, CNAME, MX, TXT, QTYPE
from dnslib.server import DNSServer, DNSHandler, BaseResolver
import requests
from colorama import init, Fore, Style

init(autoreset=True)

class AIDNSResolver(BaseResolver):
    def __init__(self):
        self.cache = {}
        self.threat_domains = set()
        self.analytics = {
            'total_queries': 0,
            'blocked_threats': 0,
            'ai_suggestions': 0,
            'cache_hits': 0
        }
        self.load_threat_intelligence()
        
    def load_threat_intelligence(self):
        """Load known malicious domains from threat intelligence"""
        known_threats = [
            'malware.com', 'phishing.net', 'spam.org',
            'trojan.info', 'virus.biz', 'scam.co'
        ]
        self.threat_domains.update(known_threats)
        
    def is_threat_domain(self, domain: str) -> bool:
        """AI-powered threat detection"""
        domain_lower = domain.lower()
        
        # Check against known threats
        if domain_lower in self.threat_domains:
            return True
            
        # AI heuristics for threat detection
        threat_patterns = [
            r'.*-secure-.*\.com$',  # Fake security domains
            r'.*paypal.*\.tk$',      # Suspicious TLD with paypal
            r'.*bank.*\.ml$',        # Suspicious banking domains
            r'.*[0-9]{5,}.*\.com$',  # Domains with many numbers
        ]
        
        for pattern in threat_patterns:
            if re.match(pattern, domain_lower):
                return True
                
        return False
        
    def get_ai_suggestion(self, domain: str) -> Optional[str]:
        """AI-powered domain suggestions for typos"""
        common_typos = {
            'gogle.com': 'google.com',
            'yahho.com': 'yahoo.com',
            'facbook.com': 'facebook.com',
            'twiter.com': 'twitter.com',
            'amazom.com': 'amazon.com',
        }
        
        if domain.lower() in common_typos:
            self.analytics['ai_suggestions'] += 1
            return common_typos[domain.lower()]
            
        return None
        
    def smart_cache_key(self, qname: str, qtype: int) -> str:
        """Generate intelligent cache key"""
        return f"{qname}:{qtype}"
        
    def resolve(self, request, handler):
        """Main DNS resolution logic with AI features"""
        reply = request.reply()
        qname = str(request.q.qname).rstrip('.')
        qtype = request.q.qtype
        
        self.analytics['total_queries'] += 1
        
        print(f"{Fore.CYAN}[BHAI DNS] Query: {qname} ({QTYPE[qtype]}){Style.RESET_ALL}")
        
        # Threat detection
        if self.is_threat_domain(qname):
            print(f"{Fore.RED}[THREAT BLOCKED] {qname}{Style.RESET_ALL}")
            self.analytics['blocked_threats'] += 1
            # Return NXDOMAIN for threats
            reply.header.rcode = 3
            return reply
            
        # Check cache first
        cache_key = self.smart_cache_key(qname, qtype)
        if cache_key in self.cache:
            cached_entry = self.cache[cache_key]
            if datetime.now() < cached_entry['expires']:
                print(f"{Fore.GREEN}[CACHE HIT] {qname}{Style.RESET_ALL}")
                self.analytics['cache_hits'] += 1
                reply.add_answer(*cached_entry['records'])
                return reply
                
        # AI suggestion for typos
        suggestion = self.get_ai_suggestion(qname)
        if suggestion:
            print(f"{Fore.YELLOW}[AI SUGGESTION] {qname} -> {suggestion}{Style.RESET_ALL}")
            qname = suggestion
            
        try:
            # Forward to upstream DNS
            upstream_response = dns.resolver.resolve(qname, QTYPE[qtype])
            
            records = []
            for rdata in upstream_response:
                if qtype == QTYPE.A:
                    rr = RR(qname, QTYPE.A, rdata=A(str(rdata)))
                elif qtype == QTYPE.AAAA:
                    rr = RR(qname, QTYPE.AAAA, rdata=AAAA(str(rdata)))
                elif qtype == QTYPE.CNAME:
                    rr = RR(qname, QTYPE.CNAME, rdata=CNAME(str(rdata)))
                else:
                    continue
                    
                records.append(rr)
                reply.add_answer(rr)
                
            # Cache the response
            self.cache[cache_key] = {
                'records': records,
                'expires': datetime.now() + timedelta(minutes=5)
            }
            
            print(f"{Fore.GREEN}[RESOLVED] {qname} -> {len(records)} records{Style.RESET_ALL}")
            
        except Exception as e:
            print(f"{Fore.RED}[ERROR] Failed to resolve {qname}: {e}{Style.RESET_ALL}")
            reply.header.rcode = 3  # NXDOMAIN
            
        return reply
        
    def get_analytics(self) -> Dict:
        """Get DNS analytics"""
        return self.analytics.copy()

class BhaiDNSServer:
    def __init__(self, host='0.0.0.0', port=5353):
        self.host = host
        self.port = port
        self.resolver = AIDNSResolver()
        self.server = None
        
    def start(self):
        """Start the DNS server"""
        print(f"{Fore.MAGENTA}🚀 Starting Bhai Ka DNS Server on {self.host}:{self.port}{Style.RESET_ALL}")
        print(f"{Fore.MAGENTA}💡 AI Features: Threat Detection, Smart Caching, Typo Correction{Style.RESET_ALL}")
        
        self.server = DNSServer(
            self.resolver,
            port=self.port,
            address=self.host,
            tcp=False
        )
        
        try:
            self.server.start_thread()
            print(f"{Fore.GREEN}✅ Bhai Ka DNS is running! Use: dig @{self.host} -p {self.port} example.com{Style.RESET_ALL}")
            
            # Keep the server running
            while True:
                time.sleep(1)
                
        except KeyboardInterrupt:
            print(f"\n{Fore.YELLOW}🛑 Shutting down Bhai Ka DNS...{Style.RESET_ALL}")
            self.stop()
            
    def stop(self):
        """Stop the DNS server"""
        if self.server:
            self.server.stop()
            
    def get_stats(self):
        """Get server statistics"""
        return self.resolver.get_analytics()

if __name__ == "__main__":
    server = BhaiDNSServer()
    server.start()