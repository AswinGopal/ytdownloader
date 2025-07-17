// src/main.rs (thin CLI wrapper around your library)

use anyhow::Result;
use ytdownloader::{modes, preset, ui, updater};

fn main() -> Result<()> {
    loop {
        match ui::main_menu()? {
            ui::MenuChoice::Preset(p) => {
                let url = inquire::Text::new("Enter video URL:").prompt()?;
                preset::run_preset(p, &url, None)?;
            }
            ui::MenuChoice::Manual => modes::manual::run()?,
            ui::MenuChoice::Chapter => modes::chapters::run()?,
            ui::MenuChoice::Timestamp => modes::timestamp::run()?,
            ui::MenuChoice::Update => updater::check_and_update()?,
            ui::MenuChoice::Quit => {
                println!("👋 Goodbye!");
                break;
            }
        }
        println!();
    }
    Ok(())
}
