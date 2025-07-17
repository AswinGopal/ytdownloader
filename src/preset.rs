use crate::types::{DlOpts, Preset, Result};
use crate::yt;

/// Map a `Preset` + url to `DlOpts` and launch yt-dlp.
/// *`progress`* is forwarded to `yt::run`.
pub fn run_preset(p: Preset, url: &str, progress: yt::ProgressCb) -> Result<()> {
    let (format, extra) = match p {
        Preset::Mp3 => (None, vec!["-x".into(), "--audio-format".into(), "mp3".into()]),
        Preset::M4a => (None, vec!["-x".into(), "--audio-format".into(), "m4a".into()]),
        Preset::Video1080p => (Some("137+140".into()), Vec::new()),
        Preset::Best => (None, Vec::new()),
    };

    let opts = DlOpts {
        url: url.into(),
        format,
        sections: None,
        extra,
    };
    yt::run(opts, progress)
}