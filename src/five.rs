use std::cmp;
use std::collections::HashSet;
use std::time::Instant;

extern crate test;

type Vectors = Vec<((i16, i16), (i16, i16))>;

pub fn main() {
    let (vectors, x_max, y_max) = parse_input(utils::get_input(5, true));

    let now = Instant::now();

    let danger = solve(&vectors, x_max, y_max);

    println!("Finished in : {:.2?}", now.elapsed());

    println!("Number of dangerous areas: {}", danger);
}

fn parse_input(lines: Vec<String>) -> (Vectors, usize, usize) {
    let mut vectors: Vectors = Vec::new();
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;

    for line in lines {
        let str_coords = line.split(" -> ");

        let mut coords: Vec<(i16, i16)> = Vec::new();

        for coord in str_coords {
            let split: Vec<&str> = coord.split(',').collect();

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

    (vectors, x_max, y_max)
}

fn solve(vectors: &Vectors, x_max: usize, y_max: usize) -> usize {
    let mut plotted = vec![vec![0u8; y_max]; x_max];
    let mut danger = HashSet::new();

    for vector in vectors {
        plot_line(vector.0, vector.1, &mut plotted, &mut danger);
    }

    danger.len()
}

type Plotted = Vec<Vec<u8>>;

fn plot_line(
    start: (i16, i16),
    end: (i16, i16),
    plotted: &mut Plotted,
    danger: &mut HashSet<(i16, i16)>,
) {
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

    for i in 0..(magnitude + 1) {
        plot(
            (start.0 + x_direction * i, start.1 + y_direction * i),
            plotted,
            danger,
        )
    }
}

fn plot(coord: (i16, i16), plotted: &mut Plotted, danger: &mut HashSet<(i16, i16)>) {
    let point: &mut u8 = &mut plotted[coord.0 as usize][coord.1 as usize];
    if point == &1 {
        danger.insert(coord);
    } else {
        *point = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_simulation(b: &mut Bencher) -> () {
        let (vectors, x_max, y_max) = parse_input(utils::get_input(5, true));

        b.iter(|| solve(&vectors, x_max, y_max));
    }
}
