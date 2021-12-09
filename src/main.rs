#![feature(test)]
#![allow(non_snake_case)]

use std::env;

mod eight;
mod five;
mod seven;
mod six;
mod nine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse() {
            Ok(5) => five::main(),
            Ok(6) => six::main(),
            Ok(7) => seven::main(),
            Ok(8) => eight::main(),
            Ok(9) => nine::main(),
            _ => {
                println!("Enter the day as an argument")
            }
        }
    } else {
        println!("Enter the day as an argument")
    }
}
