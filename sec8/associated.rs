#[derive(Debug)]
struct Rectangle {
	w:i32,
	h:i32
}

impl Rectangle {
	fn build(w:i32,h:i32) ->Rectangle {
		Rectangle {w, h}
	}
}

fn main() {
	let r1=Rectangle::build(2,3);
	println!("{:?}",r1);
}
