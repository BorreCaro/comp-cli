use crate::server::listen;
use std::fs;
use serde::{Deserialize, Serialize};
use tiny_http;
mod server;
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
    name.to_lowercase().replace(" ","_").chars().skip(3).collect()
}
fn main() {
    let Ok(server) = tiny_http::Server::http("127.0.0.1:27121") else {
        eprintln!("Couldn't lift server");
        return;
    };
    loop {
        let output = listen(&server);
        match output {
            Ok(output) => {
                let d = match serde_json::from_str::<Data>(&output) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Couldn't parse the string: {e}");
                        continue;
                    }
                };
                let path = &format!("problems/{}/tests", normalize_name(&d.name));
                if std::path::Path::new(path).is_dir() {
                    println!("Directory already exists");
                    println!("Do you want to overwrite it?: Y/n");
                    let mut input = String::new();
                    if let Err(e) = std::io::stdin().read_line(&mut input) {
                        eprintln!("Error while reading input {e}");
                        println!("Skipping");
                        continue;
                    }
                    match input.trim().chars().next() {
                        Some('n')|Some('N') => {println!("Skipping"); continue},
                        _ => println!("Overwriting"),
                    }
                }
                if let Err(e) = fs::create_dir_all(path) {eprintln!("Error: {e}"); continue;}

                for (i, test) in d.tests.iter().enumerate() {
                    if let Err(e) = fs::write(&format!("{path}/{}.in", i+1), &test.input) {
                        eprintln!("Error while writing in: {e}");
                        continue;
                    }
                    if let Err(e) = fs::write(&format!("{path}/{}.out", i+1), &test.output) {
                        eprintln!("Error while writing out: {e}");
                        continue;
                    }
                }
            },
            Err(e) => eprintln!("Error {e}"),
        }
    }
}
