mod cal;
mod helper;
// use helper::sum;
// use helper::sum2;
use cal::bow::sum_bow;
use cal::paisit::pp::sum_pp;
// use cal::sum2;
use helper::sum;

fn main() {
    println!("Hello, world!");
    println!("sum : {}", helper::sum(1, 2));
    println!("sum : {}", sum(1, 2));
    println!("sum : {}", sum_bow(1, 2));
    println!("sum : {}", sum_pp(2, 2));
}
