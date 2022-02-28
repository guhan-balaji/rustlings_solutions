// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });

    // lock goes out of scope before reaching body
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }

    // while (x ...)
    // {  <- x goes out of scope before body
    // ...
    // }
    //
    // while let y = expression {
    // ...
    // } <- y goes out of scope here
    // Temp variables in the expression follow the same rule as y.
    // This applies to if let and match statements as well.
}
