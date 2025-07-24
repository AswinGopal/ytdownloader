use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DlOpts {
    pub url: String,
    pub format: Option<String>,
    pub sections: Option<String>,
    pub extra: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Preset {
    Mp3,
    M4a,
    Video1080p,
    Best,
}

pub type Result<T> = anyhow::Result<T>;
