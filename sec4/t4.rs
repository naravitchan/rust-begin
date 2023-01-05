fn main() {
	println!("{:?}",sub_add(4,2));

}
fn sub_add(a:i32,b:i32)->(i32,i32){
		(a+b,a-b)
}