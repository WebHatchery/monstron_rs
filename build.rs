use std::process::Command;

fn git_output(arguments: &[&str]) -> Option<String> {
    let output = Command::new("git").args(arguments).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
    if let Some(head_ref) = git_output(&["symbolic-ref", "-q", "HEAD"]) {
        if let Some(head_path) = git_output(&["rev-parse", "--git-path", &head_ref]) {
            println!("cargo:rerun-if-changed={head_path}");
        }
    }
    if let Some(files) = git_output(&["-c", "core.quotepath=false", "ls-files"]) {
        for file in files.lines().filter(|file| !file.is_empty()) {
            println!("cargo:rerun-if-changed={file}");
        }
    }

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_owned());
    let commit = git_output(&["rev-parse", "--short=12", "HEAD"])
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned());
    let dirty = git_output(&["status", "--porcelain", "--untracked-files=no"])
        .is_some_and(|status| !status.is_empty());
    let dirty_suffix = if dirty { "-dirty" } else { "" };
    println!("cargo:rustc-env=HATCHSPIRE_BUILD_ID={version}+g{commit}{dirty_suffix}");
}
