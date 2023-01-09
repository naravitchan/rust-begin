#[derive(Debug)]
struct User {
    email: String,
    age: i32,
}

fn build(age: i32) -> User {
    User {
        email: String::from("test@email.com"),
        age,
    }
}

fn main() {
    let mut rob = User {
        email: String::from("rob@email.com"),
        age: 22,
    };
    rob.age = 21;
    println!("{:?}", rob);
    println!("{:?}", rob.age);
    println!("{:?}", rob.email);
    let u2 = User {
        email: String::from("sg@email.com"),
        age: 28,
    };
    println!("{:?}", u2);
    if rob.age > u2.age {
        println!("rob is elder");
    } else {
        println!("u2 is elder");
    }

    let u3 = build(19);
    println!("{:?}", u3);
}
