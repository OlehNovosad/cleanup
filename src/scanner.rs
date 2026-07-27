use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::{eprintln, fmt};
use walkdir::WalkDir;

use crate::progress_bar::progress_bar;

#[derive(Debug)]
pub struct DirEntryInfo {
    pub path: String,
    pub size: u64,
}

impl fmt::Display for DirEntryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} bytes", self.path, self.size)?;
        Ok(())
    }
}

pub fn start_scan(path: String) -> Result<Vec<DirEntryInfo>, walkdir::Error> {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop);

    let handle = thread::spawn(move || {
        progress_bar(stop_clone);
    });

    let mut scan_res: Vec<DirEntryInfo> = Vec::new();

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    scan_res.push(DirEntryInfo {
                        path: entry.path().to_string_lossy().into_owned(),
                        size: entry.metadata().map(|m| m.len()).unwrap_or(0),
                    });
                }
            }
            Err(err) => eprintln!("Error reading entry: {err}"),
        }
    }

    stop.store(true, Ordering::Relaxed);
    handle.join().unwrap();

    Ok(scan_res)
}
