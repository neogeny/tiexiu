use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{bar}] {msg}").unwrap());

    for i in 0..100 {
        if i == 50 {
            // This safely interrupts the bar, prints, and resumes it
            pb.suspend(|| {
                println!("--- Reached the halfway point ---");
            });
        }
        pb.inc(1);
        std::thread::sleep(Duration::from_millis(50));
    }
    pb.finish_with_message("Done");
}
