use sysinfo::Disks;

#[macro_export]
macro_rules! to_gb {
    ($val:expr) => {
        $val / 1_024 / 1_024 / 1_024
    };
}

pub fn print_drives() -> Disks {
    let drives = Disks::new_with_refreshed_list();
    let visible_drive: Vec<_> = drives
        .list()
        .iter()
        .filter(|d| d.mount_point().to_str() != Some("/System/Volumes/Data"))
        .collect();

    println!("Found {} drives:", visible_drive.len());
    for drive in visible_drive {
        println!("--- Drive Info ---");
        println!("Name: {:?}", drive.name());
        println!("Mount Point: {:?}", drive.mount_point());
        println!("File System: {:?}", drive.file_system());
        println!("Total Space: {} GB", to_gb!(drive.total_space()));
        println!("Available Space: {} GB", to_gb!(drive.available_space()));
        println!("Is Removable: {}", drive.is_removable());
    }

    drives
}
