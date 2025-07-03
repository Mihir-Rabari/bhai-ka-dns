use crate::errors::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypoCorrection {
    pub original: String,
    pub suggestions: Vec<String>,
    pub confidence: f64,
}

pub struct TypoCorrector {
    common_domains: Vec<String>,
}

impl TypoCorrector {
    pub fn new() -> Self {
        let common_domains = vec![
            "google.com".to_string(),
            "facebook.com".to_string(),
            "twitter.com".to_string(),
            "instagram.com".to_string(),
            "youtube.com".to_string(),
            "amazon.com".to_string(),
            "microsoft.com".to_string(),
            "apple.com".to_string(),
            "netflix.com".to_string(),
            "linkedin.com".to_string(),
        ];

        Self { common_domains }
    }

    pub async fn correct(&self, domain: &str) -> Result<TypoCorrection> {
        let suggestions = self.generate_suggestions(domain).await?;
        let confidence = if suggestions.is_empty() { 0.0 } else { 0.8 };

        Ok(TypoCorrection {
            original: domain.to_string(),
            suggestions,
            confidence,
        })
    }

    async fn generate_suggestions(&self, domain: &str) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();

        for common_domain in &self.common_domains {
            let distance = self.levenshtein_distance(domain, common_domain);
            let max_distance = (domain.len().max(common_domain.len()) / 3).max(1);

            if distance <= max_distance {
                suggestions.push(common_domain.clone());
            }
        }

        // Sort by edit distance
        suggestions.sort_by(|a, b| {
            let dist_a = self.levenshtein_distance(domain, a);
            let dist_b = self.levenshtein_distance(domain, b);
            dist_a.cmp(&dist_b)
        });

        // Take top 3 suggestions
        suggestions.truncate(3);

        Ok(suggestions)
    }

    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }
}

impl Default for TypoCorrector {
    fn default() -> Self {
        Self::new()
    }
}