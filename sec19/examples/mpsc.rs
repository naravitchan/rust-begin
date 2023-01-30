use std::sync::mpsc; // mpsc -  multiple producer | single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (s, r) = mpsc::channel();
    thread::spawn(move || {
        // let val = String::from("hi");
        // s.send(val).unwrap();
        // println!("{}", val);
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            s.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let rec = r.recv().unwrap(); // wait for msg

    for received in r {
        println!("Got {}", received);
    }
}
