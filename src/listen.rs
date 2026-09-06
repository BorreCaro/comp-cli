use crate::server::listen;
use anyhow::Context;
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
        None => PathBuf::from(format!("problems/{}", normalize_name(&d.name))),
    };
    fs::create_dir_all(&path.join("tests"))?;
    for (i, test) in d.tests.iter().enumerate() {
        fs::write(path.join(&format!("tests/{}.in", i + 1)), &test.input)?;
        fs::write(path.join(&format!("tests/{}.out", i + 1)), &test.output)?;
    }
    if !path.join("main.cpp").exists() {fs::write(path.join("main.cpp"), "")?;}
    Ok(())
}
pub fn cli_listen(path: Option<&Path>, once: bool) -> Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:27121")
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;
    loop {
        if once {
            let output = listen(&server).context("Error while reading request")?;
            handle_problem(&output, path)?;
            return Ok(());
        }
        match listen(&server) {
            Ok(output) => {
                if let Err(e) = handle_problem(&output, path) {
                    eprintln!("Error handling problem: {e}");
                }
            },
            Err(e) => eprintln!("Error while reading request {e}"),
        }
    }
}
