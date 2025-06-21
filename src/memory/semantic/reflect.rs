use crate::memory::semantic::latent_graph::LatentGraph;

pub fn reflect(graph: &LatentGraph, object_id: &str) -> Vec<(String, f32)> {
    if let Some(reference) = graph.clusters.get(object_id) {
        let mut results = graph
            .clusters
            .iter()
            .filter_map(|(id, cluster)| {
                if id == object_id {
                    return None;
                }
                let sim = cosine_similarity(&reference.embedding, &cluster.embedding);
                Some((id.clone(), sim))
            })
            .collect::<Vec<_>>();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(5);
        results
    } else {
        vec![]
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
