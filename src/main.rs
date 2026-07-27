mod drive;
mod progress_bar;
mod scanner;

use crate::drive::print_drives;
use std::fs;
use std::io::{self, Write};

fn main() {
    let mut output = fs::File::create("output.txt").expect("Failed to create file");

    select_drive();
    run_scan("/Users/olehnovosad/Documents", &mut output);
}

fn select_drive() {
    let mut drive = String::new();

    let drives = print_drives();

    io::stdin()
        .read_line(&mut drive)
        .expect("Failed to read line");

    let drive = drive.trim();
    println!("You entered: {}", drive);

    let total_size: u64 = drives
        .list()
        .iter()
        .filter(|disk| disk.mount_point().to_str() == Some(drive))
        .map(|disk| to_gb!((disk.total_space() - disk.available_space())))
        .sum();

    println!("{}", total_size);
}

fn run_scan(path: &str, writer: &mut impl Write) {
    match scanner::start_scan(path.to_string()) {
        Ok(scan_res) => {
            for res in scan_res.iter() {
                writeln!(writer, "{}", res).expect("Failed to write to file");
            }
        }
        Err(err) => {
            writeln!(writer, "Error while scanning: {err}").expect("Failed to write to file");
        }
    }
}
