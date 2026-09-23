use anyhow::{Context, Result, bail};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

enum TestVerdict {
    Ok,
    WrongAnswer { out: String },
    RuntimeError,
    TimeLimitExceeded,
}

fn compile(source: &Path, path: &Path) -> Result<()> {
    let comp_out = Command::new("g++")
        .arg(source)
        .arg("-O2")
        .arg("-std=c++23")
        .arg("-o")
        .arg(
            path.join("sol")
                .with_extension(std::env::consts::EXE_EXTENSION),
        )
        .output()
        .context("Error launching g++")?;

    if !(comp_out.status.success()) {
        bail!(String::from_utf8_lossy(&comp_out.stderr).into_owned());
    }
    Ok(())
}
fn compare_outputs(out1: &str, out2: &str) -> bool {
    out1.split_whitespace().eq(out2.split_whitespace())
}

fn print_veredict(veredict: TestVerdict, test_out: &str, err: &str, i: u64) {
    print!("Test {i}: ");
    match veredict {
        TestVerdict::Ok => println!("Ok"),
        TestVerdict::WrongAnswer { out } => {
            println!("Wrong Answer");
            println!("--- Expected");
            print!("{test_out}");
            println!("--- Got");
            println!("{out}");
        }
        TestVerdict::RuntimeError => println!("Runtime Error"),
        TestVerdict::TimeLimitExceeded => println!("Time Limit Exceeded"),
    }
    if !err.trim().is_empty() {
        println!("stderr:\n{}", err.trim());
    }
}
fn get_veredict(child: &mut Child, limit: u64, test_out: &str) -> Result<TestVerdict> {
    let status = match child.wait_timeout(Duration::from_secs(limit)) {
        Ok(statusopt) => match statusopt {
            Some(status) => status,
            None => {
                child.kill().context("Failed to kill child process")?;
                child.wait().context("Failed to wait for child process")?;
                return Ok(TestVerdict::TimeLimitExceeded);
            }
        },
        Err(_) => bail!("Failed to wait for child process"),
    };
    if !status.success() {
        return Ok(TestVerdict::RuntimeError);
    }
    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout
            .read_to_string(&mut output)
            .context("Failed to read stdout")?;
    };
    if compare_outputs(&output, test_out) {
        Ok(TestVerdict::Ok)
    } else {
        Ok(TestVerdict::WrongAnswer { out: output })
    }
}
fn manage_test(path: &Path, tests_dir: &Path, i: u64, limit: u64) -> Result<()> {
    let test_in = File::open(tests_dir.join(format!("{i}.in")))
        .context(format!("Couldn't open {}/{i}.in", tests_dir.display()))?;
    let test_out = fs::read_to_string(tests_dir.join(format!("{i}.out")))
        .context(format!("Couldn't read {}/{i}.out", tests_dir.display()))?;
    let exe_path = path
        .join("sol")
        .with_extension(std::env::consts::EXE_EXTENSION);
    let mut child = Command::new(&exe_path)
        .stdin(test_in)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context(format!("Failed to launch {}", exe_path.display()))?;
    let mut err_output = String::new();
    let verdict = get_veredict(&mut child, limit, &test_out)?;
    if let Some(mut stderr) = child.stderr.take() {
        stderr
            .read_to_string(&mut err_output)
            .context("Failed to read stderr")?;
    }
    print_veredict(verdict, &test_out, &err_output, i);
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
            println!("Ran {} tests", i - 1);
            break;
        }
        manage_test(&path, &tests_dir, i, limit)?;
        i += 1;
    }
    Ok(())
}
