//! Embeds the toolchain and target information shown in the startup banner.

use std::env;
use std::process::Command;

fn main() {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let version = Command::new(rustc)
        .arg("-V")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=HYEONBOT_RUSTC_VERSION={version}");
    println!(
        "cargo:rustc-env=HYEONBOT_TARGET={}",
        env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env=HYEONBOT_PROFILE={}",
        env::var("PROFILE").unwrap()
    );
    println!("cargo:rerun-if-changed=build.rs");
}
