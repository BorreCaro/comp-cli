use anyhow::{Result, bail};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
fn compile(source: &Path, path: &Path) -> Result<()> {
    let comp_out = Command::new("g++")
        .arg(source)
        .arg("-O2")
        .arg("-std=c++23")
        .arg("-o")
        .arg(path.join("sol"))
        .output()?;
    if !(comp_out.status.success()) {
        bail!(String::from_utf8(comp_out.stderr)?);
    }
    Ok(())
}
fn compare_outputs(out1: &str, out2: &str) -> bool {
    out1.split_whitespace().eq(out2.split_whitespace())
}
fn manage_test(path: &Path, i: &u32) -> Result<()> {
    let test_in = File::open(path.join(format!("tests/{i}.in")))?;
    let output = Command::new(path.join("sol")).stdin(test_in).output()?;
    if !output.status.success() {
        println!("Test {i}: RTE");
        return Ok(());
    }
    let output = String::from_utf8(output.stdout)?;
    let test_out = fs::read_to_string(path.join(format!("tests/{i}.out")))?;
    print!("Test {i}: ");
    println!(
        "{}",
        if compare_outputs(&output, &test_out) {
            "OK"
        } else {
            "WA"
        }
    );
    Ok(())
}
pub fn run(path: Option<&Path>) -> Result<()> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    if !(path.join("tests").is_dir()) {
        bail!("Couldn't find tests directory");
    }
    // TODO: source code arg
    let source = path.join("main.cpp");
    if !source.is_file() {
        bail!("Couldn't find {}", source.display());
    }
    compile(&source, &path)?;

    let mut i = 1;
    loop {
        if !(path.join(format!("tests/{i}.in")).exists()
            && path.join(format!("tests/{i}.out")).exists())
        {
            break;
        }
        manage_test(&path, &i)?;
        i += 1;
    }
    if i == 1 {
        bail!("No tests found")
    }
    Ok(())
}
