use anyhow::Result;
use regex::Regex;
use std::{fs, path::PathBuf};

pub fn load_filenames(dir_path: &str) -> Result<Vec<PathBuf>> {
    let mut filenames = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            filenames.push(path);
        }
    }

    let re = Regex::new(r"transformed-combined-frame-(\d+)\.pcd$").unwrap();
    filenames.sort_by_key(|path| {
        path.to_str()
            .and_then(|s| re.captures(s))
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok())
            .unwrap()
    });

    Ok(filenames)
}
