use std::sync::{Arc, Mutex};
use std::thread;
static mut SEAT: i32 = 1;

fn cab(name: &str) {
    unsafe {
        // println!("{:?}", SEAT);
        if SEAT >= 1 {
            println!("Cab Available for {}", name);
            println!("Booking Confirmed");
            println!("Thanks for Choosing our Service");
            SEAT -= 1;
        } else {
            println!("Cab Not Available for {}", name);
        }
    }
}

fn main() {
    // let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let name = ["Rob", "Peter", "Bob", "Robert"];

    for i in 0..4 {
        // let counter = Arc::clone(&counter);
        println!("S");
        let handle = thread::spawn(move || {
            // let mut num = counter.lock().unwrap();
            let n = name[i];
            cab(n);
            // *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
