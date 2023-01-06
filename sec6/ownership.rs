fn main() {
    let s = String::from("hello");
    take(s);
    // println!("{}", s); error[E0382]: borrow of moved value: `s`
}

fn take(s1: String) {
    println!("{}", s1);
}
