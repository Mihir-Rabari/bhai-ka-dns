use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::rr::{DNSClass, RecordType};
use trust_dns_client::udp::UdpClientStream;
use trust_dns_proto::op::{Message, Query};
use trust_dns_proto::rr::Name;
use dashmap::DashMap;
use lru::LruCache;
use chrono::Utc;

use crate::ai::AIEngine;
use crate::analytics::Analytics;
use crate::config::DNSConfig;
use crate::db::{Database, DnsQuery, AiAnalysis};
use crate::errors::{AppError, Result};

pub mod cache;
pub mod resolver;

use cache::DNSCache;
use resolver::DNSResolver;

pub struct DNSServer {
    config: DNSConfig,
    socket: Arc<UdpSocket>,
    cache: Arc<DNSCache>,
    resolver: Arc<DNSResolver>,
    ai_engine: Arc<AIEngine>,
    analytics: Arc<Analytics>,
    stats: Arc<ServerStats>,
}

#[derive(Debug, Default)]
pub struct ServerStats {
    pub total_queries: parking_lot::Mutex<u64>,
    pub cache_hits: parking_lot::Mutex<u64>,
    pub threats_blocked: parking_lot::Mutex<u64>,
    pub ai_suggestions: parking_lot::Mutex<u64>,
    pub errors: parking_lot::Mutex<u64>,
    pub average_response_time: parking_lot::Mutex<f64>,
}

impl DNSServer {
    pub async fn new(config: DNSConfig, analytics: Arc<Analytics>) -> Result<Self> {
        let addr = format!("{}:{}", config.host, config.port);
        let socket = Arc::new(UdpSocket::bind(&addr).await
            .map_err(|e| AppError::Dns(format!("Failed to bind to {}: {}", addr, e)))?);
        
        let cache = Arc::new(DNSCache::new(config.cache_size, config.cache_ttl));
        let resolver = Arc::new(DNSResolver::new(config.upstream_servers.clone()).await?);
        let ai_engine = Arc::new(AIEngine::new().await?);
        let stats = Arc::new(ServerStats::default());
        
        info!("DNS server initialized on {}", addr);
        
        Ok(Self {
            config,
            socket,
            cache,
            resolver,
            ai_engine,
            analytics,
            stats,
        })
    }
    
