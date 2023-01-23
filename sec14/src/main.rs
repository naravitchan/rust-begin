mod another_test;
mod panic;
fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4)
    }
    // #[test]
    // fn another() {
    //     panic!("test fail");
    //     // assert_eq!(2 + 2, 4)
    // }

    #[test]
    fn it_add_two() {
        assert_eq!(add_two(2), 4)
    }

    // custom msg
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rob");
        assert!(
            result.contains("Rob"),
            "Greeting did not contain name. value was {}",
            result
        )
    }
}
fn main() {
    println!("Hello, world!");
}
