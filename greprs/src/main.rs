extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(
            &mut stderr,
            "Problem parsing argumentsL {}",
            err
        ).expect("Could not write to stderr");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "Application error:{}",
            e
        ).expect("Could not write to stderr");
        process::exit(1);
    }
}

