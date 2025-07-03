use crate::errors::Result;
use mongodb::{Collection, Database as MongoDatabase};
use bson::{doc, Document};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStats {
    pub total_queries: i64,
    pub unique_domains: i64,
    pub top_domains: Vec<(String, i64)>,
    pub query_types: Vec<(String, i64)>,
}

pub struct QueryBuilder {
    db: MongoDatabase,
}

impl QueryBuilder {
    pub fn new(db: MongoDatabase) -> Self {
        Self { db }
    }

    pub async fn get_query_stats(&self, days: i32) -> Result<QueryStats> {
        let dns_queries: Collection<Document> = self.db.collection("dns_queries");
        
        let start_date = Utc::now() - chrono::Duration::days(days as i64);
        let filter = doc! {
            "timestamp": {
                "$gte": start_date
            }
        };

        // Total queries
        let total_queries = dns_queries.count_documents(filter.clone(), None).await? as i64;

        // Unique domains
        let unique_domains = dns_queries
            .distinct("domain", filter.clone(), None)
            .await?
            .len() as i64;

        // Top domains
        let top_domains_pipeline = vec![
            doc! { "$match": filter.clone() },
            doc! { "$group": {
                "_id": "$domain",
                "count": { "$sum": 1 }
            }},
            doc! { "$sort": { "count": -1 } },
            doc! { "$limit": 10 }
        ];

        let mut top_domains_cursor = dns_queries.aggregate(top_domains_pipeline, None).await?;
        let mut top_domains = Vec::new();
        
        while let Some(doc) = top_domains_cursor.next().await {
            match doc {
                Ok(document) => {
                    if let (Some(domain), Some(count)) = (
                        document.get_str("_id").ok(),
                        document.get_i32("count").ok()
                    ) {
                        top_domains.push((domain.to_string(), count as i64));
                    }
                }
                Err(_) => continue,
            }
        }

        // Query types
        let query_types_pipeline = vec![
            doc! { "$match": filter },
            doc! { "$group": {
                "_id": "$query_type",
                "count": { "$sum": 1 }
            }},
            doc! { "$sort": { "count": -1 } }
        ];

        let mut query_types_cursor = dns_queries.aggregate(query_types_pipeline, None).await?;
        let mut query_types = Vec::new();
        
        while let Some(doc) = query_types_cursor.next().await {
            match doc {
                Ok(document) => {
                    if let (Some(query_type), Some(count)) = (
                        document.get_str("_id").ok(),
                        document.get_i32("count").ok()
                    ) {
                        query_types.push((query_type.to_string(), count as i64));
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(QueryStats {
            total_queries,
            unique_domains,
            top_domains,
            query_types,
        })
    }

    pub async fn get_domain_history(&self, domain: &str, limit: i64) -> Result<Vec<Document>> {
        let dns_queries: Collection<Document> = self.db.collection("dns_queries");
        
        let filter = doc! { "domain": domain };
        let options = mongodb::options::FindOptions::builder()
            .limit(limit)
            .sort(doc! { "timestamp": -1 })
            .build();

        let mut cursor = dns_queries.find(filter, options).await?;
        let mut results = Vec::new();
        
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(document) => results.push(document),
                Err(_) => continue,
            }
        }

        Ok(results)
    }

    pub async fn get_analytics_data(&self, metric_type: &str, days: i32) -> Result<Vec<Document>> {
        let analytics: Collection<Document> = self.db.collection("analytics");
        
        let start_date = Utc::now() - chrono::Duration::days(days as i64);
        let filter = doc! {
            "metric_type": metric_type,
            "date": {
                "$gte": start_date
            }
        };

        let options = mongodb::options::FindOptions::builder()
            .sort(doc! { "date": 1 })
            .build();

        let mut cursor = analytics.find(filter, options).await?;
        let mut results = Vec::new();
        
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(document) => results.push(document),
                Err(_) => continue,
            }
        }

        Ok(results)
    }
}