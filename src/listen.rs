use crate::server::listen;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use tiny_http;
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
fn handle_problem(output: &str) -> Result<()>{
    let d = serde_json::from_str::<Data>(&output)?;
    let path = &format!("problems/{}/tests", normalize_name(&d.name));
    if std::path::Path::new(path).is_dir() {
        println!("Directory already exists");
        println!("Do you want to overwrite it?: Y/n");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().chars().next() {
            Some('n') | Some('N') => {
                println!("Skipping");
                return Ok(());
            }
            _ => println!("Overwriting"),
        }
    }
    fs::create_dir_all(path)?;

    for (i, test) in d.tests.iter().enumerate() {
        fs::write(&format!("{path}/{}.in", i + 1), &test.input)?;
        fs::write(&format!("{path}/{}.out", i + 1), &test.output)?;
    }
    Ok(())
}
pub fn cli_listen() -> Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:27121")
        .map_err(|e| anyhow::anyhow!("Couldn't lift server: {}", e))?;
    loop {
        let output =
            listen(&server).map_err(|e| anyhow::anyhow!("Error while reading request: {}", e))?;
        if let Err(e) = handle_problem(&output) {
            eprintln!("Error handling problem: {e}");
        }
    }
}
