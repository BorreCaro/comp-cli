use crate::server::listen;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
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
        .chars()
        .skip(3) // Working on codeforces, anything else not my problem
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
fn handle_problem(output: &str, path: Option<&Path>) -> Result<()> {
    let d = serde_json::from_str::<Data>(output)?;
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from(normalize_name(&d.name).to_string()),
    };
    fs::create_dir_all(path.join("tests"))?;
    for (i, test) in (1..).zip(d.tests.iter()) {
        fs::write(path.join("tests").join(format!("{i}.in")), &test.input)
            .with_context(|| format!("{}/tests/{i}.in", path.display()))?;

        fs::write(path.join("tests").join(format!("{i}.out")), &test.output)
            .with_context(|| format!("{}/tests/{i}.out", path.display()))?;
    }
    if !path.join("main.cpp").exists() {
        fs::write(path.join("main.cpp"), "").context("Couldn't write main.cpp")?;
    }
    Ok(())
}
pub fn cli_listen(path: Option<&Path>, once: bool) -> Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:27121")
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;
    if once {
        let output = listen(&server).context("Error while reading request")?;
        return handle_problem(&output, path);
    }
    loop {
        match listen(&server) {
            Ok(output) => {
                if let Err(e) = handle_problem(&output, path) {
                    eprintln!("Error handling problem: {e}");
                }
            }
            Err(e) => eprintln!("Error while reading request {e}"),
        }
    }
}
