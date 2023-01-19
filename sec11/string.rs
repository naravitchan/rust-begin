fn main() {
    let a = 1;
    let mut s = a.to_string();
    s.push_str(" Hello");
    s.push('o');
    println!("{}", s);

    let s1 = String::from("sad");
    let s2 = String::from("da");
    // let s3 = &s1 + &s2; // err `+` cannot be used to concatenate two `&str` strings
    let s3 = format!("{}-{}", s1, s2);
    println!("{}", s3);

    // slice
    println!("- slice -");
    let s1 = String::from("sad");
    println!("{}", &s1[0..1]);

    for n in s1.chars() {
        println!("{}", n);
    }
}
