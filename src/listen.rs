use crate::server::listen;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use tiny_http;
use std::path::PathBuf;
use std::path::Path;
#[derive(Serialize, Deserialize)]
struct Test {
    input: String,
    output: String,
}
#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    tests: Vec<Test>,
}
fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .chars()
        .skip(3)
        .collect()
}
fn handle_problem(output: &str, path: Option<&Path>) -> Result<()>{
    let d = serde_json::from_str::<Data>(&output)?;
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from(format!("problems/{}/tests", normalize_name(&d.name))),
    };
    fs::create_dir_all(&path)?;
    for (i, test) in d.tests.iter().enumerate() {
        fs::write(path.join(&format!("{}.in", i + 1)), &test.input)?;
        fs::write(path.join(&format!("{}.out", i + 1)), &test.output)?;
    }
    Ok(())
}
pub fn cli_listen(path: Option<&Path>) -> Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:27121")
        .map_err(|e| anyhow::anyhow!("Couldn't lift server: {}", e))?;
    loop {
        let output =
            listen(&server).map_err(|e| anyhow::anyhow!("Error while reading request: {}", e))?;
        if let Err(e) = handle_problem(&output, path) {
            eprintln!("Error handling problem: {e}");
        }
    }
}
