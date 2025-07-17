// src/ui.rs (only used by CLI – isolate inquire here for easy swap with egui)

use inquire::Select;
use crate::types::{Preset, Result};

pub enum MenuChoice {
    Preset(Preset),
    Manual,
    Chapter,
    Timestamp,
    Update,
    Quit,
}

pub fn main_menu() -> Result<MenuChoice> {
    let item = Select::new(
        "Choose an option:",
        vec![
            "🎛️ Use Preset",
            "🧠 Manual Format Input",
            "📚 Download by Chapter",
            "⏱️ Download by Timestamp",
            "⬆️ Update yt-dlp",
            "❌ Quit",
        ],
    )
    .prompt()?;

    Ok(match item {
        "🎛️ Use Preset" => {
            let preset = Select::new(
                "Choose a preset:",
                vec![
                    "🎵 MP3 Audio",
                    "🎶 M4A Audio",
                    "🎥 1080p Video",
                    "🔥 Best Available",
                    "  Back",
                ],
            )
            .prompt()?;

            //early return if user wants out
            if preset == "  Back" {
                return main_menu();         // jump straight back to the top‑level menu
            }

            MenuChoice::Preset(match preset {
                "🎵 MP3 Audio" => Preset::Mp3,
                "🎶 M4A Audio" => Preset::M4a,
                "🎥 1080p Video" => Preset::Video1080p,
                _ => Preset::Best,
            })
        }
        "🧠 Manual Format Input" => MenuChoice::Manual,
        "📚 Download by Chapter" => MenuChoice::Chapter,
        "⏱️ Download by Timestamp" => MenuChoice::Timestamp,
        "⬆️ Update yt-dlp" => MenuChoice::Update,
        _ => MenuChoice::Quit,
    })
}