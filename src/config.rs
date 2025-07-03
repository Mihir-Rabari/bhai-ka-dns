use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub dns: DNSConfig,
    pub web: WebConfig,
    pub database: DatabaseConfig,
    pub ai: AIConfig,
    pub analytics: AnalyticsConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSConfig {
    pub host: String,
    pub port: u16,
    pub upstream_servers: Vec<String>,
    pub cache_size: usize,
    pub cache_ttl: u64,
    pub enable_ai_features: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub static_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub uri: String,
    pub database_name: String,
    pub max_pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub threat_detection: bool,
    pub typo_correction: bool,
    pub domain_analysis: bool,
    pub ml_model_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub metrics_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry: u64,
    pub enable_registration: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dns: DNSConfig {
                host: "0.0.0.0".to_string(),
                port: 5353,
                upstream_servers: vec![
                    "8.8.8.8:53".to_string(),
                    "1.1.1.1:53".to_string(),
                ],
                cache_size: 10000,
                cache_ttl: 300,
                enable_ai_features: true,
            },
            web: WebConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                cors_origins: vec!["*".to_string()],
                static_dir: "./frontend/dist".to_string(),
            },
            database: DatabaseConfig {
                uri: "mongodb://localhost:27017".to_string(),
                database_name: "bhai_dns".to_string(),
                max_pool_size: 100,
            },
            ai: AIConfig {
                threat_detection: true,
                typo_correction: true,
                domain_analysis: true,
                ml_model_path: None,
            },
            analytics: AnalyticsConfig {
                enabled: true,
                retention_days: 30,
                metrics_port: 9090,
            },
            auth: AuthConfig {
                jwt_secret: "your-secret-key-change-in-production".to_string(),
                token_expiry: 3600,
                enable_registration: true,
            },
        }
    }
}

impl Config {
    pub async fn load(path: &str) -> Result<Self> {
        // Load from environment variables first
        dotenvy::dotenv().ok();
        
        // Try to load from file, fallback to default
        let config = match std::fs::read_to_string(path) {
            Ok(contents) => toml::from_str(&contents)?,
            Err(_) => {
                tracing::warn!("Config file not found, using defaults");
                Self::default()
            }
        };
        
        Ok(config)
    }
    
    pub fn dns_address(&self) -> SocketAddr {
        format!("{}:{}", self.dns.host, self.dns.port)
            .parse()
            .expect("Invalid DNS address")
    }
    
    pub fn web_address(&self) -> SocketAddr {
        format!("{}:{}", self.web.host, self.web.port)
            .parse()
            .expect("Invalid web address")
    }
}