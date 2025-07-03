#!/usr/bin/env python3
"""
Bhai Ka DNS - Startup Script
Launches both DNS server and web interface simultaneously
"""

import subprocess
import sys
import time
import threading
import signal
import os
from colorama import init, Fore, Style

init(autoreset=True)

class BhaiDNSLauncher:
    def __init__(self):
        self.dns_process = None
        self.web_process = None
        self.running = True
        
    def print_banner(self):
        """Print the awesome Bhai Ka DNS banner"""
        banner = f"""
{Fore.MAGENTA}
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║  {Fore.CYAN}🚀 BHAI KA DNS - AI-POWERED DNS SERVER 🚀{Fore.MAGENTA}               ║
║                                                              ║
║  {Fore.GREEN}✨ Features:{Fore.MAGENTA}                                               ║
║     {Fore.YELLOW}• AI-Powered Threat Detection{Fore.MAGENTA}                         ║
║     {Fore.YELLOW}• Smart Caching & Performance{Fore.MAGENTA}                         ║
║     {Fore.YELLOW}• Typo Correction & Suggestions{Fore.MAGENTA}                       ║
║     {Fore.YELLOW}• Real-time Analytics Dashboard{Fore.MAGENTA}                       ║
║     {Fore.YELLOW}• Modern Web Interface{Fore.MAGENTA}                                ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
{Style.RESET_ALL}
        """
        print(banner)
        
    def check_dependencies(self):
        """Check if all required dependencies are installed"""
        required_modules = [
            'flask', 'dnslib', 'dnspython', 'requests', 
            'colorama', 'validators'
        ]
        
        missing_modules = []
        for module in required_modules:
            try:
                __import__(module)
            except ImportError:
                missing_modules.append(module)
                
        if missing_modules:
            print(f"{Fore.RED}❌ Missing required modules: {', '.join(missing_modules)}{Style.RESET_ALL}")
            print(f"{Fore.YELLOW}💡 Please install them with: pip install -r requirements.txt{Style.RESET_ALL}")
            return False
            
        print(f"{Fore.GREEN}✅ All dependencies are installed!{Style.RESET_ALL}")
        return True
        
    def start_dns_server(self):
        """Start the DNS server in a separate process"""
        try:
            print(f"{Fore.CYAN}🔧 Starting DNS Server on port 5353...{Style.RESET_ALL}")
            self.dns_process = subprocess.Popen([
                sys.executable, 'dns_server.py'
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            time.sleep(2)  # Give it time to start
            
            if self.dns_process.poll() is None:
                print(f"{Fore.GREEN}✅ DNS Server started successfully!{Style.RESET_ALL}")
                return True
            else:
                print(f"{Fore.RED}❌ DNS Server failed to start{Style.RESET_ALL}")
                return False
        except Exception as e:
            print(f"{Fore.RED}❌ Error starting DNS server: {e}{Style.RESET_ALL}")
            return False
            
    def start_web_interface(self):
        """Start the web interface in a separate process"""
        try:
            print(f"{Fore.CYAN}🌐 Starting Web Interface on port 8080...{Style.RESET_ALL}")
            self.web_process = subprocess.Popen([
                sys.executable, 'web_app.py'
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            time.sleep(3)  # Give it time to start
            
            if self.web_process.poll() is None:
                print(f"{Fore.GREEN}✅ Web Interface started successfully!{Style.RESET_ALL}")
                return True
            else:
                print(f"{Fore.RED}❌ Web Interface failed to start{Style.RESET_ALL}")
                return False
        except Exception as e:
            print(f"{Fore.RED}❌ Error starting web interface: {e}{Style.RESET_ALL}")
            return False
            
    def show_status(self):
        """Display current status and access information"""
        print(f"\n{Fore.GREEN}🎉 Bhai Ka DNS is now running!{Style.RESET_ALL}")
        print(f"\n{Fore.CYAN}📋 Service Status:{Style.RESET_ALL}")
        print(f"   {Fore.GREEN}✅ DNS Server:{Style.RESET_ALL} Running on port 5353")
        print(f"   {Fore.GREEN}✅ Web Interface:{Style.RESET_ALL} Running on port 8080")
        
        print(f"\n{Fore.YELLOW}🌐 Access Points:{Style.RESET_ALL}")
        print(f"   {Fore.CYAN}Landing Page:{Style.RESET_ALL} http://localhost:8080")
        print(f"   {Fore.CYAN}DNS Server:{Style.RESET_ALL} Use dig @localhost -p 5353 example.com")
        
        print(f"\n{Fore.YELLOW}🔧 Test Commands:{Style.RESET_ALL}")
        print(f"   {Fore.WHITE}dig @localhost -p 5353 google.com{Style.RESET_ALL}")
        print(f"   {Fore.WHITE}curl http://localhost:8080/api/health{Style.RESET_ALL}")
        
        print(f"\n{Fore.MAGENTA}💡 Features to try:{Style.RESET_ALL}")
        print(f"   • Visit the web interface for DNS lookup tools")
        print(f"   • Try the AI-powered domain suggestions")
        print(f"   • Check the real-time analytics dashboard")
        print(f"   • Test typo correction (try 'gogle.com')")
        
        print(f"\n{Fore.RED}🛑 Press Ctrl+C to stop all services{Style.RESET_ALL}")
        
    def signal_handler(self, signum, frame):
        """Handle shutdown signals"""
        print(f"\n{Fore.YELLOW}🛑 Shutting down Bhai Ka DNS...{Style.RESET_ALL}")
        self.running = False
        self.cleanup()
        
    def cleanup(self):
        """Clean up processes and exit"""
        if self.dns_process:
            print(f"{Fore.YELLOW}🔧 Stopping DNS Server...{Style.RESET_ALL}")
            self.dns_process.terminate()
            try:
                self.dns_process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.dns_process.kill()
                
        if self.web_process:
            print(f"{Fore.YELLOW}🌐 Stopping Web Interface...{Style.RESET_ALL}")
            self.web_process.terminate()
            try:
                self.web_process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.web_process.kill()
                
        print(f"{Fore.GREEN}✅ Bhai Ka DNS stopped successfully!{Style.RESET_ALL}")
        print(f"{Fore.MAGENTA}👋 Thanks for using Bhai Ka DNS!{Style.RESET_ALL}")
        
    def monitor_processes(self):
        """Monitor the health of both processes"""
        while self.running:
            time.sleep(5)
            
            # Check DNS server
            if self.dns_process and self.dns_process.poll() is not None:
                print(f"{Fore.RED}⚠️  DNS Server stopped unexpectedly!{Style.RESET_ALL}")
                self.running = False
                break
                
            # Check web interface
            if self.web_process and self.web_process.poll() is not None:
                print(f"{Fore.RED}⚠️  Web Interface stopped unexpectedly!{Style.RESET_ALL}")
                self.running = False
                break
                
    def run(self):
        """Main run method"""
        # Set up signal handlers
        signal.signal(signal.SIGINT, self.signal_handler)
        signal.signal(signal.SIGTERM, self.signal_handler)
        
        self.print_banner()
        
        # Check dependencies
        if not self.check_dependencies():
            return 1
            
        # Start services
        if not self.start_dns_server():
            return 1
            
        if not self.start_web_interface():
            self.cleanup()
            return 1
            
        # Show status
        self.show_status()
        
        # Monitor processes
        monitor_thread = threading.Thread(target=self.monitor_processes, daemon=True)
        monitor_thread.start()
        
        # Keep running until interrupted
        try:
            while self.running:
                time.sleep(1)
        except KeyboardInterrupt:
            pass
        finally:
            self.cleanup()
            
        return 0

def main():
    """Main entry point"""
    launcher = BhaiDNSLauncher()
    sys.exit(launcher.run())

if __name__ == "__main__":
    main()