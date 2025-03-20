use anyhow::Result;
use ndarray::Array1;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Clone)]
struct ErrorEntry {
    error: String,
    fix: String,
    embedding: Vec<f32>,
}

fn load_database() -> Result<Vec<ErrorEntry>> {
    let data = fs::read_to_string("error.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).map_err(|e| anyhow::anyhow!("Failed to parse 25.json: {}", e))
}

fn save_database(database: &[ErrorEntry]) -> Result<()> {
    let json_str = serde_json::to_string_pretty(database)?;
    fs::write("error.json", json_str)?;
    Ok(())
}

fn handle_no_suggestion(
    model: &SentenceEmbeddingsModel,
    database: &mut Vec<ErrorEntry>,
    input: &str,
) -> Result<()> {
    println!("No close match found (similarity < 0.8). Add a suggestion? (yes/no)");
    print!("> ");
    io::stdout().flush()?;
    let mut add_input = String::new();
    io::stdin().read_line(&mut add_input)?;

    if add_input.trim().to_lowercase() == "yes" {
        println!("Enter your suggestion:");
        print!("> ");
        io::stdout().flush()?;
        let mut suggestion = String::new();
        io::stdin().read_line(&mut suggestion)?;
        let suggestion = suggestion.trim().to_string();

        let new_embedding = model
            .encode(&[input])?
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get embedding"))?
            .clone();

        database.push(ErrorEntry {
            error: input.to_string(),
            fix: suggestion,
            embedding: new_embedding,
        });

        save_database(database)?;
        println!("Suggestion added to database!");
    } else {
        println!("No suggestion provided.");
    }

    Ok(())
}

fn cosine_similarity(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let dot_product = a.dot(b);
    let norm_a = a.dot(a).sqrt();
    let norm_b = b.dot(b).sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

fn main() -> Result<()> {
    println!("RustEdge Agent starting.........");

    // Load the pre-trained model
    let model = SentenceEmbeddingsModel::new(SentenceEmbeddingsModelType::AllMiniLmL12V2.into())?;
    println!("AI model loaded successfully!");

    // Load and initialize database
    let mut database = load_database()?;
    if database.is_empty() {
        println!("Warning: error.json is empty or missing. Starting with an empty database.");
    }

    // Compute embeddings for any entries missing them
    let mut updated = false;
    for entry in &mut database {
        if entry.embedding.is_empty() {
            entry.embedding = model
                .encode(&[entry.error.clone()])?
                .get(0)
                .ok_or_else(|| anyhow::anyhow!("Failed to get embedding for '{}'", entry.error))?
                .clone();
            updated = true;
        }
    }
    if updated {
        save_database(&database)?;
        println!("Database updated with new embeddings.");
    }

    // Interactive loop
    println!("RustEdge Agent (AI Edition) ready!");
    loop {
        println!("Enter a Rust error message (or 'quit' to exit):");
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting RustEdge Agent.");
            break;
        }

        let input_embedding = model
            .encode(&[input])?
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get embedding for input"))?
            .clone();

        let input_vec = Array1::from_vec(input_embedding);
        let mut best_similarity = -1.0;
        let mut best_fix = None;

        for entry in &database {
            let entry_vec = Array1::from_vec(entry.embedding.clone());
            let similarity = cosine_similarity(&input_vec, &entry_vec);
            if similarity > best_similarity {
                best_similarity = similarity;
                best_fix = Some(&entry.fix);
            }
        }

        if let Some(fix) = best_fix {
            if best_similarity > 0.8 {
                println!("Suggested fix: {}", fix);
                println!("Confidence: {:.2}%", best_similarity * 100.0);
            } else {
                println!(
                    "Best match similarity: {:.2}% (below 0.8 threshold)",
                    best_similarity * 100.0
                );
                handle_no_suggestion(&model, &mut database, input)?;
            }
        } else {
            println!("No matches found in database.");
            handle_no_suggestion(&model, &mut database, input)?;
        }
    }

    Ok(())
}
