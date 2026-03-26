#![allow(
    clippy::panic,
    clippy::manual_assert,
    reason = "build script: panic on error is acceptable"
)]

use std::cmp::Ordering;
use std::env;
use std::ffi::OsString;
use std::process::Command;

const MINIMUM_MACOS_VERSION: &str = "10.5";

fn main() {
    println!("cargo::rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
    println!("cargo::rerun-if-env-changed=RUSTC");
    println!("cargo::rerun-if-env-changed=TARGET");

    let target_os = env::var("CARGO_CFG_TARGET_OS").ok();
    if target_os.as_deref() != Some("macos") {
        panic!("text-input-source-rs supports macOS targets only");
    }

    let deployment_target = match env::var("MACOSX_DEPLOYMENT_TARGET") {
        Ok(value) => value,
        Err(_) => rustc_deployment_target()
            .unwrap_or_else(|error| panic!("failed to determine macOS deployment target: {error}")),
    };

    if compare_versions(&deployment_target, MINIMUM_MACOS_VERSION) == Ordering::Less {
        panic!(
            "text-input-source-rs requires macOS {MINIMUM_MACOS_VERSION} or later, got deployment target {deployment_target}"
        );
    }
}

fn rustc_deployment_target() -> Result<String, String> {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let target = env::var("TARGET").map_err(|error| format!("missing TARGET: {error}"))?;
    let output = Command::new(rustc)
        .arg("--target")
        .arg(target)
        .arg("--print=deployment-target")
        .output()
        .map_err(|error| format!("failed to invoke rustc: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "rustc --print=deployment-target exited with {}: {}",
            output.status,
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|error| format!("invalid rustc output encoding: {error}"))?;
    let deployment_target = stdout.trim();
    if deployment_target.is_empty() {
        return Err("rustc returned an empty deployment target".into());
    }

    Ok(deployment_target.to_owned())
}

fn compare_versions(actual: &str, minimum: &str) -> Ordering {
    parse_version(actual).cmp(&parse_version(minimum))
}

fn parse_version(version: &str) -> (u32, u32, u32) {
    let normalized = normalize_version_string(version);
    let mut parts = normalized.split('.').map(parse_component);
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

fn normalize_version_string(version: &str) -> &str {
    version.split('=').next_back().map_or(version, str::trim)
}

fn parse_component(component: &str) -> u32 {
    component.parse::<u32>().unwrap_or(0)
}
