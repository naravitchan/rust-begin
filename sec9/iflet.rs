fn main() {
	let some_u8=Some(4);
	if let Some(3)=some_u8 {
		println!("three")
	} else if let Some(4)=some_u8{
		println!("Some other value")
	}
}