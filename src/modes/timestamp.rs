use inquire::Text;
use inquire::Select;
use std::process::Command;

use crate::types::{DlOpts, Result};
use crate::yt;

pub fn run() -> Result<()> {
    let url = Text::new("Enter video URL (or leave blank to go back):").prompt()?;
    if url.trim().is_empty() { return Ok(()); }

    // format selection
    let format = match Select::new("\nSelect download format:", vec!["Best (default)", "Enter manually"]).prompt()? {
        "Best (default)" => None,
        _ => {
            println!("\n📦 Available formats:\n");
            Command::new("yt-dlp").args(["--list-formats", &url]).status().ok();
            println!("💡 e.g. '137+140' or 'bestvideo+bestaudio'\n");
            Text::new("Format string:").prompt().ok()
        }
    };

    let start = Text::new("Start time (HH:MM:SS):").prompt()?;
    let end = Text::new("End time   (HH:MM:SS):").prompt()?;

    let opts = DlOpts {
        url,
        format,
            sections: Some(format!("*{}-{}", start, end)),
            extra: Vec::new(),
    };
    yt::run(opts, None)
}
