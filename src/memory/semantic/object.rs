use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Affect {
    Pleasant,
    Unpleasant,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct AffectScore {
    pub value: f32,
    pub class: Affect,
}

impl AffectScore {
    pub fn from_value(val: f32) -> Self {
        let class = if val > 0.3 {
            Affect::Pleasant
        } else if val < -0.3 {
            Affect::Unpleasant
        } else {
            Affect::Unknown
        };
        Self { value: val, class }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectCluster {
    pub name: String,
    pub embedding: Vec<f32>,
    pub tags: Vec<String>,
    pub affect: AffectScore,
    pub known: bool,
}

impl ObjectCluster {
    pub fn new(name: String, embedding: Vec<f32>, tags: Vec<String>) -> Self {
        Self {
            name,
            embedding,
            tags,
            affect: AffectScore::from_value(0.0),
            known: false,
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }

    pub fn update_affect(&mut self, delta: f32) {
        self.affect.value += delta;
        self.affect = AffectScore::from_value(self.affect.value);
    }
}

#[derive(Debug, Clone)]
pub struct LatentGraph {
    pub clusters: HashMap<String, ObjectCluster>,
    pub relations: Vec<(String, String, String)>, // (source, relation, target)
}

impl LatentGraph {
    pub fn new() -> Self {
        Self {
            clusters: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_cluster(&mut self, cluster: ObjectCluster) {
        self.clusters.insert(cluster.name.clone(), cluster);
    }

    pub fn add_relation(&mut self, source: &str, relation: &str, target: &str) {
        self.relations
            .push((source.to_string(), relation.to_string(), target.to_string()));
    }

    pub fn get_nearest(&self, embedding: &[f32]) -> Option<&ObjectCluster> {
        self.clusters.values().min_by(|a, b| {
            let da = cosine_distance(&a.embedding, embedding);
            let db = cosine_distance(&b.embedding, embedding);
            da.partial_cmp(&db).unwrap()
        })
    }
}

fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    1.0 - (dot / (norm_a * norm_b + 1e-6))
}
