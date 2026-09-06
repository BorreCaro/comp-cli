use anyhow::{Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;
pub fn run(path: Option<&Path>) -> Result<()> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    if !(path.join("tests").is_dir()) {
        bail!("Couldn't find tests directory");
    }
    let output = Command::new("g++")
        .arg(path.join("main.cpp"))
        .arg("-O2")
        .arg("-std=c++23")
        .arg("-o")
        .arg(path.join("sol"))
        .output()?;
    if !output.status.success() {
        bail!(String::from_utf8(output.stderr)?);
    }

    Ok(())
}
