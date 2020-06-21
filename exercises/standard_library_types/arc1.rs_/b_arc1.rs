// rdfa:src="https://github.com/BaderSZ/rustlings-solutions/blob/master/exercises/threads/threads1.rs"
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

struct JobStatus {
    jobs_completed: AtomicUsize,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: AtomicUsize::new(0) });
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            
            status_shared.jobs_completed.fetch_add(1, Ordering::Relaxed);
        }
    });
    while status.jobs_completed.load(Ordering::Relaxed) < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
