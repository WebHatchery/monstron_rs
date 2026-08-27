use std::process::Command;

fn git_output(arguments: &[&str]) -> Option<String> {
    let output = Command::new("git").args(arguments).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn main() {
    println!("cargo:rerun-if-changed=assets/branding/hatchspire.ico");
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

    let windows_target = std::env::var("TARGET")
        .map(|target| target.contains("windows"))
        .unwrap_or(false);
    if cfg!(windows) && windows_target {
        let mut resource = winres::WindowsResource::new();
        resource
            .set_icon("assets/branding/hatchspire.ico")
            .set_language(0x0409)
            .set_version_info(winres::VersionInfo::FILEVERSION, packed_version(&version))
            .set_version_info(
                winres::VersionInfo::PRODUCTVERSION,
                packed_version(&version),
            )
            .set("FileDescription", "Hatchspire")
            .set("ProductName", "Hatchspire")
            .set("OriginalFilename", "hatchspire.exe")
            .set("InternalName", "hatchspire")
            .set("ProductVersion", &version)
            .set("FileVersion", &version)
            .set(
                "Comments",
                "Internal preview; public release approval required",
            );
        resource
            .compile()
            .expect("Windows application resource compilation failed");
    }
}

fn packed_version(version: &str) -> u64 {
    let mut parts = version
        .split('.')
        .map(|part| part.parse::<u64>().unwrap_or(0));
    (parts.next().unwrap_or(0) << 48)
        | (parts.next().unwrap_or(0) << 32)
        | (parts.next().unwrap_or(0) << 16)
        | parts.next().unwrap_or(0)
}
