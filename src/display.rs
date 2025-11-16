use std::{fmt::Display, fs, os::unix::fs::MetadataExt, path::Path};

pub fn printkv<S, D>(k: S, v: D)
where
    S: AsRef<str>,
    D: Display,
{
    let k = format!("{}:", k.as_ref());
    println!("    {k:<25}{v}")
}

pub fn fmt_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    if bytes as f64 >= TB {
        format!("{:.2} TB", bytes as f64 / TB)
    } else if bytes as f64 >= GB {
        format!("{:.2} GB", bytes as f64 / GB)
    } else if bytes as f64 >= MB {
        format!("{:.2} MB", bytes as f64 / MB)
    } else if bytes as f64 >= KB {
        format!("{:.2} KB", bytes as f64 / KB)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn fmt_file_size<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let meta = match fs::metadata(path) {
        Ok(v) => v,
        Err(_) => return "".into(),
    };

    fmt_size(meta.size())
}
