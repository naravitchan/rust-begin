use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=4 {
            println!("hi from spawn thread {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..10 {
        println!("hello from main thread {}", i);
        thread::sleep(Duration::from_millis(10));
    }
    handle.join().unwrap();
}
