use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let simulated_user_specified_value: u32 = 10;
    let simulated_random_number: u32 = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// fn simulated_expensive_cal(intensity: u32) -> u32 {
//     println!("Calculation Slowly....");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculation Slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today do {} Pushups", expensive_closure(intensity));
        println!("Today do {} Situps", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("take a break");
        } else {
            println!("Today run for {} mins", expensive_closure(intensity));
        }
    }
}
