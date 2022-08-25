use std::env;
use std::process::Command;
use time::format_description;
use time::OffsetDateTime;

fn main() {
    println!("cargo:rustc-env=HASH={}", hash());
    println!("cargo:rustc-env=DATE={}", date());
    println!("cargo:rustc-env=COMPILER={}", compiler());

    build_dependencies();
    generate_bindings();
}

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
    let time = OffsetDateTime::now_utc();
    let format = format_description::parse("[day]-[month]-[year]").unwrap();

    time.format(&format).unwrap()
}

fn compiler() -> String {
    let output = Command::new("rustc").arg("--version").current_dir(env!("CARGO_MANIFEST_DIR")).output();

    match output {
        Ok(v) => String::from_utf8_lossy(&v.stdout).to_string(),
        Err(_) => "ERROR".to_string(),
    }
}

fn build_dependencies() {
    #[cfg(feature = "syzygy")]
    build_fathom();
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

fn generate_bindings() {
    #[cfg(all(feature = "bindgen", feature = "syzygy"))]
    generate_fathom_bindings();
}

#[cfg(all(feature = "bindgen", feature = "syzygy"))]
fn generate_fathom_bindings() {
    let bindings = bindgen::Builder::default()
        .header("./deps/fathom/src/tbprobe.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .unwrap();

    bindings.write_to_file("./src/tablebases/syzygy/bindings.rs").unwrap();
}
