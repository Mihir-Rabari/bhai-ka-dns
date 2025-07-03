use crate::errors::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAnalysis {
    pub domain: String,
    pub category: String,
    pub risk_score: f64,
    pub features: DomainFeatures,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainFeatures {
    pub length: usize,
    pub subdomain_count: usize,
    pub has_suspicious_keywords: bool,
    pub entropy: f64,
    pub age_estimate: Option<u32>,
    pub tld: String,
}

pub struct DomainAnalyzer;

impl DomainAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, domain: &str) -> Result<DomainAnalysis> {
        let features = self.extract_features(domain).await?;
        let risk_score = self.calculate_risk_score(&features).await?;
        let category = self.categorize_domain(&features, risk_score).await?;
        let recommendations = self.generate_recommendations(&features, risk_score).await?;

        Ok(DomainAnalysis {
            domain: domain.to_string(),
            category,
            risk_score,
            features,
            recommendations,
        })
    }

    async fn extract_features(&self, domain: &str) -> Result<DomainFeatures> {
        let parts: Vec<&str> = domain.split('.').collect();
        let tld = parts.last().unwrap_or("").to_string();
        let subdomain_count = if parts.len() > 2 { parts.len() - 2 } else { 0 };
        
        let suspicious_keywords = vec!["phishing", "scam", "fake", "malware", "virus"];
        let has_suspicious_keywords = suspicious_keywords.iter()
            .any(|keyword| domain.to_lowercase().contains(keyword));

        let entropy = self.calculate_entropy(domain);

        Ok(DomainFeatures {
            length: domain.len(),
            subdomain_count,
            has_suspicious_keywords,
            entropy,
            age_estimate: None, // Would require external API call
            tld,
        })
    }

    async fn calculate_risk_score(&self, features: &DomainFeatures) -> Result<f64> {
        let mut score = 0.0;

        // Length-based scoring
        if features.length > 50 {
            score += 0.3;
        } else if features.length < 5 {
            score += 0.2;
        }

        // Subdomain scoring
        if features.subdomain_count > 3 {
            score += 0.4;
        }

        // Suspicious keywords
        if features.has_suspicious_keywords {
            score += 0.8;
        }

        // Entropy scoring (high entropy = random-looking domain)
        if features.entropy > 3.5 {
            score += 0.3;
        }

        // TLD scoring
        let suspicious_tlds = vec!["tk", "ml", "ga", "cf"];
        if suspicious_tlds.contains(&features.tld.as_str()) {
            score += 0.4;
        }

        Ok(score.min(1.0))
    }

    async fn categorize_domain(&self, _features: &DomainFeatures, risk_score: f64) -> Result<String> {
        let category = if risk_score > 0.7 {
            "High Risk"
        } else if risk_score > 0.4 {
            "Medium Risk"
        } else {
            "Low Risk"
        };

        Ok(category.to_string())
    }

    async fn generate_recommendations(&self, features: &DomainFeatures, risk_score: f64) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if risk_score > 0.5 {
            recommendations.push("Consider blocking this domain".to_string());
        }

        if features.has_suspicious_keywords {
            recommendations.push("Domain contains suspicious keywords".to_string());
        }

        if features.subdomain_count > 3 {
            recommendations.push("High number of subdomains detected".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Domain appears safe".to_string());
        }

        Ok(recommendations)
    }

    fn calculate_entropy(&self, domain: &str) -> f64 {
        use std::collections::HashMap;
        
        let mut char_counts = HashMap::new();
        let total_chars = domain.len() as f64;

        for c in domain.chars() {
            *char_counts.entry(c).or_insert(0.0) += 1.0;
        }

        let mut entropy = 0.0;
        for count in char_counts.values() {
            let probability = count / total_chars;
            entropy -= probability * probability.log2();
        }

        entropy
    }
}

impl Default for DomainAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}