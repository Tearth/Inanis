use common::time::DateTime;
use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rustc-env=DATE={}", date());
    println!("cargo:rustc-env=COMPILER={}", compiler());
    println!("cargo:rustc-env=TARGET={}", target());
    println!("cargo:rustc-env=PROFILE={}", profile());

    #[cfg(feature = "syzygy")]
    build_fathom();

    #[cfg(feature = "syzygy")]
    generate_fathom_bindings();
}

fn date() -> String {
    let datetime = DateTime::now();
    format!("{}-{:0>2}-{:0>2}", datetime.year, datetime.month, datetime.day)
}

fn compiler() -> String {
    let output = Command::new("rustc").args(["--version", "--verbose"]).current_dir(env!("CARGO_MANIFEST_DIR")).output();

    match output {
        Ok(output) => {
            let output_content = String::from_utf8_lossy(&output.stdout).to_string();
            let lines = output_content.split('\n').collect::<Vec<&str>>();
            let rustc_version = lines[0].trim();
            let llvm_version = lines[lines.len() - 2].trim();

            format!("{}, {}", rustc_version, llvm_version)
        }
        Err(_) => String::from("ERROR"),
    }
}

fn target() -> String {
    env::var("TARGET").unwrap_or(String::from("ERROR"))
}

fn profile() -> String {
    let mut features = Vec::new();
    let profile = env::var("PROFILE").unwrap_or(String::from("ERROR"));

    if cfg!(feature = "dev") {
        features.push("dev");
    }

    if cfg!(feature = "syzygy") {
        features.push("syzygy");
    }

    if features.is_empty() {
        features.push("none");
    }

    format!("{} ({})", profile, features.join(", "))
}

#[cfg(feature = "syzygy")]
fn build_fathom() {
    let cc = &mut cc::Build::new();
    cc.file("./deps/fathom/src/tbprobe.c");
    cc.include("./deps/fathom/src/");
    cc.define("_CRT_SECURE_NO_WARNINGS", None);

    // MSVC doesn't support stdatomic.h, so use clang on Windows
    if env::consts::OS == "windows" {
        cc.compiler("clang");
    }

    cc.compile("fathom");
}

#[cfg(feature = "syzygy")]
fn generate_fathom_bindings() {
    let bindings = bindgen::Builder::default()
        .header("./deps/fathom/src/tbprobe.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .generate()
        .unwrap();

    bindings.write_to_file("./src/tablebases/syzygy/bindings.rs").unwrap();
}
