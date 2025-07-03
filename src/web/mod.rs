use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderValue, Method},
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use serde::{Deserialize, Serialize};

use crate::{
    analytics::Analytics,
    config::WebConfig,
    db::Database,
    errors::Result,
};

pub mod handlers;
pub mod auth;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub analytics: Arc<Analytics>,
}

pub struct WebServer {
    config: WebConfig,
    app: Router,
}

impl WebServer {
    pub async fn new(
        config: WebConfig,
        db: Database,
        analytics: Arc<Analytics>,
    ) -> Result<Self> {
        let state = AppState { db, analytics };
        
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any)
            .allow_origin(Any);
        
        let app = Router::new()
            // Health check
            .route("/health", get(health_check))
            
            // DNS API
            .route("/api/dns/lookup", post(dns_lookup))
            .route("/api/dns/analyze", post(analyze_domain))
            .route("/api/dns/suggest", post(suggest_domains))
            
            // Analytics API
            .route("/api/analytics/stats", get(get_stats))
            .route("/api/analytics/dashboard", get(get_dashboard_stats))
            .route("/api/analytics/trends", get(get_trends))
            
            // Admin API
            .route("/api/admin/threats", get(list_threats))
            .route("/api/admin/threats", post(add_threat))
            .route("/api/admin/threats/:domain", axum::routing::delete(remove_threat))
            
            // User management
            .route("/api/auth/register", post(register_user))
            .route("/api/auth/login", post(login_user))
            .route("/api/auth/profile", get(get_profile))
            
            .layer(cors)
            .with_state(state);
        
        Ok(Self { config, app })
    }
    
    pub async fn start(self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        info!("🌐 Web server starting on {}", addr);
        
        axum::serve(listener, self.app)
            .await
            .map_err(|e| crate::errors::AppError::Internal(format!("Server error: {}", e)))?;
        
        Ok(())
    }
}