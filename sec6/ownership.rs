fn main() {
    let mut s = String::from("hello");
    // take(s);
    // println!("{}", s); error[E0382]: borrow of moved value: `s`
    s = take(s);
    println!("{}", s);

    let mut a = String::from("hello Ref");
    take_ref(&mut a);
    println!("{}", a);
}

fn take(s1: String) -> String {
    println!("{}", s1);
    s1 // return s1 to fix moved error
}

fn take_ref(s1: &mut String) {
    println!("{}", s1);
    s1.push_str(" add to Ref String");
    // the data it refers to cannot be borrowed as mutable (fix by pass mut)
}
