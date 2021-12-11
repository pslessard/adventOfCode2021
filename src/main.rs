#![feature(test)]
#![allow(non_snake_case)]

use std::env;

mod eight;
mod eleven;
mod five;
mod nine;
mod seven;
mod six;
mod ten;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse() {
            Ok(5) => five::main(),
            Ok(6) => six::main(),
            Ok(7) => seven::main(),
            Ok(8) => eight::main(),
            Ok(9) => nine::main(),
            Ok(10) => ten::main(),
            Ok(11) => eleven::main(),
            _ => {
                println!("Enter the day as an argument")
            }
        }
    } else {
        println!("Enter the day as an argument")
    }
}
