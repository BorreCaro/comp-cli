use anyhow::{Context, Result, bail};
use std::fs::{self, File};
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;
fn compile(source: &Path, path: &Path) -> Result<()> {
    let comp_out = Command::new("g++")
        .arg(source)
        .arg("-O2")
        .arg("-std=c++23")
        .arg("-o")
        .arg(path.join("sol").with_extension(std::env::consts::EXE_EXTENSION))
        .output().context("Error launching g++")?;

    if !(comp_out.status.success()) {
        bail!(String::from_utf8_lossy(&comp_out.stderr).into_owned());
    }
    Ok(())
}
fn compare_outputs(out1: &str, out2: &str) -> bool {
    out1.split_whitespace().eq(out2.split_whitespace())
}
fn manage_test(path: &Path, tests_dir: &Path, i: u32, limit: u64) -> Result<()> {
    let test_in = File::open(tests_dir.join(format!("{i}.in"))).context(format!("Couldn't open {}/{i}.in", tests_dir.display()))?;
    let mut child = Command::new(path.join("sol").with_extension(std::env::consts::EXE_EXTENSION)).stdin(test_in).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().context(format!("Failed to launch {}", path.join("sol").with_extension(std::env::consts::EXE_EXTENSION).display(), ))?;
    let status = match child.wait_timeout(Duration::from_secs(limit)) {
    Ok(statusopt) => {
        match statusopt {
            Some(status) => status,
            None => {
            child.kill().context("Failed to kill child process")?;
            child.wait().context("Failed to wait for child process")?;
            println!("Test {i}: TLE");
            return Ok(());
            },
        }
    },
        Err(_) => bail!("Failed to wait for child process"),
    };
    let mut output = String::new();
    let mut err_output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut output).context("Failed to read stdout")?;
    }
    if let Some(mut stderr) = child.stderr.take() {
        stderr.read_to_string(&mut err_output).context("Failed to read stderr")?;
    }
    if !status.success() {
        println!("Test {i}: RTE");
        if !err_output.trim().is_empty() {println!("stderr:\n{}", err_output.trim());}
        return Ok(());
    }
    let test_out = fs::read_to_string(tests_dir.join(format!("{i}.out"))).context(format!("Couldn't read {}/{i}.out", tests_dir.display()))?;
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

    if !err_output.trim().is_empty() {println!("stderr:\n{}", err_output.trim());}
    Ok(())
}
pub fn run(path: Option<&Path>, limit: u64) -> Result<()> {
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
        manage_test(&path, &tests_dir, i, limit)?;
        i += 1;
    }
    Ok(())
}
