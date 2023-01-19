extern crate communicator;
#[warn(unused_imports)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
// use TrafficLight::{Red, Yellow};
use TrafficLight::*;
fn main() {
    communicator::client::connect();
    let _r = Red;
    let _y = Yellow;
    let _g = Green;
}
