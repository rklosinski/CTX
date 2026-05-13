use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn main() {
    let sha = git(&["rev-parse", "--short", "HEAD"]);
    let branch = git(&["rev-parse", "--abbrev-ref", "HEAD"]);
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    let mut build_info = String::new();
    if let Some(s) = sha {
        build_info.push('+');
        build_info.push_str(&s);
    }
    if let Some(b) = branch {
        if b != "main" && b != "HEAD" {
            build_info.push('.');
            build_info.push_str(&b);
        }
    }
    if dirty {
        build_info.push_str("-dirty");
    }

    println!("cargo:rustc-env=CTX_BUILD_INFO={build_info}");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/index");
}
