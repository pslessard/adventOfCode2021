// use std::collections::HashMap;

extern crate pprof;
extern crate test;

type Input = u8;

pub fn main() {
    let lines = parse(utils::get_input(13, false));
    {
        let solved = solve_first(&lines);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

fn solve_first(_lines: &Input) -> u8 {
    0
}

fn solve_second(_lines: &Input) -> u8 {
    0
}

fn parse(_lines: Vec<String>) -> Input {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[bench]
    // fn bench(b: &mut Bencher) -> () {
    //     let lines = parse(utils::get_input(13, false));

    //     // start profiling
    //     // let guard = pprof::ProfilerGuard::new(100).unwrap();

    //     // run benchmark
    //     b.iter(|| solve_second(&lines));

    //     // build flamegraph
    //     // if let Ok(report) = guard.report().build() {
    //     //     use std::fs::File;
    //     //     let file = File::create("flamegraph.svg").unwrap();
    //     //     report.flamegraph(file).unwrap();
    //     // };

    //     // Put this into Cargo.toml if you want a useful flamegraph
    //     // [profile.release]
    //     // debug = true
    // }
}
