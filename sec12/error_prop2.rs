use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let output = read();
    match output {
        Ok(fi) => println!("Ok {:?}", fi),
        Err(e) => println!("err {:?}", e),
    };
}
fn read() -> Result<String, io::Error> {
    let mut f = File::open("hello2.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
