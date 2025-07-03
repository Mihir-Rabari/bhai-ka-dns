use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use tracing::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use bloom::{ASMS, BloomFilter};
use sha2::{Sha256, Digest};

use crate::errors::{AppError, Result};
use super::{ThreatResult, calculate_domain_entropy, has_suspicious_pattern, check_brand_impersonation, analyze_tld_suspicion};

#[derive(Debug, Serialize, Deserialize)]
struct ThreatIntelligenceFeed {
    domains: Vec<String>,
    source: String,
    confidence: f64,
}

pub struct ThreatDetector {
    known_threats: RwLock<HashSet<String>>,
    threat_bloom: RwLock<BloomFilter>,
    client: Client,
    model_weights: HashMap<String, f64>,
}

impl ThreatDetector {
    pub async fn new() -> Result<Self> {
        let mut known_threats = HashSet::new();
        
        // Load initial threat intelligence
        Self::load_initial_threats(&mut known_threats);
        
        // Create bloom filter for fast threat checking
        let mut bloom = BloomFilter::with_rate(0.1, 1000000);
        for threat in &known_threats {
            bloom.insert(&Self::hash_domain(threat));
        }
        
        // Initialize ML model weights
        let model_weights = Self::initialize_model_weights();
        
        let detector = Self {
            known_threats: RwLock::new(known_threats),
            threat_bloom: RwLock::new(bloom),
            client: Client::new(),
            model_weights,
        };
        
        // Start background threat intelligence updates
        let detector_clone = detector.client.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Update hourly
            loop {
                interval.tick().await;
                if let Err(e) = Self::update_threat_intelligence(detector_clone.clone()).await {
                    tracing::warn!("Failed to update threat intelligence: {}", e);
                }
            }
        });
        
        Ok(detector)
    }
    
    pub async fn analyze(&self, domain: &str) -> Result<ThreatResult> {
        let domain_lower = domain.to_lowercase();
        let domain_hash = Self::hash_domain(&domain_lower);
        
        // Fast bloom filter check
        if self.threat_bloom.read().await.contains(&domain_hash) {
            if self.known_threats.read().await.contains(&domain_lower) {
                return Ok(ThreatResult {
                    is_threat: true,
                    threat_type: "known_malicious".to_string(),
                    confidence: 0.95,
                    reasons: vec!["Domain found in threat intelligence database".to_string()],
                });
            }
        }
        
        // AI-based analysis
        self.ai_analysis(&domain_lower).await
    }
    
    async fn ai_analysis(&self, domain: &str) -> Result<ThreatResult> {
        let mut threat_score = 0.0;
        let mut reasons = Vec::new();
        
        // Feature extraction
        let features = self.extract_features(domain);
        
        // Apply ML model
        for (feature, value) in &features {
            if let Some(&weight) = self.model_weights.get(feature) {
                threat_score += value * weight;
            }
        }
        
        // Threshold-based classification
        let is_threat = threat_score > 0.7;
        let confidence = if is_threat {
            threat_score.min(0.95)
        } else {
            (1.0 - threat_score).min(0.95)
        };
        
        // Generate reasons based on features
        if features.get("suspicious_patterns").unwrap_or(&0.0) > &0.0 {
            reasons.push("Contains suspicious patterns".to_string());
        }
        if features.get("brand_impersonation").unwrap_or(&0.0) > &0.0 {
            reasons.push("Potential brand impersonation".to_string());
        }
        if features.get("suspicious_tld").unwrap_or(&0.0) > &0.0 {
            reasons.push("Uses suspicious top-level domain".to_string());
        }
        if features.get("low_entropy").unwrap_or(&0.0) > &0.0 {
            reasons.push("Unusual character distribution".to_string());
        }
        if features.get("excessive_length").unwrap_or(&0.0) > &0.0 {
            reasons.push("Unusually long domain name".to_string());
        }
        
        let threat_type = if is_threat {
            if reasons.iter().any(|r| r.contains("phishing") || r.contains("impersonation")) {
                "phishing".to_string()
            } else if reasons.iter().any(|r| r.contains("suspicious")) {
                "suspicious".to_string()
            } else {
                "potential_threat".to_string()
            }
        } else {
            "safe".to_string()
        };
        
        Ok(ThreatResult {
            is_threat,
            threat_type,
            confidence,
            reasons,
        })
    }
    
    fn extract_features(&self, domain: &str) -> HashMap<String, f64> {
        let mut features = HashMap::new();
        
        // Basic domain features
        features.insert("length".to_string(), domain.len() as f64 / 100.0); // Normalized
        features.insert("subdomain_count".to_string(), domain.matches('.').count() as f64);
        features.insert("hyphen_count".to_string(), domain.matches('-').count() as f64);
        features.insert("digit_count".to_string(), domain.chars().filter(|c| c.is_ascii_digit()).count() as f64);
        
        // Entropy analysis
        let entropy = calculate_domain_entropy(domain);
        features.insert("entropy".to_string(), entropy / 5.0); // Normalized
        features.insert("low_entropy".to_string(), if entropy < 2.5 { 1.0 } else { 0.0 });
        features.insert("high_entropy".to_string(), if entropy > 4.5 { 1.0 } else { 0.0 });
        
        // Pattern analysis
        let (has_suspicious, _) = has_suspicious_pattern(domain);
        features.insert("suspicious_patterns".to_string(), if has_suspicious { 1.0 } else { 0.0 });
        
        let (has_brand_flags, _) = check_brand_impersonation(domain);
        features.insert("brand_impersonation".to_string(), if has_brand_flags { 1.0 } else { 0.0 });
        
        let (has_tld_flags, _) = analyze_tld_suspicion(domain);
        features.insert("suspicious_tld".to_string(), if has_tld_flags { 1.0 } else { 0.0 });
        
        // Length-based features
        features.insert("excessive_length".to_string(), if domain.len() > 30 { 1.0 } else { 0.0 });
        features.insert("short_domain".to_string(), if domain.len() < 4 { 1.0 } else { 0.0 });
        
        // Character analysis
        let vowel_count = domain.chars().filter(|c| "aeiou".contains(*c)).count() as f64;
        let consonant_count = domain.chars().filter(|c| c.is_ascii_alphabetic() && !"aeiou".contains(*c)).count() as f64;
        let vowel_ratio = if domain.len() > 0 { vowel_count / domain.len() as f64 } else { 0.0 };
        features.insert("vowel_ratio".to_string(), vowel_ratio);
        
        // Repeating character analysis
        let mut max_repeat = 0;
        let mut current_repeat = 1;
        let chars: Vec<char> = domain.chars().collect();
        for i in 1..chars.len() {
            if chars[i] == chars[i-1] {
                current_repeat += 1;
            } else {
                max_repeat = max_repeat.max(current_repeat);
                current_repeat = 1;
            }
        }
        max_repeat = max_repeat.max(current_repeat);
        features.insert("max_repeat_chars".to_string(), max_repeat as f64);
        
        features
    }
    
    fn initialize_model_weights() -> HashMap<String, f64> {
        let mut weights = HashMap::new();
        
        // These weights are based on threat analysis patterns
        weights.insert("suspicious_patterns".to_string(), 0.8);
        weights.insert("brand_impersonation".to_string(), 0.9);
        weights.insert("suspicious_tld".to_string(), 0.6);
        weights.insert("low_entropy".to_string(), 0.4);
        weights.insert("high_entropy".to_string(), 0.3);
        weights.insert("excessive_length".to_string(), 0.2);
        weights.insert("hyphen_count".to_string(), 0.1);
        weights.insert("digit_count".to_string(), 0.15);
        weights.insert("max_repeat_chars".to_string(), 0.25);
        
        weights
    }
    
    fn load_initial_threats(threats: &mut HashSet<String>) {
        // Load some known malicious domains (in production, this would come from threat feeds)
        let initial_threats = vec![
            "malware.com",
            "phishing.net", 
            "spam.org",
            "trojan.info",
            "virus.biz",
            "scam.co",
            "fake-bank.com",
            "phish-paypal.tk",
            "malicious-site.ml",
            "dangerous-download.cf",
        ];
        
        for threat in initial_threats {
            threats.insert(threat.to_string());
        }
        
        info!("Loaded {} initial threat domains", threats.len());
    }
    
    fn hash_domain(domain: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(domain.as_bytes());
        hasher.finalize().to_vec()
    }
    
    async fn update_threat_intelligence(client: Client) -> Result<()> {
        // In a real implementation, this would fetch from threat intelligence feeds
        // For now, we'll simulate the update process
        debug!("Updating threat intelligence...");
        
        // This is where you'd integrate with threat intelligence providers like:
        // - VirusTotal
        // - OpenPhish
        // - PhishTank
        // - Custom threat feeds
        
        Ok(())
    }
    
    pub async fn add_threat(&self, domain: String) {
        let domain_hash = Self::hash_domain(&domain);
        
        self.known_threats.write().await.insert(domain.clone());
        self.threat_bloom.write().await.insert(&domain_hash);
        
        info!("Added new threat domain: {}", domain);
    }
    
    pub async fn remove_threat(&self, domain: &str) {
        self.known_threats.write().await.remove(domain);
        // Note: Bloom filters don't support removal, so we'd need to rebuild it
        // or use a counting bloom filter in production
        
        info!("Removed threat domain: {}", domain);
    }
}