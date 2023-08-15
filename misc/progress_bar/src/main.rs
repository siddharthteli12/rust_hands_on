use std::{thread, time::Duration};

fn main() {
    let bar = indicatif::ProgressBar::new(100);
    let _ = thread::spawn(move || {
        for _ in 0..100 {
            bar.inc(1);
            thread::sleep(Duration::from_millis(100));
        }
        bar.finish();
    })
    .join();
}
