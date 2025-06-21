use crate::memory::semantic::object::ObjectCluster;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub static SEMANTIC_GRAPH: Lazy<Mutex<LatentGraph>> = Lazy::new(|| Mutex::new(LatentGraph::new()));

#[derive(Clone)]
pub struct LatentGraph {
    pub clusters: HashMap<String, ObjectCluster>,
    pub relations: Vec<(String, String, String)>, // (source, relation, target)
}

impl LatentGraph {
    pub fn new() -> Self {
        Self {
            clusters: HashMap::new(),
            relations: vec![],
        }
    }

    pub fn add_cluster(&mut self, id: String, cluster: ObjectCluster) {
        self.clusters.insert(id, cluster);
    }

    pub fn add_relation(&mut self, source: &str, relation: &str, target: &str) {
        self.relations
            .push((source.to_string(), relation.to_string(), target.to_string()));
    }

    pub fn find_nearest(&self, embedding: &[f32]) -> Option<(String, f32)> {
        self.clusters
            .iter()
            .filter_map(|(id, cluster)| {
                let sim = cosine_similarity(&cluster.embedding, embedding);
                Some((id.clone(), sim))
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}

impl Default for LatentGraph {
    fn default() -> Self {
        LatentGraph::new()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
