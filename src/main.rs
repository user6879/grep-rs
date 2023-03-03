extern crate grs;

use std::env;
use std::process;

use grs::run;
use grs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    println!("\n");

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
