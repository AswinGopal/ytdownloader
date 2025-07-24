use std::process::{Command, Stdio};

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value as Json;

use crate::types::Result;

/// Call `yt-dlp --version` and return trimmed string (e.g. "2025.06.24").
fn current_version() -> Result<String> {
    let out = Command::new("yt-dlp").arg("--version").output()?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// Query GitHub Releases API for the latest tag name.
fn latest_version() -> Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("ytdownloader (+https://github.com/yourname)"),
    );
    let client = Client::builder().default_headers(headers).build()?;
    let resp: Json = client
    .get("https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest")
    .send()?
    .json()?;
    let tag = resp["tag_name"]
    .as_str()
    .ok_or_else(|| anyhow::anyhow!("tag_name missing"))?;
    Ok(tag.trim_start_matches("yt-dlp ").to_owned())
}

/// Automatically update yt-dlp **only if** a newer version exists.
/// Prints status messages either way.
pub fn check_and_update() -> Result<()> {
    let current = current_version()?;
    let latest = latest_version()?;

    if current == latest {
        println!("✅ yt-dlp is already up to date ({}).", current);
        return Ok(());
    }

    println!("⬆️  New yt-dlp version available: {} → updating…", latest);
    let status = Command::new("yt-dlp")
    .arg("-U")
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()?;
    anyhow::ensure!(status.success(), "yt-dlp updater failed: {:?}", status);
    println!("✅ Updated yt-dlp from {} to {}.", current, latest);
    Ok(())
}
