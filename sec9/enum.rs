// #[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum IpAddrKind{
	gg,
	v6,
}
fn main() {
	let four=IpAddrKind::gg;
	let six=IpAddrKind::v6;
	println!("{:?}",four);
	println!("{:?}",six);
	route(four);
	route(six);
}
fn route(ip:IpAddrKind){
	println!("{:?}",ip);
}
