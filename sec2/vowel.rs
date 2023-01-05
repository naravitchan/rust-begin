use std::io;
fn main() {
    let mut v = String::new();
    println!("Enter a Character");
    io::stdin().read_line(&mut v).expect("Failed");
    let v: char = v.trim().parse().expect("Failed");

    if v == 'a' || v == 'e' || v == 'i' || v == 'o' || v == 'u' {
        println!("vowel");
    } else {
        println!("consonant");
    }
}
