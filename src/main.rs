use crate::drive::print_drives;
use std::{io, println};

mod drive;

fn main() {
    let mut drive = String::new();

    // Get list of drives
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
