use std::env;
use std::path::PathBuf;
use std::fs::canonicalize;
use std::process::Command;

use once_cell::sync::Lazy;

static OUT_DIR: Lazy<String> = Lazy::new(|| {
    env::var("OUT_DIR").unwrap()
});

static OUT_PATH: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from((*OUT_DIR).clone())
});

fn main() {
    let build_dir_path = canonicalize(&(*OUT_PATH))
        .expect("Failed to canonicalize OUT_PATH");

    let build_dir = match build_dir_path.to_str() {
        Some(s) => s.to_owned(),
        None => panic!("Cannot convert canonicalized OUT_PATH to a valid utf-8 str")
    };

    let status = Command::new("clang")
        .args(&[
            "-c",
            "-flto",
            "-fvisibility=hidden",
            "a.c",
            "-o",
            &(build_dir.clone() + "/a.o"),
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let status = Command::new("ar")
        .current_dir(build_dir_path)
        .args(&["crus", "liba.a", "a.o"])
        .status()
        .unwrap();
    assert!(status.success());

    // Tell cargo to where to find library aspawn
    println!("cargo:rustc-link-search=native={}", build_dir);

    // Tell cargo to tell rustc to link the aspawn statically
    println!("cargo:rustc-link-lib=static=a");

    // Tell cargo to invalidate the built crate whenever the submodule changes
    println!("cargo:rerun-if-changed=a.c");
}
