use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::cmp;


fn main() {
    // file reading courtesy of https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    use std::time::Instant;
    let start = Instant::now();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut plotted = HashSet::new();
        let mut danger = HashSet::new();

        for line in lines {
            let line = line.unwrap();
            let str_coords = line.split(" -> ");

            let mut coords: Vec<(i32, i32)> = Vec::new();

            for coord in str_coords {
                let split: Vec<&str> = coord.split(",").collect();
                let x: i32 = split[0].parse::<i32>().unwrap();
                let y: i32 = split[1].parse::<i32>().unwrap();

                coords.push((x, y));
            }

            plot_line(coords[0], coords[1], &mut plotted, &mut danger);
        }

        println!("Number of dangerous areas: {}", danger.len());
    }
    else {
        println!("Failed to read input file")
    }

    println!("Finished in : {:.2?}", start.elapsed())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn plot_line(start: (i32, i32), end: (i32, i32), plotted: &mut HashSet<(i32, i32)>, danger: &mut HashSet<(i32, i32)>) {
    // println!("Plotting: {:?} -> {:?}", start, end);

    let x_diff = end.0 - start.0;
    let y_diff = end.1 - start.1;

    let x_direction = x_diff.signum();
    let y_direction = y_diff.signum();

    let first = false;
    if first && x_direction != 0 && y_direction != 0 {
        // println!("Skipping");
        return;
    }

    assert!(x_diff.abs() == 0 || y_diff.abs() == 0 || x_diff.abs() == y_diff.abs());
    let magnitude = cmp::max(x_diff.abs(), y_diff.abs());

    for i in 0..(magnitude+1) {
        plot((start.0 + x_direction*i, start.1 + y_direction*i), plotted, danger)
    }
}

fn plot(coord: (i32, i32), plotted: &mut HashSet<(i32, i32)>, danger: &mut HashSet<(i32, i32)>) {
    // println!("  Plotting: {:?}", coord);
    let added = plotted.insert(coord.clone());
    if !added {
        // println!("    DANGER! {:?}", coord);
        danger.insert(coord);
    }
}
