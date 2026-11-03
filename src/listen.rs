use crate::server::listen;
use anyhow::{Context, Result};
use dirs::config_dir;
use serde::Deserialize;
use std::fs;
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};

static DEFAULT_TEMPLATE: &str = include_str!("../assets/default_template.cpp");

#[derive(Deserialize)]
struct Test {
    input: String,
    output: String,
}
#[derive(Deserialize)]
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
fn load_template() -> Result<String> {
    let Some(conf_path) = config_dir() else {
        return Ok(DEFAULT_TEMPLATE.to_string());
    };
    let template_path = conf_path.join("comp-cli").join("template.cpp");
    match fs::read_to_string(&template_path) {
        Ok(template) => Ok(template),
        Err(e) if e.kind() == NotFound => Ok(DEFAULT_TEMPLATE.to_string()),
        Err(e) => Err(e).with_context(|| {
            format!(
                "Failed to read template file at {}",
                template_path.display()
            )
        }),
    }
}
fn handle_problem(output: &str, path: Option<&Path>, template: &str) -> Result<()> {
    let d = serde_json::from_str::<Data>(output)?;
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from(normalize_name(&d.name)),
    };
    fs::create_dir_all(path.join("tests"))?;
    for (i, test) in (1..).zip(d.tests.iter()) {
        fs::write(path.join("tests").join(format!("{i}.in")), &test.input)
            .with_context(|| format!("{}/tests/{i}.in", path.display()))?;

        fs::write(path.join("tests").join(format!("{i}.out")), &test.output)
            .with_context(|| format!("{}/tests/{i}.out", path.display()))?;
    }
    if !path.join("main.cpp").exists() {
        fs::write(path.join("main.cpp"), template)
            .with_context(|| format!("Couldn't write main.cpp at {}", path.display()))?;
    }
    Ok(())
}
pub fn cli_listen(path: Option<&Path>, once: bool) -> Result<()> {
    let server = tiny_http::Server::http("127.0.0.1:27121")
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;
    let template = load_template()?;
    if once {
        let output = listen(&server).context("Error while reading request")?;
        return handle_problem(&output, path, &template);
    }
    loop {
        match listen(&server) {
            Ok(output) => {
                if let Err(e) = handle_problem(&output, path, &template) {
                    eprintln!("Error handling problem: {e}");
                }
            }
            Err(e) => eprintln!("Error while reading request {e}"),
        }
    }
}
