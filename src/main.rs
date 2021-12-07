#![feature(test)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::env;

mod five;
mod six;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse() {
            Ok(5) => {five::main() }
            Ok(6) => { six::main() },
            _ => { println!("Enter the day as an argument") }
        }
    }
    else {
        println!("Enter the day as an argument")
    }
}
