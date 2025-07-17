/// Convert seconds (float) into HH:MM:SS (no fractions).
pub fn seconds_to_hms(sec: f64) -> String {
    let s = sec as u64;
    format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}