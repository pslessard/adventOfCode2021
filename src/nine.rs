use bit_vec::BitVec;
use std::collections::VecDeque;

extern crate pprof;
extern crate test;

// struct Location {
//     height: u8,

// }
// type CaveMap = 
type Point = (usize, usize);

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

fn solve_first(coords: &[Vec<u8>]) -> u64 {
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

fn find_low_points(coords: &[Vec<u8>]) -> Vec<Point> {
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
fn get(coords: &[Vec<u8>], p: Point) -> u8 {
    coords[p.0][p.1]
}

#[inline]
fn get_arr(coords: &[[usize; 100]], p: Point) -> usize {
    coords[p.0][p.1]
}

fn u(p: Point) -> Point {
    assert!(p.0 != 0);
    (p.0 - 1, p.1)
}
fn d(p: Point) -> Point {
    (p.0 + 1, p.1)
}
fn l(p: Point) -> Point {
    assert!(p.1 != 0);
    (p.0, p.1 - 1)
}
fn r(p: Point) -> Point {
    (p.0, p.1 + 1)
}

fn get_neighbors(p: Point, y_max: usize, x_max: usize) -> Vec<Point> {
    let mut v = Vec::new();
    if p.0 != 0 {
        v.push(u(p));
    }
    if p.0 + 1 != y_max {
        v.push(d(p));
    }
    if p.1 != 0 {
        v.push(l(p));
    }
    if p.1 + 1 != x_max {
        v.push(r(p));
    }
    v
}

fn find_basins(coords: &[Vec<u8>]) -> Vec<usize> {
    let mut basins: Vec<usize> = vec![0];

    let mut basin_coords: [[usize; 100]; 100] = [[0; 100]; 100];

    let mut points_to_check: [Vec<Point>; 9] = [Vec::new(), Vec::with_capacity(20), Vec::with_capacity(20), Vec::with_capacity(20), Vec::with_capacity(20), Vec::with_capacity(20), Vec::with_capacity(20), Vec::with_capacity(20), Vec::with_capacity(20)];

    for (y, row) in coords.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if *p == 0 {
                basins.push(1);
                basin_coords[y][x] = basins.len();
            }
            else if *p != 9 {
                points_to_check[*p as usize].push((y, x));
            }
        }
    }

    for list in points_to_check.iter().skip(1) {
        for p in list.iter() {
            let flows_to = get_neighbors(*p, coords.len(), coords[0].len()).iter().filter(|&other| get(coords, *other) < get(coords, *p)).map(|&other| get_arr(&basin_coords, other)).collect::<Vec<usize>>();

            if flows_to.is_empty() {
                basins.push(1);
                // println!("found new basin for {}: {}", i, basins.len());
                basin_coords[p.0][p.1] = basins.len();
            }

            else if flows_to.len() == 1 || flows_to.iter().min() == flows_to.iter().max() {
                // println!("found new basin for {}: {}", i, basins.len());
                basin_coords[p.0][p.1] = flows_to[0];
                basins[flows_to[0] - 1] += 1;
            }
            // else it's in between two basins, so it's in none of them
            else {
                assert!(get(coords, *p) == 9);
            }
        }
    }

    basins.sort_unstable();
    basins
}

fn get_unchecked_neighbors(p: Point, y_max: usize, x_max: usize, checked: &[BitVec]) -> Vec<Point> {
    let mut v = Vec::new();
    if p.0 != 0 && !checked[p.0 - 1][p.1] {
        v.push(u(p));
    }
    if p.0 + 1 != y_max && !checked[p.0 + 1][p.1] {
        v.push(d(p));
    }
    if p.1 != 0 && !checked[p.0][p.1 - 1] {
        v.push(l(p));
    }
    if p.1 + 1 != x_max && !checked[p.0][p.1 + 1] {
        v.push(r(p));
    }
    v
}

fn find_basin(coords: &[Vec<u8>], start: Point, checked: &mut [BitVec]) -> usize {
    let mut basin_size = 1;
    checked[start.0].set(start.1, true);

    let mut points_to_check: VecDeque<Point> = VecDeque::with_capacity(150);
    points_to_check.push_back(start);

    while let Some(p) = points_to_check.pop_front() {
        checked[p.0].set(p.1, true);
        if get(coords, p) != 9 {
            basin_size += 1;
            points_to_check.extend(get_unchecked_neighbors(p, coords.len(), coords[0].len(), &checked).iter());
        }
    }
    basin_size
}

fn find_basins_fast(coords: &[Vec<u8>], low_points: &[Point]) -> Vec<usize> {
    let mut checked = Vec::new();
    for _ in 0..100 {
        checked.push(BitVec::from_elem(low_points.len(), false))
    }
    low_points.iter().map(|&lp| find_basin(coords, lp, &mut checked)).collect()
}

fn solve_second(coords: &[Vec<u8>]) -> usize {
    // let low_points = find_low_points(coords);
    // let basins = find_basins_fast(coords, &low_points);
    let basins = find_basins(coords);

    // println!("{:?}", basins);
    basins.iter().rev().take(3).product()
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
        let lines = parse(utils::get_input(9, true));

        // start profiling
        // let guard = pprof::ProfilerGuard::new(100).unwrap();
        // let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread"]).build().unwrap();
        println!("abt to run");

        // run benchmark
        b.iter(|| solve_second(&lines));

        // build flamegraph
        println!("done running");
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

    // #[test]
    // fn test() -> () {
    //     let lines = parse(utils::get_input(9, true));

    //     // start profiling
    //     // let guard = pprof::ProfilerGuard::new(100).unwrap();
    //     let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread"]).build().unwrap();
    //     println!("abt to run");

    //     // run benchmark
    //     solve_second(&lines);
    //     // b.iter(|| solve_second(&lines));

    //     // build flamegraph
    //     println!("done running");
    //     if let Ok(report) = guard.report().build() {
    //         println!("built report");
    //         use std::fs::File;
    //         let file = File::create("flamegraph.svg").unwrap();
    //         println!("file created");
    //         report.flamegraph(file).unwrap();
    //         println!("flamegraph created");
    //     };

    //     // Put this into Cargo.toml if you want a useful flamegraph
    //     // [profile.release]
    //     // debug = true
    // }
}
