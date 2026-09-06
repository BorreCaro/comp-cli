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
        .arg(path.join("sol").with_extension(std::env::consts::EXE_EXTENSION))
        .output()?;

    if !(comp_out.status.success()) {
        bail!(String::from_utf8_lossy(&comp_out.stderr).into_owned());
    }
    Ok(())
}
fn compare_outputs(out1: &str, out2: &str) -> bool {
    out1.split_whitespace().eq(out2.split_whitespace())
}
fn manage_test(path: &Path, tests_dir: &Path, i: u32) -> Result<()> {
    let test_in = File::open(tests_dir.join(format!("{i}.in")))?;
    let output = Command::new(path.join("sol").with_extension(std::env::consts::EXE_EXTENSION)).stdin(test_in).output()?;
    if !output.status.success() {
        println!("Test {i}: RTE");
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    let output = String::from_utf8_lossy(&output.stdout);
    let test_out = fs::read_to_string(tests_dir.join(format!("{i}.out")))?;
    print!("Test {i}: ");
    if compare_outputs(&output, &test_out){
        println!("OK");
    }
    else {
        println!("WA");
        println!("--- Expected");
        print!("{test_out}");
        println!("--- Got");
        println!("{output}");
    }
    Ok(())
}
pub fn run(path: Option<&Path>) -> Result<()> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    let tests_dir = path.join("tests");
    if !tests_dir.is_dir() {
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
        if !(tests_dir.join(format!("{i}.in")).exists()
            && tests_dir.join(format!("{i}.out")).exists())
        {
            println!("Ran {} tests", i-1);
            break;
        }
        manage_test(&path, &tests_dir, i)?;
        i += 1;
    }
    Ok(())
}
