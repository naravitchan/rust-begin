use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // Err(_error) => {
        //     panic!("Custom : File not found");
        // }
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(e) => {
                panic!("Not able to create file {:?}", e);
            }
        },
        Err(error) => {
            panic!("Unable to open file {:?}", error);
        }
    };
    println!("{:?}", f);
}
