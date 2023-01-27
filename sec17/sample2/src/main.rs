extern crate sample2;
use sample2::mix;
use sample2::PrimaryColor;
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
