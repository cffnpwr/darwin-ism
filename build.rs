use std::path::Path;
use std::process::Command;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};

const GIT_HEAD_PATH: &str = ".git/HEAD";
const JJ_OP_HEADS_PATH: &str = ".jj/repo/op_heads/heads";

fn main() {
    // --- commit hash ---
    println!("cargo:rerun-if-env-changed=NIX_DARWIN_ISM_GIT_HASH");
    let git_hash = get_git_hash_from_nix()
        .or_else(|| {
            if Path::new(GIT_HEAD_PATH).exists() {
                println!("cargo:rerun-if-changed={GIT_HEAD_PATH}");
            } else if Path::new(JJ_OP_HEADS_PATH).exists() {
                println!("cargo:rerun-if-changed={JJ_OP_HEADS_PATH}");
            }
            get_git_hash_from_jj().or_else(get_git_hash_from_git)
        })
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=DARWIN_ISM_GIT_HASH={git_hash}");

    // --- built-at ---
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    let built_at = get_source_date_epoch()
        .or_else(get_current_time)
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=DARWIN_ISM_BUILT_AT={built_at}");
}

fn get_git_hash_from_nix() -> Option<String> {
    std::env::var("NIX_DARWIN_ISM_GIT_HASH")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s[..s.len().min(7)].to_owned())
}

fn get_git_hash_from_jj() -> Option<String> {
    Command::new("jj")
        .args([
            "--ignore-working-copy",
            "--color=never",
            "log",
            "--no-graph",
            "-r=@-",
            "-T=commit_id.short(7)",
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8(o.stdout).unwrap().trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn get_git_hash_from_git() -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8(o.stdout).unwrap().trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn get_source_date_epoch() -> Option<String> {
    let epoch: u64 = std::env::var("SOURCE_DATE_EPOCH")
        .ok()?
        .trim()
        .parse()
        .ok()?;
    let dt: DateTime<Utc> = (UNIX_EPOCH + Duration::from_secs(epoch)).into();
    Some(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

fn get_current_time() -> Option<String> {
    let dt: DateTime<Utc> = std::time::SystemTime::now().into();
    Some(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}