    pub async fn start(self: Arc<Self>) -> Result<()> {
        info!("🚀 Starting DNS server on {}:{}", self.config.host, self.config.port);
        
        let mut buf = vec![0u8; 512];
        
        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let server = self.clone();
                    let data = buf[..len].to_vec();
                    
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_query(data, addr).await {
                            error!("Error handling query from {}: {}", addr, e);
                            *server.stats.errors.lock() += 1;
                        }
                    });
                }
                Err(e) => {
                    error!("Error receiving DNS query: {}", e);
                    *self.stats.errors.lock() += 1;
                }
            }
        }
    }
    
    async fn handle_query(&self, data: Vec<u8>, client_addr: SocketAddr) -> Result<()> {
        let start_time = Instant::now();
        *self.stats.total_queries.lock() += 1;
        
        // Parse the DNS message
        let message = Message::from_vec(&data)
            .map_err(|e| AppError::Dns(format!("Failed to parse DNS message: {}", e)))?;
        
        if message.queries().is_empty() {
            return Err(AppError::Dns("No queries in message".to_string()));
        }
        
        let query = &message.queries()[0];
        let domain = query.name().to_string().trim_end_matches('.').to_string();
        let query_type = query.query_type();
        
        debug!("🔍 DNS Query: {} {} from {}", domain, query_type, client_addr);
        
        // AI threat detection
        if self.config.enable_ai_features {
            if let Ok(threat_result) = self.ai_engine.detect_threat(&domain).await {
                if threat_result.is_threat {
                    warn!("🚫 Threat detected: {} (confidence: {:.2})", domain, threat_result.confidence);
                    *self.stats.threats_blocked.lock() += 1;
                    
                    // Log to analytics
                    self.log_query_analytics(&domain, query_type, client_addr, start_time, true, false).await?;
                    
                    // Return NXDOMAIN for threats
                    let mut response = message.clone();
                    response.set_response_code(trust_dns_proto::op::ResponseCode::NXDomain);
                    self.send_response(response, client_addr).await?;
                    return Ok(());
                }
            }
        }
        
        // Check cache first
        let cache_key = format!("{}:{}", domain, query_type);
        if let Some(cached_response) = self.cache.get(&cache_key).await {
            debug!("💾 Cache hit for {}", domain);
            *self.stats.cache_hits.lock() += 1;
            
            self.log_query_analytics(&domain, query_type, client_addr, start_time, false, true).await?;
            self.send_response(cached_response, client_addr).await?;
            return Ok(());
        }
        
        // AI typo correction
        let corrected_domain = if self.config.enable_ai_features {
            if let Ok(suggestion) = self.ai_engine.suggest_correction(&domain).await {
                if let Some(corrected) = suggestion.suggestion {
                    info!("🔧 AI suggestion: {} -> {}", domain, corrected);
                    *self.stats.ai_suggestions.lock() += 1;
                    corrected
                } else {
                    domain.clone()
                }
            } else {
                domain.clone()
            }
        } else {
            domain.clone()
        };
        
        // Resolve the query
        match self.resolver.resolve(&corrected_domain, query_type).await {
            Ok(response) => {
                // Cache the response
                self.cache.set(cache_key, response.clone(), Duration::from_secs(self.config.cache_ttl)).await;
                
                // Log analytics
                self.log_query_analytics(&domain, query_type, client_addr, start_time, false, false).await?;
                
                // Send response
                self.send_response(response, client_addr).await?;
            }
            Err(e) => {
                error!("❌ Failed to resolve {}: {}", corrected_domain, e);
                
                // Return NXDOMAIN
                let mut response = message.clone();
                response.set_response_code(trust_dns_proto::op::ResponseCode::NXDomain);
                self.send_response(response, client_addr).await?;
            }
        }
        
        // Update average response time
        let response_time = start_time.elapsed().as_millis() as f64;
        let mut avg_time = self.stats.average_response_time.lock();
        *avg_time = (*avg_time * 0.9) + (response_time * 0.1); // Exponential moving average
        
        Ok(())
    }
    
    async fn send_response(&self, response: Message, client_addr: SocketAddr) -> Result<()> {
        let response_data = response.to_vec()
            .map_err(|e| AppError::Dns(format!("Failed to serialize response: {}", e)))?;
        
        self.socket.send_to(&response_data, client_addr).await
            .map_err(|e| AppError::Dns(format!("Failed to send response: {}", e)))?;
        
        Ok(())
    }
    
    async fn log_query_analytics(
        &self,
        domain: &str,
        query_type: RecordType,
        client_addr: SocketAddr,
        start_time: Instant,
        threat_detected: bool,
        cache_hit: bool,
    ) -> Result<()> {
        let response_time = start_time.elapsed().as_millis() as f64;
        
        // Perform AI analysis if enabled
        let ai_analysis = if self.config.enable_ai_features && !threat_detected {
            match self.ai_engine.analyze_domain(domain).await {
                Ok(analysis) => Some(AiAnalysis {
                    security_score: analysis.security_score,
                    trust_level: analysis.trust_level,
                    category: analysis.category,
                    flags: analysis.flags,
                    recommendations: analysis.recommendations,
                    confidence: analysis.confidence,
                }),
                Err(e) => {
                    warn!("AI analysis failed for {}: {}", domain, e);
                    None
                }
            }
        } else {
            None
        };
        
        let query_record = DnsQuery {
            id: None,
            domain: domain.to_string(),
            query_type: query_type.to_string(),
            client_ip: client_addr.ip().to_string(),
            timestamp: Utc::now(),
            response_code: if threat_detected { 3 } else { 0 }, // NXDOMAIN for threats
            response_time_ms: response_time,
            ai_analysis,
            threat_detected,
            cache_hit,
            upstream_server: if !cache_hit { Some("upstream".to_string()) } else { None },
        };
        
        // Log to analytics (non-blocking)
        let analytics = self.analytics.clone();
        tokio::spawn(async move {
            if let Err(e) = analytics.log_dns_query(query_record).await {
                warn!("Failed to log DNS query analytics: {}", e);
            }
        });
        
        Ok(())
    }
    
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("total_queries".to_string(), (*self.stats.total_queries.lock()).into());
        stats.insert("cache_hits".to_string(), (*self.stats.cache_hits.lock()).into());
        stats.insert("threats_blocked".to_string(), (*self.stats.threats_blocked.lock()).into());
        stats.insert("ai_suggestions".to_string(), (*self.stats.ai_suggestions.lock()).into());
        stats.insert("errors".to_string(), (*self.stats.errors.lock()).into());
        stats.insert("average_response_time_ms".to_string(), (*self.stats.average_response_time.lock()).into());
        
        let cache_hit_rate = if *self.stats.total_queries.lock() > 0 {
            (*self.stats.cache_hits.lock() as f64) / (*self.stats.total_queries.lock() as f64) * 100.0
        } else {
            0.0
        };
        stats.insert("cache_hit_rate_percent".to_string(), cache_hit_rate.into());
        
        stats
    }
}