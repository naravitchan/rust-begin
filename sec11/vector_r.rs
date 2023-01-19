fn main() {
    // Reading Data From Vectors
    let mut v = vec![1, 2, 3];
    // let value = v[100]; err the len is 3 but the index is 100
    let value = v.get(10);
    println!("{:?}", value);
    if let None = value {
        println!("value is None")
    }

    // For
    for i in &mut v {
        *i += 2;
        println!("{}", i)
    }
    // println!("{:?}", v)  err value borrowed here after move - fix to use &v
    println!("{:?}", v)
}
