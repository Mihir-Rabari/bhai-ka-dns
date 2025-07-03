use mongodb::{Client, Database as MongoDatabase, Collection, IndexModel};
use bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};
use crate::{config::DatabaseConfig, errors::Result};

pub mod models;
pub mod queries;

pub use models::*;
pub use queries::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQuery {
    pub id: String,
    pub domain: String,
    pub query_type: String,
    pub client_ip: String,
    pub response_code: String,
    pub response_time_ms: u64,
    pub timestamp: DateTime,
    pub cached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRecord {
    pub id: String,
    pub metric_type: String,
    pub value: f64,
    pub tags: std::collections::HashMap<String, serde_json::Value>,
    pub timestamp: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime,
    pub last_login: Option<DateTime>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatRecord {
    pub id: String,
    pub domain: String,
    pub threat_type: String,
    pub severity: String,
    pub source: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub id: String,
    pub domain: String,
    pub analysis_type: String,
    pub result: serde_json::Value,
    pub confidence: f64,
    pub timestamp: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_queries: u64,
    pub unique_domains: u64,
    pub blocked_queries: u64,
    pub cache_hit_rate: f64,
    pub average_response_time: f64,
    pub top_domains: Vec<(String, u64)>,
    pub threat_detections: u64,
}

pub struct Database {
    client: Client,
    db: MongoDatabase,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let client = Client::with_uri_str(&config.uri).await?;
        let db = client.database(&config.database_name);

        let database = Self { client, db };
        database.create_indexes().await?;

        Ok(database)
    }

    async fn create_indexes(&self) -> Result<()> {
        // DNS queries indexes
        let dns_queries: Collection<Document> = self.db.collection("dns_queries");
        dns_queries.create_index(
            IndexModel::builder()
                .keys(doc! { "domain": 1, "timestamp": -1 })
                .build(),
            None
        ).await?;
        dns_queries.create_index(
            IndexModel::builder()
                .keys(doc! { "timestamp": -1 })
                .build(),
            None
        ).await?;
        dns_queries.create_index(
            IndexModel::builder()
                .keys(doc! { "client_ip": 1 })
                .build(),
            None
        ).await?;

        // Analytics indexes
        let analytics: Collection<Document> = self.db.collection("analytics");
        analytics.create_index(
            IndexModel::builder()
                .keys(doc! { "date": -1 })
                .build(),
            None
        ).await?;
        analytics.create_index(
            IndexModel::builder()
                .keys(doc! { "metric_type": 1, "date": -1 })
                .build(),
            None
        ).await?;

        // Users indexes
        let users: Collection<Document> = self.db.collection("users");
        users.create_index(
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .build(),
            None
        ).await?;
        users.create_index(
            IndexModel::builder()
                .keys(doc! { "username": 1 })
                .build(),
            None
        ).await?;

        // Threats indexes
        let threats: Collection<Document> = self.db.collection("threats");
        threats.create_index(
            IndexModel::builder()
                .keys(doc! { "domain": 1 })
                .build(),
            None
        ).await?;
        threats.create_index(
            IndexModel::builder()
                .keys(doc! { "threat_type": 1 })
                .build(),
            None
        ).await?;
        threats.create_index(
            IndexModel::builder()
                .keys(doc! { "updated_at": -1 })
                .build(),
            None
        ).await?;

        Ok(())
    }

    pub fn get_database(&self) -> &MongoDatabase {
        &self.db
    }

    pub async fn save_dns_query(&self, query: &DnsQuery) -> Result<()> {
        let collection: Collection<DnsQuery> = self.db.collection("dns_queries");
        collection.insert_one(query, None).await?;
        Ok(())
    }

    pub async fn save_analytics_record(&self, record: &AnalyticsRecord) -> Result<()> {
        let collection: Collection<AnalyticsRecord> = self.db.collection("analytics");
        collection.insert_one(record, None).await?;
        Ok(())
    }

    pub async fn save_threat_record(&self, threat: &ThreatRecord) -> Result<()> {
        let collection: Collection<ThreatRecord> = self.db.collection("threats");
        collection.insert_one(threat, None).await?;
        Ok(())
    }

    pub async fn save_ai_analysis(&self, analysis: &AiAnalysis) -> Result<()> {
        let collection: Collection<AiAnalysis> = self.db.collection("ai_analysis");
        collection.insert_one(analysis, None).await?;
        Ok(())
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let collection: Collection<User> = self.db.collection("users");
        let user = collection
            .find_one(doc! { "email": email }, None)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let collection: Collection<User> = self.db.collection("users");
        let user = collection
            .find_one(doc! { "username": username }, None)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let collection: Collection<User> = self.db.collection("users");
        collection.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn get_dashboard_stats(&self) -> Result<DashboardStats> {
        // This is a simplified implementation
        // In production, you'd want to use aggregation pipelines for better performance
        Ok(DashboardStats {
            total_queries: 0,
            unique_domains: 0,
            blocked_queries: 0,
            cache_hit_rate: 0.0,
            average_response_time: 0.0,
            top_domains: vec![],
            threat_detections: 0,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub uri: String,
    pub database_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub timeout_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            uri: "mongodb://localhost:27017".to_string(),
            database_name: "bhai_dns".to_string(),
            max_connections: 100,
            min_connections: 5,
            timeout_ms: 5000,
        }
    }
}