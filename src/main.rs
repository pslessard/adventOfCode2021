#![feature(test)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::env;

mod eight;
mod eleven;
mod five;
mod nine;
mod seven;
mod six;
mod ten;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;
mod sixteen;
mod seventeen;
mod eighteen;
mod nineteen;
mod twenty;
mod twentyone;
mod twentytwo;
mod twentythree;
mod twentyfour;
mod twentyfive;

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
            Ok(12) => twelve::main(),
            Ok(13) => thirteen::main(),
            Ok(14) => fourteen::main(),
            Ok(15) => fifteen::main(),
            Ok(16) => sixteen::main(),
            Ok(17) => seventeen::main(),
            Ok(18) => eighteen::main(),
            Ok(19) => nineteen::main(),
            Ok(20) => twenty::main(),
            Ok(21) => twentyone::main(),
            Ok(22) => twentytwo::main(),
            Ok(23) => twentythree::main(),
            Ok(24) => twentyfour::main(),
            Ok(25) => twentyfive::main(),
            _ => {
                println!("Enter the day as an argument")
            }
        }
    } else {
        println!("Enter the day as an argument")
    }
}
