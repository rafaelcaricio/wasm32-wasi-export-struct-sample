use std::path::{Path, PathBuf};
use std::env;

use copy_dir;

const LIB_NAME: &str = "quickjs";

fn exists(path: impl AsRef<Path>) -> bool {
    PathBuf::from(path.as_ref()).exists()
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let code_dir = out_path.join("quickjs");
    if exists(&code_dir) {
        std::fs::remove_dir_all(&code_dir).unwrap();
    }
    copy_dir::copy_dir("./vendor", &code_dir).expect("Could not copy vendor directory");

    eprintln!("Building Makefile...");
    std::process::Command::new("wasimake")
        .arg("cmake")
        .arg(".")
        .current_dir(&code_dir)
        .spawn()
        .expect("Could not use `wasimake` to build the Makefile")
        .wait()
        .expect("Could not use `wasimake` to build the Makefile");

    eprintln!("Compiling sample...");
    let cmd = std::process::Command::new("make")
        .arg(LIB_NAME)
        .current_dir(&code_dir)
        .spawn()
        .expect("Could not compile sample")
        .wait()
        .expect("Could not compile sample");
    if !cmd.success() {
        panic!("Could not compile sample! {}", cmd.to_string());
    }

    // Instruct cargo to statically link quickjs.
    println!(
        "cargo:rustc-link-search=native={}",
        code_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static={}", LIB_NAME);
}
