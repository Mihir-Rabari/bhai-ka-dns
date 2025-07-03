use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};
use serde_json::json;
use tokio::time::interval;
use tracing::{debug, error, info};

use crate::db::{Database, DnsQuery, AnalyticsRecord, DashboardStats};
use crate::errors::Result;

#[derive(Clone)]
pub struct Analytics {
    db: Database,
}

impl Analytics {
    pub async fn new(db: Database) -> Result<Self> {
        let analytics = Self { db };
        
        // Start background aggregation task
        let analytics_clone = analytics.clone();
        tokio::spawn(async move {
            analytics_clone.start_aggregation_task().await;
        });
        
        info!("Analytics service initialized");
        Ok(analytics)
    }
    
    pub async fn log_dns_query(&self, query: DnsQuery) -> Result<()> {
        self.db.dns_queries().insert_one(query, None).await?;
        Ok(())
    }
    
    async fn start_aggregation_task(&self) {
        let mut interval = interval(tokio::time::Duration::from_secs(300)); // Every 5 minutes
        
        loop {
            interval.tick().await;
            if let Err(e) = self.aggregate_metrics().await {
                error!("Failed to aggregate metrics: {}", e);
            }
        }
    }
    
    async fn aggregate_metrics(&self) -> Result<()> {
        let now = Utc::now();
        let five_minutes_ago = now - Duration::minutes(5);
        
        debug!("Aggregating metrics for the last 5 minutes");
        
        // Query count
        let query_count = self.db.dns_queries()
            .count_documents(bson::doc! {
                "timestamp": {
                    "$gte": five_minutes_ago,
                    "$lt": now
                }
            }, None)
            .await? as f64;
        
        // Threat count
        let threat_count = self.db.dns_queries()
            .count_documents(bson::doc! {
                "timestamp": {
                    "$gte": five_minutes_ago,
                    "$lt": now
                },
                "threat_detected": true
            }, None)
            .await? as f64;
        
        // Cache hit rate
        let cache_hits = self.db.dns_queries()
            .count_documents(bson::doc! {
                "timestamp": {
                    "$gte": five_minutes_ago,
                    "$lt": now
                },
                "cache_hit": true
            }, None)
            .await? as f64;
        
        let cache_hit_rate = if query_count > 0.0 {
            (cache_hits / query_count) * 100.0
        } else {
            0.0
        };
        
        // Store aggregated metrics
        let metrics = vec![
            AnalyticsRecord {
                id: None,
                date: now,
                metric_type: "query_count".to_string(),
                value: query_count,
                metadata: json!({
                    "period": "5min"
                }).as_object().unwrap().clone(),
            },
            AnalyticsRecord {
                id: None,
                date: now,
                metric_type: "threat_count".to_string(),
                value: threat_count,
                metadata: json!({
                    "period": "5min"
                }).as_object().unwrap().clone(),
            },
            AnalyticsRecord {
                id: None,
                date: now,
                metric_type: "cache_hit_rate".to_string(),
                value: cache_hit_rate,
                metadata: json!({
                    "period": "5min"
                }).as_object().unwrap().clone(),
            },
        ];
        
        self.db.analytics().insert_many(metrics, None).await?;
        
        Ok(())
    }
}