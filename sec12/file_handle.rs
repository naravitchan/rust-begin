use std::fs::File;
fn main() {
    let f = File::open("hello2.txt");
    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            panic!("Custom : File not found");
        }
    };
    println!("{:?}", f);
}
