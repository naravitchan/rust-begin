#[derive(Debug)]
enum SpreadSheet {
    Integer(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let row = vec![
        SpreadSheet::Integer(3),
        SpreadSheet::Float(3.4),
        SpreadSheet::Text(String::from("Hello")),
    ];
    println!("{:?}", row);
}
