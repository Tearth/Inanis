extern crate chrono;

use chrono::prelude::*;
use std::env;
use std::process::Command;

fn hash() -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%H")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();

    match output {
        Ok(v) => String::from_utf8_lossy(&v.stdout).to_string(),
        Err(_) => "ERROR".to_string(),
    }
}

fn date() -> String {
    Utc::now().format("%d-%m-%Y").to_string()
}

fn main() {
    println!("cargo:rustc-env=HASH={}", hash());
    println!("cargo:rustc-env=DATE={}", date());
}
