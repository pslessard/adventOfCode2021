use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::cmp;
use std::time::Instant;


fn main() {
    let mut vectors: Vec<((i16, i16), (i16, i16))> = Vec::new();

    let mut x_max: usize = 0;
    let mut y_max: usize = 0;

    // file reading courtesy of https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines {
            let line = line.unwrap();
            let str_coords = line.split(" -> ");

            let mut coords: Vec<(i16, i16)> = Vec::new();

            for coord in str_coords {
                let split: Vec<&str> = coord.split(",").collect();
                
                let x: i16 = split[0].parse::<i16>().unwrap();
                let y: i16 = split[1].parse::<i16>().unwrap();

                if x as usize >= x_max {
                    x_max = x as usize + 1;
                }
                if y as usize >= y_max {
                    y_max = y as usize + 1;
                }

                coords.push((x, y));
            }

            vectors.push((coords[0], coords[1]));
        }

        let now = Instant::now();

        let mut plotted = vec![vec![0u8; y_max]; x_max];
        let mut danger = HashSet::new();

        for vector in &vectors {
            plot_line(vector.0, vector.1, &mut plotted, &mut danger);
        }

        println!("Number of dangerous areas: {}", danger.len());

        println!("Finished in : {:.2?}", now.elapsed())
    }
    else {
        println!("Failed to read input file")
    }
}

// file reading courtesy of https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Plotted = Vec<Vec<u8>>;

fn plot_line(start: (i16, i16), end: (i16, i16), plotted: &mut Plotted, danger: &mut HashSet<(i16, i16)>) {
    let x_diff = end.0 - start.0;
    let y_diff = end.1 - start.1;

    let x_direction = x_diff.signum();
    let y_direction = y_diff.signum();

    let first = false;
    if first && x_direction != 0 && y_direction != 0 {
        return;
    }

    assert!(x_diff.abs() == 0 || y_diff.abs() == 0 || x_diff.abs() == y_diff.abs());
    let magnitude = cmp::max(x_diff.abs(), y_diff.abs());

    for i in 0..(magnitude+1) {
        plot((start.0 + x_direction*i, start.1 + y_direction*i), plotted, danger)
    }
}

fn plot(coord: (i16, i16), plotted: &mut Plotted, danger: &mut HashSet<(i16, i16)>) {
    let point: &mut u8 = &mut plotted[coord.0 as usize][coord.1 as usize];
    if point == &1 {
        danger.insert(coord);
    }
    else {
        *point = 1;
    }
}
