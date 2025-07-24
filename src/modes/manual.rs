use crate::types::{DlOpts, Result};
use crate::yt;
use inquire::Text;

pub fn run() -> Result<()> {
    let url = Text::new("Enter video URL (or leave blank to go back):").prompt()?;
    if url.trim().is_empty() || url.eq_ignore_ascii_case("back") {
        return Ok(());
    }

    println!("\n📦 Fetching format list...\n");
    std::process::Command::new("yt-dlp")
        .arg("-F")
        .arg(&url)
        .status()
        .ok();

    let fmt = Text::new("\nEnter format code (e.g. 137+140):").prompt()?;

    let opts = DlOpts {
        url,
        format: Some(fmt),
        sections: None,
        extra: Vec::new(),
    };
    yt::run(opts, None)
}
