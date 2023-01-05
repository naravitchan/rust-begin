fn main() {
	
	add();

}
fn fun1() {
	add();
	fn add(){
		println!("{}",2+3);
	}
}