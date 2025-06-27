use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub async fn embed_text(text: &str) -> Result<Vec<f32>, String> {
    let mut cmd = Command::new("./llama.cpp/build/bin/llama-embedding");
    cmd.arg("-m")
        .arg("models/7B/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf")
        .arg("-p")
        .arg(text)
        .stdout(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("spawn error: {}", e))?;
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let reader = BufReader::new(stdout);

    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if let Some(embedding_data) = line.strip_prefix("embedding ") {
            if let Some(values_str) = embedding_data.split(':').nth(1) {
                let values = values_str
                    .split_whitespace()
                    .filter_map(|s| s.parse::<f32>().ok());
                vec.extend(values);
            }
        }
    }

    if vec.is_empty() {
        Err("no embedding found".into())
    } else {
        Ok(vec)
    }
}
