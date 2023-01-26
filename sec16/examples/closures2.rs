fn main() {
    let x = vec![4];
    let equal = move |z| z == x;
    let y = vec![4];
    // println!("{:?}", x); // err: value borrowed here after move
    assert!(equal(y));
}
