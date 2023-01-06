fn main() {
    let mut s = String::from("hello");
    // take(s);
    // println!("{}", s); error[E0382]: borrow of moved value: `s`
    s = take(s);
    println!("{}", s);

    let a = String::from("hello Ref");
    take_ref(&a);
    println!("{}", a);
}

fn take(s1: String) -> String {
    println!("{}", s1);
    s1 // return s1 to fix moved error
}

fn take_ref(s1: &String) {
    println!("{}", s1);
}
