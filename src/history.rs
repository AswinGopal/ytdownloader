use std::{
    collections::HashSet,
    fs::{File, OpenOptions, create_dir_all},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

/// Append “<title> - <url>” to ~/.config/ytrst/history.log **once per URL**.
///
/// ‣ If the URL is already present in any line of the file, nothing is written.  
/// ‣ Keeps O(n) worst‑case behaviour but `n` is tiny (one line per download).
pub fn log_entry(title: &str, url: &str) {
    // 1. Build final file path: <config>/ytrst/history.log
    let mut path: PathBuf = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("ytdownloader");
    let _ = create_dir_all(&path);
    path.push("ythis.log");

    // 2. Fast dedup: read the file once & cache URLs into a HashSet
    let mut seen: HashSet<String> = HashSet::new();
    if let Ok(file) = File::open(&path) {
        for line in BufReader::new(file).lines().flatten() {
            // Each line ends with “ - <url>”
            if let Some(pos) = line.rfind(" - ") {
                seen.insert(line[pos + 3..].to_string());
            }
        }
    }

    if seen.contains(url) {
        return; // Already logged: skip writing
    }

    // 3. Append the new, unique line
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(file, "{} - {}", title.trim(), url.trim());
    }
}
