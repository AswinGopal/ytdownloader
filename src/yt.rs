use crate::history;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use dirs::download_dir;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::io::{BufRead, BufReader};

use crate::types::{DlOpts, Result};

/// Build the default output template (Downloads/ on all OSes).
fn output_template(clipping: bool) -> String {
    let dir: PathBuf = download_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    let base = if clipping {
        "%(title)s-%(section_start)s-%(section_end)s.%(ext)s"
    } else {
        "%(title)s.%(ext)s"
    };
    format!("{}/{}", dir.display(), base)
}

/// Optional callback for live progress (0.0‒100.0).
pub type ProgressCb = Option<Box<dyn Fn(f64) + Send + Sync + 'static>>;

/// Public entry‑point used by CLI *and* GUI.
pub fn run(opts: DlOpts, progress: ProgressCb) -> Result<()> {
    // ❶ Query yt‑dlp once for the title (no download, very fast)
    let title = std::process::Command::new("yt-dlp")
    .args([
        "--print",
        "title",
        "--skip-download",
        "--no-warnings",
        &opts.url,
    ])
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::null())
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .unwrap_or_else(|_| opts.url.clone());

    let clipping = opts.sections.is_some();
    let template = output_template(clipping);

    // Build arg list
    let mut args = vec![
        "--no-warnings".into(),
        "--newline".into(),
        "-o".into(),
        template,
    ];
    if let Some(sec) = &opts.sections {
        args.extend(["--download-sections".into(), sec.clone()]);
    }
    if let Some(fmt) = &opts.format {
        args.extend(["-f".into(), fmt.clone()]);
    }
    args.extend(opts.extra.clone());
    args.push(opts.url.clone());

    let status = if clipping {
        // Spinner‑only mode
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner} ⏬ Downloading with yt-dlp…")
            .unwrap()
            .tick_chars("⠂⠒⠒⠤⠤⠂"),
        );
        pb.enable_steady_tick(Duration::from_millis(120));

        let status = Command::new("yt-dlp")
        .args(&args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

        pb.finish_and_clear();
        status
    } else {
        // Full progress‑bar mode
        let mut child = Command::new("yt-dlp")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

        let stdout = child.stdout.take().expect("capture stdout");
        let reader = BufReader::new(stdout);

        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>3}% {msg}")
            .unwrap()
            .progress_chars("=>-"),
        );
        pb.set_message("Downloading…");
        pb.enable_steady_tick(Duration::from_millis(120));

        let re_pct = Regex::new(r"\[download\]\s+([\d\.]+)%").unwrap();
        let pb_clone = pb.clone();
        thread::spawn(move || {
            // FIX: Replace `.flatten()` with an explicit loop to handle errors.
            for line_result in reader.lines() {
                let line = match line_result {
                    Ok(line) => line,
                      Err(_) => continue, // Skip bad lines
                };
                if let Some(c) = re_pct.captures(&line) {
                    if let Ok(p) = c[1].parse::<f64>() {
                        pb_clone.set_position(p.round() as u64);
                    }
                }
            }
        });

        let status = child.wait()?;
        pb.finish_and_clear();
        if let Some(cb) = progress {
            cb(100.0);
        }
        status
    };

    // ❷ Persist: "<title> - <url>" only after a successful download
    anyhow::ensure!(status.success(), "yt-dlp exited with {:?}", status);
    history::log_entry(&title, &opts.url);

    Ok(())
}
