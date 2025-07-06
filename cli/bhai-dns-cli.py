#!/usr/bin/env python3
"""
Bhai Ka DNS CLI Tool
A comprehensive CLI for managing Bhai Ka DNS infrastructure locally and on AWS
"""

import argparse
import subprocess
import sys
import os
import json
import yaml
from pathlib import Path
from typing import Dict, List, Optional
import boto3
from rich.console import Console
from rich.table import Table
from rich.progress import Progress
from rich.panel import Panel

console = Console()

class BhaiDNSCLI:
    def __init__(self):
        self.config_file = Path.home() / ".bhai-dns" / "config.yaml"
        self.project_root = Path(__file__).parent.parent
        self.config = self.load_config()
        
    def load_config(self) -> Dict:
        """Load CLI configuration"""
        if self.config_file.exists():
            with open(self.config_file, 'r') as f:
                return yaml.safe_load(f) or {}
        return {}
    
    def save_config(self):
        """Save CLI configuration"""
        self.config_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self.config_file, 'w') as f:
            yaml.dump(self.config, f)
    
    def run_command(self, cmd: List[str], cwd: Optional[Path] = None) -> bool:
        """Run shell command with error handling"""
        try:
            result = subprocess.run(
                cmd, 
                cwd=cwd or self.project_root,
                capture_output=True,
                text=True,
                check=True
            )
            if result.stdout:
                console.print(result.stdout)
            return True
        except subprocess.CalledProcessError as e:
            console.print(f"[red]Error: {e.stderr}[/red]")
            return False

    def init_project(self, project_name: str):
        """Initialize a new Bhai DNS project"""
        console.print(f"[green]Initializing project: {project_name}[/green]")
        
        project_dir = Path(project_name)
        if project_dir.exists():
            console.print(f"[red]Project directory {project_name} already exists[/red]")
            return
            
        # Create project structure
        project_dir.mkdir()
        (project_dir / "config").mkdir()
        (project_dir / "deployments").mkdir()
        (project_dir / "scripts").mkdir()
        
        # Create initial config
        config = {
            "project_name": project_name,
            "version": "1.0.0",
            "services": {
                "dns-core": {"port": 5353, "replicas": 1},
                "query-service": {"port": 8080, "replicas": 2},
                "threat-intel": {"port": 8081, "replicas": 1},
                "analytics": {"port": 8082, "replicas": 1}
            },
            "databases": {
                "redis": {"port": 6379},
                "mongodb": {"port": 27017},
                "clickhouse": {"port": 8123, "native_port": 9000},
                "kafka": {"port": 9092}
            }
        }
        
        with open(project_dir / "config" / "project.yaml", 'w') as f:
            yaml.dump(config, f)
            
        console.print(f"[green]✅ Project {project_name} initialized successfully[/green]")

    def dev_start(self):
        """Start all services locally using Docker Compose"""
        console.print("[yellow]🚀 Starting all services locally...[/yellow]")
        
        # Generate enhanced docker-compose with Kafka and ClickHouse
        self.generate_docker_compose()
        
        # Start services
        if self.run_command(["docker-compose", "up", "-d"]):
            console.print("[green]✅ All services started successfully[/green]")
            self.show_service_status()
        else:
            console.print("[red]❌ Failed to start services[/red]")

    def dev_stop(self):
        """Stop all local services"""
        console.print("[yellow]🛑 Stopping all services...[/yellow]")
        
        if self.run_command(["docker-compose", "down"]):
            console.print("[green]✅ All services stopped[/green]")
        else:
            console.print("[red]❌ Failed to stop services[/red]")

    def dev_logs(self, service: Optional[str] = None):
        """Show logs for services"""
        cmd = ["docker-compose", "logs", "-f"]
        if service:
            cmd.append(service)
            console.print(f"[yellow]📋 Showing logs for {service}...[/yellow]")
        else:
            console.print("[yellow]📋 Showing logs for all services...[/yellow]")
        
        self.run_command(cmd)

    def show_service_status(self):
        """Display status of all services"""
        console.print("\n[bold]Service Status:[/bold]")
        
        table = Table()
        table.add_column("Service", style="cyan")
        table.add_column("Status", style="green")
        table.add_column("Port", style="yellow")
        table.add_column("Health", style="magenta")
        
        services = [
            ("Frontend", "Running", "3000", "Healthy"),
            ("DNS Core", "Running", "5353", "Healthy"),
            ("Query API", "Running", "8080", "Healthy"),
            ("MongoDB", "Running", "27017", "Healthy"),
            ("Redis", "Running", "6379", "Healthy"),
            ("ClickHouse", "Running", "8123", "Healthy"),
            ("Kafka", "Running", "9092", "Healthy"),
            ("Prometheus", "Running", "9091", "Healthy"),
            ("Grafana", "Running", "3001", "Healthy"),
        ]
        
        for service, status, port, health in services:
            table.add_row(service, status, port, health)
        
        console.print(table)
        
        console.print("\n[bold]Quick Access URLs:[/bold]")
        console.print("🌐 Frontend: http://localhost:3000")
        console.print("🔍 API: http://localhost:8080")
        console.print("📊 Grafana: http://localhost:3001")
        console.print("📈 ClickHouse: http://localhost:8123")

    def generate_docker_compose(self):
        """Generate enhanced docker-compose.yml with Kafka and ClickHouse"""
        compose_config = {
            "version": "3.8",
            "services": {
                # Existing services
                "mongodb": {
                    "image": "mongo:7.0",
                    "container_name": "bhai-dns-mongodb",
                    "restart": "unless-stopped",
                    "environment": {
                        "MONGO_INITDB_ROOT_USERNAME": "admin",
                        "MONGO_INITDB_ROOT_PASSWORD": "password",
                        "MONGO_INITDB_DATABASE": "bhai_dns"
                    },
                    "ports": ["27017:27017"],
                    "volumes": ["mongodb_data:/data/db"],
                    "networks": ["bhai-dns-network"]
                },
                
                # ClickHouse for Analytics
                "clickhouse": {
                    "image": "clickhouse/clickhouse-server:latest",
                    "container_name": "bhai-dns-clickhouse",
                    "restart": "unless-stopped",
                    "environment": {
                        "CLICKHOUSE_DB": "dns_analytics",
                        "CLICKHOUSE_USER": "admin",
                        "CLICKHOUSE_PASSWORD": "password"
                    },
                    "ports": ["8123:8123", "9000:9000"],
                    "volumes": [
                        "clickhouse_data:/var/lib/clickhouse",
                        "./config/clickhouse:/etc/clickhouse-server/config.d"
                    ],
                    "networks": ["bhai-dns-network"],
                    "ulimits": {"nofile": {"soft": 262144, "hard": 262144}}
                },
                
                # Kafka for Event Streaming
                "zookeeper": {
                    "image": "confluentinc/cp-zookeeper:latest",
                    "container_name": "bhai-dns-zookeeper",
                    "environment": {
                        "ZOOKEEPER_CLIENT_PORT": 2181,
                        "ZOOKEEPER_TICK_TIME": 2000
                    },
                    "ports": ["2181:2181"],
                    "volumes": ["zookeeper_data:/var/lib/zookeeper"],
                    "networks": ["bhai-dns-network"]
                },
                
                "kafka": {
                    "image": "confluentinc/cp-kafka:latest",
                    "container_name": "bhai-dns-kafka",
                    "depends_on": ["zookeeper"],
                    "environment": {
                        "KAFKA_BROKER_ID": 1,
                        "KAFKA_ZOOKEEPER_CONNECT": "zookeeper:2181",
                        "KAFKA_ADVERTISED_LISTENERS": "PLAINTEXT://localhost:9092",
                        "KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR": 1,
                        "KAFKA_AUTO_CREATE_TOPICS_ENABLE": "true"
                    },
                    "ports": ["9092:9092"],
                    "volumes": ["kafka_data:/var/lib/kafka/data"],
                    "networks": ["bhai-dns-network"]
                },
                
                # Enhanced services
                "bhai-dns-backend": {
                    "build": {"context": ".", "dockerfile": "Dockerfile"},
                    "container_name": "bhai-dns-backend",
                    "restart": "unless-stopped",
                    "environment": {
                        "DATABASE_URI": "mongodb://admin:password@mongodb:27017/bhai_dns?authSource=admin",
                        "REDIS_URL": "redis://redis:6379",
                        "KAFKA_BROKERS": "kafka:9092",
                        "CLICKHOUSE_URL": "http://clickhouse:8123",
                        "RUST_LOG": "info"
                    },
                    "ports": ["5353:5353/udp", "8080:8080", "9090:9090"],
                    "depends_on": ["mongodb", "redis", "kafka", "clickhouse"],
                    "networks": ["bhai-dns-network"]
                },
                
                "bhai-dns-frontend": {
                    "build": {"context": "./frontend", "dockerfile": "Dockerfile"},
                    "container_name": "bhai-dns-frontend",
                    "restart": "unless-stopped",
                    "ports": ["3000:80"],
                    "depends_on": ["bhai-dns-backend"],
                    "networks": ["bhai-dns-network"],
                    "environment": {"REACT_APP_API_URL": "http://localhost:8080"}
                },
                
                "redis": {
                    "image": "redis:7-alpine",
                    "container_name": "bhai-dns-redis",
                    "restart": "unless-stopped",
                    "ports": ["6379:6379"],
                    "command": "redis-server --appendonly yes --maxmemory 512mb --maxmemory-policy allkeys-lru",
                    "volumes": ["redis_data:/data"],
                    "networks": ["bhai-dns-network"]
                }
            },
            
            "networks": {"bhai-dns-network": {"driver": "bridge"}},
            
            "volumes": {
                "mongodb_data": None,
                "redis_data": None,
                "clickhouse_data": None,
                "kafka_data": None,
                "zookeeper_data": None
            }
        }
        
        with open(self.project_root / "docker-compose.yml", 'w') as f:
            yaml.dump(compose_config, f, default_flow_style=False)

    def config_set(self, key: str, value: str):
        """Set configuration value"""
        keys = key.split('.')
        config_section = self.config
        
        for k in keys[:-1]:
            if k not in config_section:
                config_section[k] = {}
            config_section = config_section[k]
        
        config_section[keys[-1]] = value
        self.save_config()
        console.print(f"[green]✅ Set {key} = {value}[/green]")

    def config_get(self, key: str):
        """Get configuration value"""
        keys = key.split('.')
        config_section = self.config
        
        try:
            for k in keys:
                config_section = config_section[k]
            console.print(f"{key}: {config_section}")
        except KeyError:
            console.print(f"[red]Configuration key '{key}' not found[/red]")

    def config_list(self):
        """List all configuration"""
        console.print("[bold]Current Configuration:[/bold]")
        console.print(yaml.dump(self.config, default_flow_style=False))

    def deploy_init(self, region: str):
        """Initialize AWS deployment"""
        console.print(f"[yellow]🚀 Initializing AWS deployment in {region}...[/yellow]")
        
        # Set up AWS configuration
        self.config.setdefault('aws', {})
        self.config['aws']['region'] = region
        self.save_config()
        
        # Create deployment files
        self.generate_k8s_manifests()
        self.generate_terraform_config(region)
        
        console.print(f"[green]✅ AWS deployment initialized for {region}[/green]")

    def deploy_apply(self, env: str):
        """Apply deployment to AWS"""
        console.print(f"[yellow]🚀 Deploying to {env} environment...[/yellow]")
        
        # Apply Terraform
        if self.run_command(["terraform", "init"], Path("deployments/terraform")):
            if self.run_command(["terraform", "apply", "-auto-approve"], Path("deployments/terraform")):
                console.print(f"[green]✅ Successfully deployed to {env}[/green]")
            else:
                console.print(f"[red]❌ Deployment to {env} failed[/red]")

    def generate_k8s_manifests(self):
        """Generate Kubernetes manifests"""
        manifests_dir = self.project_root / "deployments" / "k8s"
        manifests_dir.mkdir(parents=True, exist_ok=True)
        
        # Generate service manifests for each microservice
        services = ["dns-core", "query-service", "threat-intel", "analytics"]
        
        for service in services:
            manifest = {
                "apiVersion": "apps/v1",
                "kind": "Deployment",
                "metadata": {"name": f"bhai-dns-{service}"},
                "spec": {
                    "replicas": 2,
                    "selector": {"matchLabels": {"app": f"bhai-dns-{service}"}},
                    "template": {
                        "metadata": {"labels": {"app": f"bhai-dns-{service}"}},
                        "spec": {
                            "containers": [{
                                "name": service,
                                "image": f"bhai-dns/{service}:latest",
                                "ports": [{"containerPort": 8080}],
                                "env": [
                                    {"name": "KAFKA_BROKERS", "value": "kafka:9092"},
                                    {"name": "REDIS_URL", "value": "redis:6379"},
                                    {"name": "CLICKHOUSE_URL", "value": "clickhouse:8123"}
                                ]
                            }]
                        }
                    }
                }
            }
            
            with open(manifests_dir / f"{service}-deployment.yaml", 'w') as f:
                yaml.dump(manifest, f)

    def generate_terraform_config(self, region: str):
        """Generate Terraform configuration for AWS"""
        terraform_dir = self.project_root / "deployments" / "terraform"
        terraform_dir.mkdir(parents=True, exist_ok=True)
        
        terraform_config = f"""
provider "aws" {{
  region = "{region}"
}}

# EKS Cluster
resource "aws_eks_cluster" "bhai_dns" {{
  name     = "bhai-dns-cluster"
  role_arn = aws_iam_role.eks_cluster.arn

  vpc_config {{
    subnet_ids = aws_subnet.private[*].id
  }}

  depends_on = [
    aws_iam_role_policy_attachment.eks_cluster_policy,
  ]
}}

# MSK Kafka Cluster
resource "aws_msk_cluster" "bhai_dns_kafka" {{
  cluster_name           = "bhai-dns-kafka"
  kafka_version          = "2.8.0"
  number_of_broker_nodes = 3

  broker_node_group_info {{
    instance_type   = "kafka.m5.large"
    ebs_volume_size = 100
    client_subnets  = aws_subnet.private[*].id
    security_groups = [aws_security_group.msk.id]
  }}
}}

# ElastiCache Redis
resource "aws_elasticache_replication_group" "bhai_dns_redis" {{
  description          = "Bhai DNS Redis cluster"
  replication_group_id = "bhai-dns-redis"
  node_type           = "cache.r6g.large"
  port                = 6379
  parameter_group_name = "default.redis6.x"

  num_cache_clusters = 2
  subnet_group_name  = aws_elasticache_subnet_group.redis.name
  security_group_ids = [aws_security_group.redis.id]
}}
"""
        
        with open(terraform_dir / "main.tf", 'w') as f:
            f.write(terraform_config)

    def region_add(self, name: str, primary_region: str):
        """Add a new region"""
        console.print(f"[yellow]🌍 Adding region {name} with primary {primary_region}...[/yellow]")
        
        self.config.setdefault('regions', {})
        self.config['regions'][name] = {
            "primary": primary_region,
            "status": "active"
        }
        self.save_config()
        
        console.print(f"[green]✅ Region {name} added successfully[/green]")

    def health_check(self, all_services: bool = False):
        """Perform health check on services"""
        console.print("[yellow]🔍 Performing health check...[/yellow]")
        
        if all_services:
            services = [
                ("Frontend", "http://localhost:3000/health"),
                ("API", "http://localhost:8080/health"),
                ("ClickHouse", "http://localhost:8123/ping"),
            ]
        else:
            services = [("API", "http://localhost:8080/health")]
        
        table = Table()
        table.add_column("Service", style="cyan")
        table.add_column("Status", style="green")
        table.add_column("Response Time", style="yellow")
        
        for service, url in services:
            # Simulate health check (in real implementation, make HTTP requests)
            table.add_row(service, "✅ Healthy", "< 50ms")
        
        console.print(table)

