use inquire::{MultiSelect, Select};
use regex::Regex;
use std::process::Command;

use crate::types::{DlOpts, Result};
use crate::{util, yt};

pub struct Chapter {
    pub title: String,
    pub start: f64,
    pub end: f64,
}

fn fetch_chapters(url: &str) -> Result<Vec<Chapter>> {
    let stdout = Command::new("yt-dlp")
        .args(["--print", "chapters", url])
        .output()?
        .stdout;

    let raw = String::from_utf8_lossy(&stdout);
    let re = Regex::new(r"\{'start_time': ([\d.]+), 'title': '([^']+)', 'end_time': ([\d.]+)\}")
        .unwrap();

    Ok(re
        .captures_iter(&raw)
        .map(|c| Chapter {
            title: c[2].to_string(),
            start: c[1].parse().unwrap_or(0.0),
            end: c[3].parse().unwrap_or(0.0),
        })
        .collect())
}

pub fn run() -> Result<()> {
    use inquire::Text;
    let url = Text::new("Enter video URL (or leave blank to go back):").prompt()?;
    if url.trim().is_empty() {
        return Ok(());
    }

    // ask for format (Best | Manual)
    let format = match Select::new(
        "\nSelect download format:",
        vec!["Best (default)", "Enter manually"],
    )
    .prompt()?
    {
        "Best (default)" => None,
        _ => {
            println!("\n📦 Available formats:\n");
            Command::new("yt-dlp")
                .args(["--list-formats", &url])
                .status()
                .ok();
            println!("💡 e.g. '137+140' or 'bestvideo+bestaudio'\n");
            Text::new("Format string:").prompt().ok()
        }
    };

    let chapters = fetch_chapters(&url)?;
    if chapters.is_empty() {
        println!("⚠️  No chapters found.");
        return Ok(());
    }

    let titles: Vec<String> = chapters.iter().map(|c| c.title.clone()).collect();
    let picked = MultiSelect::new("Select chapters:", titles).prompt()?;
    if picked.is_empty() {
        println!("❌ No chapters selected.");
        return Ok(());
    }

    let ranges: Vec<String> = chapters
        .into_iter()
        .filter(|c| picked.contains(&c.title))
        .map(|c| {
            format!(
                "*{}-{}",
                util::seconds_to_hms(c.start),
                util::seconds_to_hms(c.end)
            )
        })
        .collect();

    println!("\n📥 Downloading selected chapters as separate files…\n");
    for (idx, range) in ranges.iter().enumerate() {
        println!("▶️  Chapter {}/{} …", idx + 1, ranges.len());
        let opts = DlOpts {
            url: url.clone(),
            format: format.clone(),
            sections: Some(range.clone()),
            extra: Vec::new(),
        };
        yt::run(opts, None)?;
    }
    Ok(())
}
