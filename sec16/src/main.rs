use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    // let simulated_user_specified_value: u32 = 10;
    // let simulated_random_number: u32 = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    call()
}

// fn simulated_expensive_cal(intensity: u32) -> u32 {
//     println!("Calculation Slowly....");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("Calculation Slowly....");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating Slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} Pushups", expensive_result.value(intensity));
        println!("Today do {} Situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("take a break");
        } else {
            println!("Today run for {} mins", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    arg: u32,
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
            arg: 0,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                if self.arg != arg {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    return v;
                }
                v
            }
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn call() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    assert_eq!(v1, 1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}
