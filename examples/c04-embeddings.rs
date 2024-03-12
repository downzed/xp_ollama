use std::{fs, path::Path};

use simple_fs::{ensure_dir, read_to_string, save_be_f64, save_json};
use xp_ollama::{consts::EMBEDDING_MODEL, Result};

use ollama_rs::Ollama;

const MOCK_DIR: &str = "_mock-data";
const C04_DIR: &str = ".c04-data";

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    ensure_dir(C04_DIR)?;
    let txt = read_to_string(Path::new(MOCK_DIR).join("embeddings.txt"))?;

    // Splits
    let splits = simple_text_splitter(&txt, 500)?;

    println!(">> [debug: splits counts] {}", splits.len());

    for (i, seg) in splits.into_iter().enumerate() {
        println!();
        let file_name = format!("c04-embeddings-{:0>2}.txt", i);
        let file_path = Path::new(C04_DIR).join(file_name);
        fs::write(file_path, &seg)?;

        println!(">> [debug: text length] {}", txt.len());

        let res = ollama
            .generate_embeddings(EMBEDDING_MODEL.to_string(), seg, None)
            .await?;

        println!(">> [debug: embeddings size] {}", res.embeddings.len());
        let file_name = format!("c04-embeddings-{:0>2}.json", i);
        save_json(Path::new(C04_DIR).join(file_name), &res.embeddings)?;

        let file_name = format!("c04-embeddings-{:0>2}.be-f64.bin", i);
        let file_path = Path::new(C04_DIR).join(file_name);
        save_be_f64(&file_path, &res.embeddings)?;
    }

    Ok(())
}

// silly text splitter
// no vec search, just for learning ollama embeddings api
fn simple_text_splitter(txt: &str, num_of_char: u32) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut last = 0;
    let mut count = 0;

    for (idx, _) in txt.char_indices() {
        count += 1;
        if count == num_of_char {
            result.push(&txt[last..idx + 1]);
            last = idx + 1;
            count = 0;
        }
    }

    // remaining characters
    if last < txt.len() {
        result.push(&txt[last..]);
    }

    Ok(result.into_iter().map(String::from).collect())
}