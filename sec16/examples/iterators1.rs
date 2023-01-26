fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // println!("{:?}", v1_iter);
    // for val in v1_iter {
    //     println!("{:?}", val);
    // }
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.into_iter();
    assert_eq!(v2_iter.next(), Some(1));
    assert_eq!(v2_iter.next(), Some(2));
    assert_eq!(v2_iter.next(), Some(3));
    assert_eq!(v2_iter.next(), None);

    let mut v3 = vec![1, 2, 3];
    let mut v3_iter = v3.iter_mut();
    assert_eq!(v3_iter.next(), Some(&mut 1));
    assert_eq!(v3_iter.next(), Some(&mut 2));
    assert_eq!(v3_iter.next(), Some(&mut 3));
    assert_eq!(v3_iter.next(), None);
}
