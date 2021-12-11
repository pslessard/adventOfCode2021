extern crate pprof;
extern crate test;

type Line = Vec<char>;

pub fn main() {
    let lines = parse(utils::get_input(10, true));
    {
        let solved = solve_first(&lines);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

fn get_corruption_score(line: &Line) -> usize {
    let mut stack = Vec::with_capacity(60);
    for ch in line {
        match ch {
            '(' => stack.push(ch),
            '[' => stack.push(ch),
            '{' => stack.push(ch),
            '<' => stack.push(ch),
            ')' => {
                let open = stack.pop().unwrap();
                if *open != '(' {
                    return 3;
                }
            }
            ']' => {
                let open = stack.pop().unwrap();
                if *open != '[' {
                    return 57;
                }
            }
            '}' => {
                let open = stack.pop().unwrap();
                if *open != '{' {
                    return 1197;
                }
            }
            '>' => {
                let open = stack.pop().unwrap();
                if *open != '<' {
                    return 25137;
                }
            }
            _ => {
                panic!("invalid character")
            }
        }
    }
    0
}

fn solve_first(lines: &[Line]) -> usize {
    lines.iter().map(|line| get_corruption_score(line)).sum()
}

fn complete_line(line: &Line) -> Option<u64> {
    let mut stack: Vec<char> = Vec::with_capacity(60);

    for ch in line {
        match ch {
            '(' => stack.push(*ch),
            '[' => stack.push(*ch),
            '{' => stack.push(*ch),
            '<' => stack.push(*ch),
            ')' => {
                let open = stack.pop().unwrap();
                if open != '(' {
                    return None;
                }
            }
            ']' => {
                let open = stack.pop().unwrap();
                if open != '[' {
                    return None;
                }
            }
            '}' => {
                let open = stack.pop().unwrap();
                if open != '{' {
                    return None;
                }
            }
            '>' => {
                let open = stack.pop().unwrap();
                if open != '<' {
                    return None;
                }
            }
            _ => {
                panic!("invalid character")
            }
        }
    }

    if !stack.is_empty() {
        // let mut completion = Vec::with_capacity(stack.len());
        let mut score = 0u64;
        for open in stack.iter().rev() {
            score *= 5;

            score += match open {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => {
                    panic!("invalid character")
                }
            }
        }

        // Some(completion)
        Some(score)
    } else {
        None
    }
}

// fn calculate_completion_score(completions)

fn solve_second(lines: &[Line]) -> u64 {
    let mut val: Vec<u64> = lines
        .iter()
        .filter_map(|line| complete_line(line))
        .collect();
    val.sort_unstable();
    println!("{:?}", val);
    val[val.len() / 2]
}

fn parse(lines: Vec<String>) -> Vec<Line> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(10, true));

        // start profiling
        // let guard = pprof::ProfilerGuard::new(100).unwrap();
        // let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread"]).build().unwrap();
        // println!("abt to run");

        // run benchmark
        b.iter(|| solve_second(&lines));

        // build flamegraph
        // println!("done running");
        // if let Ok(report) = guard.report().build() {
        //     println!("built report");
        //     use std::fs::File;
        //     let file = File::create("flamegraph.svg").unwrap();
        //     println!("file created");
        //     report.flamegraph(file).unwrap();
        //     println!("flamegraph created");
        // };

        // Put this into Cargo.toml if you want a useful flamegraph
        // [profile.release]
        // debug = true
    }
}
