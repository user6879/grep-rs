use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line from file {}: {}", filename, e);
                continue;
            }
        };

        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
