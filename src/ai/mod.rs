use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, warn};
use once_cell::sync::Lazy;

use crate::errors::{AppError, Result};

pub mod threat_detection;
pub mod domain_analysis;
pub mod typo_correction;

use threat_detection::ThreatDetector;
use domain_analysis::DomainAnalyzer;
use typo_correction::TypoCorrector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResult {
    pub is_threat: bool,
    pub threat_type: String,
    pub confidence: f64,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAnalysis {
    pub security_score: f64,
    pub trust_level: String,
    pub category: String,
    pub flags: Vec<String>,
    pub recommendations: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypoSuggestion {
    pub suggestion: Option<String>,
    pub confidence: f64,
    pub reasoning: String,
}

pub struct AIEngine {
    threat_detector: ThreatDetector,
    domain_analyzer: DomainAnalyzer,
    typo_corrector: TypoCorrector,
}

impl AIEngine {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            threat_detector: ThreatDetector::new().await?,
            domain_analyzer: DomainAnalyzer::new(),
            typo_corrector: TypoCorrector::new(),
        })
    }
    
    pub async fn detect_threat(&self, domain: &str) -> Result<ThreatResult> {
        self.threat_detector.analyze(domain).await
    }
    
    pub async fn analyze_domain(&self, domain: &str) -> Result<DomainAnalysis> {
        self.domain_analyzer.analyze(domain).await
    }
    
    pub async fn suggest_correction(&self, domain: &str) -> Result<TypoSuggestion> {
        self.typo_corrector.suggest(domain).await
    }
}

// Common AI patterns and utilities
static SUSPICIOUS_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r".*-secure-.*\.com$").unwrap(),
        Regex::new(r".*paypal.*\.tk$").unwrap(),
        Regex::new(r".*bank.*\.ml$").unwrap(),
        Regex::new(r".*[0-9]{5,}.*\.com$").unwrap(),
        Regex::new(r".*-verification-.*").unwrap(),
        Regex::new(r".*-update-.*").unwrap(),
        Regex::new(r".*-login-.*").unwrap(),
    ]
});

static BRAND_KEYWORDS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "google", "facebook", "amazon", "microsoft", "apple", "netflix",
        "paypal", "ebay", "instagram", "twitter", "linkedin", "youtube",
        "bank", "wells", "chase", "citi", "hsbc", "santander"
    ]
});

static SUSPICIOUS_TLDS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![".tk", ".ml", ".cf", ".ga", ".icu", ".top", ".click", ".download"]
});

pub fn calculate_domain_entropy(domain: &str) -> f64 {
    let mut char_counts = HashMap::new();
    let total_chars = domain.len() as f64;
    
    for c in domain.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    
    -char_counts.values()
        .map(|&count| {
            let p = count as f64 / total_chars;
            p * p.log2()
        })
        .sum::<f64>()
}

pub fn has_suspicious_pattern(domain: &str) -> (bool, Vec<String>) {
    let mut flags = Vec::new();
    let domain_lower = domain.to_lowercase();
    
    for pattern in SUSPICIOUS_PATTERNS.iter() {
        if pattern.is_match(&domain_lower) {
            flags.push(format!("Matches suspicious pattern: {}", pattern.as_str()));
        }
    }
    
    (!flags.is_empty(), flags)
}

pub fn check_brand_impersonation(domain: &str) -> (bool, Vec<String>) {
    let mut flags = Vec::new();
    let domain_lower = domain.to_lowercase();
    
    for &brand in BRAND_KEYWORDS.iter() {
        if domain_lower.contains(brand) && !domain_lower.ends_with(".com") {
            flags.push(format!("Potential {} impersonation with suspicious TLD", brand));
        }
    }
    
    (!flags.is_empty(), flags)
}

pub fn analyze_tld_suspicion(domain: &str) -> (bool, Vec<String>) {
    let mut flags = Vec::new();
    let domain_lower = domain.to_lowercase();
    
    for &tld in SUSPICIOUS_TLDS.iter() {
        if domain_lower.ends_with(tld) {
            flags.push(format!("Uses suspicious TLD: {}", tld));
        }
    }
    
    (!flags.is_empty(), flags)
}

pub fn calculate_security_score(
    entropy: f64,
    suspicious_patterns: &[String],
    brand_flags: &[String],
    tld_flags: &[String],
    additional_factors: &HashMap<String, f64>,
) -> f64 {
    let mut score = 100.0;
    
    // Entropy-based scoring
    if entropy < 2.5 {
        score -= 15.0; // Very low entropy (repeated chars)
    } else if entropy > 4.5 {
        score -= 10.0; // Very high entropy (random chars)
    }
    
    // Pattern-based deductions
    score -= suspicious_patterns.len() as f64 * 25.0;
    score -= brand_flags.len() as f64 * 30.0;
    score -= tld_flags.len() as f64 * 20.0;
    
    // Additional factors
    for (factor, weight) in additional_factors {
        match factor.as_str() {
            "long_domain" => score -= weight,
            "many_hyphens" => score -= weight,
            "numeric_heavy" => score -= weight,
            "punycode" => score -= weight * 2.0,
            _ => {}
        }
    }
    
    score.max(0.0).min(100.0)
}