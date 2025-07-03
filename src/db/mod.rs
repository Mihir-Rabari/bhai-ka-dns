use mongodb::{Client, Database as MongoDatabase, Collection};
use bson::{doc, Document};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{config::DatabaseConfig, errors::Result};

pub mod models;
pub mod queries;

pub use models::*;
pub use queries::*;

#[derive(Clone)]
pub struct Database {
    pub client: Client,
    pub db: MongoDatabase,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let client = Client::with_uri_str(&config.uri).await?;
        let db = client.database(&config.database_name);
        
        // Ensure indexes exist
        Self::create_indexes(&db).await?;
        
        Ok(Self { client, db })
    }
    
    async fn create_indexes(db: &MongoDatabase) -> Result<()> {
        // DNS queries collection indexes
        let dns_queries: Collection<Document> = db.collection("dns_queries");
        dns_queries.create_index(doc! { "domain": 1, "timestamp": -1 }, None).await?;
        dns_queries.create_index(doc! { "timestamp": -1 }, None).await?;
        dns_queries.create_index(doc! { "client_ip": 1 }, None).await?;
        
        // Analytics collection indexes
        let analytics: Collection<Document> = db.collection("analytics");
        analytics.create_index(doc! { "date": -1 }, None).await?;
        analytics.create_index(doc! { "metric_type": 1, "date": -1 }, None).await?;
        
        // Users collection indexes
        let users: Collection<Document> = db.collection("users");
        users.create_index(doc! { "email": 1 }, None).await?;
        users.create_index(doc! { "username": 1 }, None).await?;
        
        // Threat intelligence collection indexes
        let threats: Collection<Document> = db.collection("threat_intelligence");
        threats.create_index(doc! { "domain": 1 }, None).await?;
        threats.create_index(doc! { "threat_type": 1 }, None).await?;
        threats.create_index(doc! { "updated_at": -1 }, None).await?;
        
        Ok(())
    }
    
    pub fn dns_queries(&self) -> Collection<DnsQuery> {
        self.db.collection("dns_queries")
    }
    
    pub fn analytics(&self) -> Collection<AnalyticsRecord> {
        self.db.collection("analytics")
    }
    
    pub fn users(&self) -> Collection<User> {
        self.db.collection("users")
    }
    
    pub fn threat_intelligence(&self) -> Collection<ThreatIntelligence> {
        self.db.collection("threat_intelligence")
    }
    
    pub fn ai_models(&self) -> Collection<AIModel> {
        self.db.collection("ai_models")
    }
}