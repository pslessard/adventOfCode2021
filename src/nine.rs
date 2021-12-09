use bit_vec::BitVec;

extern crate pprof;
extern crate test;

// struct Location {
//     height: u8,

// }
// type CaveMap = 

pub fn main() {
    let lines = parse(utils::get_input(9, false));
    {
        let solved = solve_first(&lines);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

fn solve_first(coords: &Vec<Vec<u8>>) -> u64 {
    let mut total_risk = 0u64;
    for (y, row) in coords.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if (x != 0 && point >= &row[x-1]) ||
               (x + 1 != row.len() && point >= &row[x+1]) ||
               (y != 0 && point >= &coords[y-1][x]) ||
               (y + 1 != coords.len() && point >= &coords[y+1][x]) {
                continue;
            }
            total_risk += (point + 1) as u64
        }
    }
    total_risk
}

fn find_low_points(coords: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for (y, row) in coords.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if (x != 0 && point >= &row[x-1]) ||
               (x + 1 != row.len() && point >= &row[x+1]) ||
               (y != 0 && point >= &coords[y-1][x]) ||
               (y + 1 != coords.len() && point >= &coords[y+1][x]) {
                continue;
            }
            low_points.push((y, x))
        }
    }
    low_points
}

#[inline]
fn up_has_higher_neighbor(coords: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
    (p.1 != 0 && coords[p.0][p.1] < coords[p.0][p.1-1]) ||
    (p.1 + 1 != coords[0].len() && coords[p.0][p.1] < coords[p.0][p.1+1]) ||
    (p.0 + 1 != coords.len() && coords[p.0][p.1] < coords[p.0+1][p.1])
}

#[inline]
fn down_has_higher_neighbor(coords: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
    (p.1 != 0 && coords[p.0][p.1] < coords[p.0][p.1-1]) ||
    (p.1 + 1 != coords[0].len() && coords[p.0][p.1] < coords[p.0][p.1+1]) ||
    (p.0 != 0 && coords[p.0][p.1] < coords[p.0-1][p.1])
}

#[inline]
fn left_has_higher_neighbor(coords: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
    (p.1 + 1 != coords[0].len() && coords[p.0][p.1] < coords[p.0][p.1+1]) ||
    (p.0 != 0 && coords[p.0][p.1] < coords[p.0-1][p.1]) ||
    (p.0 + 1 != coords.len() && coords[p.0][p.1] < coords[p.0+1][p.1])
}

#[inline]
fn right_has_higher_neighbor(coords: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
    (p.1 != 0 && coords[p.0][p.1] < coords[p.0][p.1-1]) ||
    (p.0 != 0 && coords[p.0][p.1] < coords[p.0-1][p.1]) ||
    (p.0 + 1 != coords.len() && coords[p.0][p.1] < coords[p.0+1][p.1])
}

fn find_basin(coords: &Vec<Vec<u8>>, point: (usize, usize), checked: &mut Vec<BitVec>) -> usize {
    let mut size = 1usize;
    let mut coord_queue = Vec::new();
    coord_queue.push(point);

    checked[point.0].set(point.1, true);

    println!("Finding basin for {:?} height {}", point, coords[point.0][point.1]);

    while let Some(p) = coord_queue.pop() {
        println!("  Checking {:?} {}", p, checked[p.0][p.1]);

        if p.0 != 0 && !checked[p.0-1][p.1] && up_has_higher_neighbor(coords, (p.0 - 1, p.1)) {
            size += 1;
            coord_queue.push((p.0 - 1, p.1));
            checked[p.0-1].set(p.1, true);
            println!("    Adding to basin u: {:?} height {:?}", (p.0 - 1, p.1), coords[p.0-1][p.1]);
        }

        let new_p = (p.0 + 1, p.1);
        if new_p.0 != coords.len() && !checked[new_p.0][new_p.1] && down_has_higher_neighbor(coords, new_p) {
            size += 1;
            coord_queue.push(new_p);
            checked[p.0+1].set(p.1, true);
            println!("    Adding to basin d: {:?} height {:?}", new_p, coords[p.0+1][p.1]);
        }

        if p.1 != 0 && !checked[p.0][p.1-1] && left_has_higher_neighbor(coords, (p.0, p.1-1)) {
            size += 1;
            coord_queue.push((p.0, p.1-1));
            checked[p.0].set(p.1-1, true);
            println!("    Adding to basin l: {:?} height {:?}", (p.0, p.1-1), coords[p.0][p.1-1]);
        }

        let new_p = (p.0, p.1+1);
        if new_p.1 != coords[0].len() && !checked[new_p.0][new_p.1] && right_has_higher_neighbor(coords, new_p) {
            size += 1;
            coord_queue.push(new_p);
            checked[p.0].set(p.1+1, true);
            println!("    Adding to basin r: {:?} height {:?}", new_p, coords[p.0][p.1+1]);
        }
    }
    size
}

fn solve_second(coords: &Vec<Vec<u8>>) -> usize {
    let low_points = find_low_points(coords);
    println!("{:?}", low_points);
    let mut checked = vec![BitVec::from_elem(coords[0].len(), false); coords.len()];

    let mut sizes = low_points.iter().map(|lp| find_basin(&coords, *lp, &mut checked)).collect::<Vec<usize>>();
    println!("{:?}", sizes);
    sizes.sort_unstable();

    sizes.iter().rev().take(3).product()
}

fn parse(lines: Vec<String>) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(8, true));

        // start profiling
        // let guard = pprof::ProfilerGuard::new(100).unwrap();

        // run benchmark
        b.iter(|| solve_second(&lines));

        // build flamegraph
        // if let Ok(report) = guard.report().build() {
        //     use std::fs::File;
        //     let file = File::create("flamegraph.svg").unwrap();
        //     report.flamegraph(file).unwrap();
        // };

        // Put this into Cargo.toml if you want a useful flamegraph
        // [profile.release]
        // debug = true
    }
}
