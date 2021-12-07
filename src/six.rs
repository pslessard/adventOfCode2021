use std::time::Instant;
use test::Bencher;

extern crate test;


pub fn main() {
    let line = utils::get_input(6)[0].clone();
    let ages: Vec<u8> = parse_input(&line);

    let now = Instant::now();

    let num_fish = run_simulation(&ages);

    println!("Finished in : {:.2?}", now.elapsed());

    println!("{} fish after 80 days", num_fish);
}


fn parse_input(line: &String) -> Vec<u8> {
    line.split(",").map(&parse_to_u8).collect()
}


fn parse_to_u8(string: &str) -> u8 {
    string.parse::<u8>().unwrap()
}


const DURATION: u16 = 256;

fn run_simulation(ages: &Vec<u8>) -> u64 {
    let mut current_ages = [0u64; 9];

    for fish in ages {
        current_ages[*fish as usize] += 1;
    }

    for _ in 0..DURATION {
        let spawning = current_ages[0];
        
        for i in 0..8 {
            current_ages[i] = current_ages[i+1];
        }

        current_ages[6] += spawning;
        current_ages[8] = spawning;
    }

    current_ages.iter().sum()
}

fn run_simulation_pre_arranged(current_ages: &mut [u64; 9]) -> u64 {
    for _ in 0..DURATION {
        let spawning = current_ages[0];
        
        for i in 0..8 {
            current_ages[i] = current_ages[i+1];
        }

        current_ages[6] += spawning;
        current_ages[8] = spawning;
    }

    current_ages.iter().sum()
}


#[bench]
fn bench_simulation(b: &mut Bencher) -> () {
    let line = utils::get_input(6)[0].clone();
    let ages: Vec<u8> = parse_input(&line);

    b.iter(|| run_simulation(&ages));
}

#[bench]
fn bench_simulation_pre_arranged(b: &mut Bencher) -> () {
    let line = utils::get_input(6)[0].clone();
    let ages: Vec<u8> = parse_input(&line);

    let mut current_ages = [0u64; 9];

    for fish in ages {
        current_ages[fish as usize] += 1;
    }

    b.iter(|| run_simulation_pre_arranged(&mut current_ages));
}
