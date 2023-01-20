fn main() {
    let s1 = String::from("Hello");
    let result;
    let s2 = String::from("Bye");
    result = longest(&s1, &s2);
    println!("{}", result);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
