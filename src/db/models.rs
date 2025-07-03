use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQuery {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub domain: String,
    pub query_type: String,
    pub client_ip: String,
    pub timestamp: DateTime<Utc>,
    pub response_code: i32,
    pub response_time_ms: f64,
    pub ai_analysis: Option<AiAnalysis>,
    pub threat_detected: bool,
    pub cache_hit: bool,
    pub upstream_server: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub security_score: f64,
    pub trust_level: String,
    pub category: String,
    pub flags: Vec<String>,
    pub recommendations: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRecord {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub date: DateTime<Utc>,
    pub metric_type: String,
    pub value: f64,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub enable_notifications: bool,
    pub dashboard_layout: String,
    pub theme: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub domain: String,
    pub threat_type: String,
    pub severity: String,
    pub description: String,
    pub source: String,
    pub confidence: f64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub version: String,
    pub model_type: String,
    pub accuracy: f64,
    pub training_data_size: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_queries: i64,
    pub threats_blocked: i64,
    pub cache_hit_rate: f64,
    pub average_response_time: f64,
    pub top_domains: Vec<DomainStat>,
    pub threat_trends: Vec<ThreatTrend>,
    pub query_trends: Vec<QueryTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStat {
    pub domain: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    pub date: DateTime<Utc>,
    pub count: i64,
    pub threat_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTrend {
    pub date: DateTime<Utc>,
    pub count: i64,
    pub average_response_time: f64,
}