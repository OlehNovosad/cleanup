use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

pub fn progress_bar(stop: Arc<AtomicBool>) {
    let frames = ["|", "/", "-", "\\"];
    let mut step = 0;

    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        // Clear the line
        print!("\r");
        let bar_width = step % 15;
        let visual_bar = "#".repeat(bar_width);
        let frame = frames[step % frames.len()];

        print!("\r{} [{:15}]", frame, visual_bar);

        std::io::stdout().flush().unwrap();

        step += 1;
        thread::sleep(Duration::from_millis(150));
    }
}
