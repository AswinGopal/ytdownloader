// Library façade; re‑exports the public surface for GUI / other crates

// src/lib.rs
pub mod types;
pub mod util;
pub mod yt;
pub mod preset;
pub mod updater;
pub mod history;

pub mod modes {
    pub mod manual;
    pub mod chapters;
    pub mod timestamp;
}

pub mod ui;