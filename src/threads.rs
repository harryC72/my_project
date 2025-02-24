use std::thread::{spawn, JoinHandle};

pub fn test_threads() {
    let mut x = 0u128;
    for i in 1..50_000_000 {
        x += i;
    }

    println!("\x1b[38;2;100;100;255mMain thread finished a little bit of work... let's go check on them...\x1b[0m");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500_000 {
            x += i;
        }
        println!("{}", x.to_string())
    };

    println!("starting new worker thread");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("worker thread completed");

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, let's get out of here");
            break;
        }
    }
}
