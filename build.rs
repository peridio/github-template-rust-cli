use std::env;
use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(output.stdout).unwrap();

    // version
    println!(
        "cargo:rustc-env=CARGO_PKG_VERSION={} {}",
        env!("CARGO_PKG_VERSION"),
        git_hash
    );

    // // target
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
}
