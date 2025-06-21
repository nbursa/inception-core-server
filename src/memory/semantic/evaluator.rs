use crate::memory::semantic::object::{ObjectCluster, Affect, AffectScore};
use std::collections::HashSet;

pub struct SemanticEvaluator;

impl SemanticEvaluator {
    pub fn assign_tags(description: &str) -> Vec<String> {
        description
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn calculate_affect(tags: &[String]) -> AffectScore {
        let mut score = 0.0;
        for tag in tags {
            match tag.as_str() {
                "pain" => score -= 1.0,
                "sharp" => score -= 0.7,
                "beautiful" => score += 0.9,
                "safe" => score += 0.6,
                _ => {}
            }
        }

        let class = if score > 0.3 {
            Affect::Pleasant
        } else if score < -0.3 {
            Affect::Unpleasant
        } else {
            Affect::Unknown
        };

        AffectScore { value: score, class }
    }

    pub fn reclassify_if_stable(cluster: &mut ObjectCluster) {
        let tag_set: HashSet<_> = cluster.tags.iter().cloned().collect();
        if tag_set.len() >= 4 && cluster.affect.value.abs() >= 0.8 {
            cluster.known = true;
            if cluster.name.starts_with("x_") {
                cluster.name = "knife".to_string(); // Placeholder
            }
        }
    }
}