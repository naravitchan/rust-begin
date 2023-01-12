fn main() {
    // let num=Some(5);
    // let txt = Some("Hello");
    // let num:Option<i32>=None;
    let txt:Option<&str>=None;
    let num:Option<i32>=Some(5);
    println!("{:?} {:?}", num, txt);
}