use anyhow::Result;
use clap::Parser;
use tokio::signal;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

mod config;
mod dns;
mod web;
mod ai;
mod analytics;
mod db;
// mod auth;  // TODO: Implement auth module
mod errors;
mod redis;

use config::Config;
use dns::DNSServer;
use web::WebServer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("bhai_dns_server={log_level}").into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting Bhai Ka DNS - AI-Powered DNS Server");
    
    // Load configuration
    let config = Config::load(&args.config).await?;
    info!("✅ Configuration loaded");
    
    // Initialize database connection
    let db = db::Database::new(&config.database).await?;
    info!("✅ Database connected");
    
    // Initialize analytics
    let analytics = Arc::new(analytics::Analytics::new(db.clone()).await?);
    info!("✅ Analytics initialized");
    
    // Start DNS server
    let dns_server = Arc::new(DNSServer::new(config.dns.clone(), analytics.clone()).await?);
    let dns_handle = tokio::spawn({
        let dns_server = dns_server.clone();
        async move {
            if let Err(e) = dns_server.start().await {
                warn!("DNS server error: {}", e);
            }
        }
    });
    info!("✅ DNS server started on port {}", config.dns.port);
    
    // Start web server
    let web_server = WebServer::new(config.web.clone(), db, analytics.clone()).await?;
    let web_handle = tokio::spawn(async move {
        if let Err(e) = web_server.start().await {
            warn!("Web server error: {}", e);
        }
    });
    info!("✅ Web server started on port {}", config.web.port);
    
    info!("🌟 Bhai Ka DNS is now running!");
    info!("🌐 Web interface: http://localhost:{}", config.web.port);
    info!("🔧 DNS server: localhost:{}", config.dns.port);
    
    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("🛑 Shutdown signal received");
        }
        _ = dns_handle => {
            warn!("DNS server terminated");
        }
        _ = web_handle => {
            warn!("Web server terminated");
        }
    }
    
    info!("👋 Bhai Ka DNS shutting down gracefully");
    Ok(())
}