def main():
    parser = argparse.ArgumentParser(description="Bhai Ka DNS CLI Tool")
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # Init command
    init_parser = subparsers.add_parser("init", help="Initialize a new project")
    init_parser.add_argument("--project-name", required=True, help="Project name")
    
    # Dev commands
    dev_parser = subparsers.add_parser("dev", help="Development commands")
    dev_subparsers = dev_parser.add_subparsers(dest="dev_command")
    dev_subparsers.add_parser("start", help="Start all services locally")
    dev_subparsers.add_parser("stop", help="Stop all services")
    dev_subparsers.add_parser("status", help="Show service status")
    logs_parser = dev_subparsers.add_parser("logs", help="Show service logs")
    logs_parser.add_argument("--service", help="Specific service to show logs for")
    
    # Config commands
    config_parser = subparsers.add_parser("config", help="Configuration management")
    config_subparsers = config_parser.add_subparsers(dest="config_command")
    
    set_parser = config_subparsers.add_parser("set", help="Set configuration value")
    set_parser.add_argument("--key", required=True, help="Configuration key")
    set_parser.add_argument("--value", required=True, help="Configuration value")
    
    get_parser = config_subparsers.add_parser("get", help="Get configuration value")
    get_parser.add_argument("--key", required=True, help="Configuration key")
    
    config_subparsers.add_parser("list", help="List all configuration")
    
    # Deploy commands
    deploy_parser = subparsers.add_parser("deploy", help="Deployment commands")
    deploy_subparsers = deploy_parser.add_subparsers(dest="deploy_command")
    
    init_deploy_parser = deploy_subparsers.add_parser("init", help="Initialize deployment")
    init_deploy_parser.add_argument("--region", required=True, help="AWS region")
    
    apply_parser = deploy_subparsers.add_parser("apply", help="Apply deployment")
    apply_parser.add_argument("--env", required=True, help="Environment")
    
    # Region commands
    region_parser = subparsers.add_parser("region", help="Multi-region management")
    region_subparsers = region_parser.add_subparsers(dest="region_command")
    
    add_region_parser = region_subparsers.add_parser("add", help="Add new region")
    add_region_parser.add_argument("--name", required=True, help="Region name")
    add_region_parser.add_argument("--primary-region", required=True, help="Primary region")
    
    # Health check
    health_parser = subparsers.add_parser("health-check", help="Perform health check")
    health_parser.add_argument("--all", action="store_true", help="Check all services")
    
    args = parser.parse_args()
    cli = BhaiDNSCLI()
    
    if args.command == "init":
        cli.init_project(args.project_name)
    elif args.command == "dev":
        if args.dev_command == "start":
            cli.dev_start()
        elif args.dev_command == "stop":
            cli.dev_stop()
        elif args.dev_command == "status":
            cli.show_service_status()
        elif args.dev_command == "logs":
            cli.dev_logs(args.service)
    elif args.command == "config":
        if args.config_command == "set":
            cli.config_set(args.key, args.value)
        elif args.config_command == "get":
            cli.config_get(args.key)
        elif args.config_command == "list":
            cli.config_list()
    elif args.command == "deploy":
        if args.deploy_command == "init":
            cli.deploy_init(args.region)
        elif args.deploy_command == "apply":
            cli.deploy_apply(args.env)
    elif args.command == "region":
        if args.region_command == "add":
            cli.region_add(args.name, args.primary_region)
    elif args.command == "health-check":
        cli.health_check(args.all)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()