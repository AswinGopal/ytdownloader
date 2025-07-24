// Library façade; re‑exports the public surface for GUI / other crates

// src/lib.rs
pub mod history;
pub mod preset;
pub mod types;
pub mod updater;
pub mod util;
pub mod yt;

pub mod modes {
    pub mod chapters;
    pub mod manual;
    pub mod timestamp;
}

pub mod ui;
