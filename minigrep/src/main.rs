extern crate minigrep;
use minigrep::Config;
use std::env;
use std::process;

// main4
fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem in Parsing {} ", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);
    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application Error {} ", err);
        process::exit(1);
    });
}
