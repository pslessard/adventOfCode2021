use bit_vec::BitVec;
use std::collections::BinaryHeap;

extern crate pprof;
extern crate test;

// struct Location {
//     height: u8,

// }
// type CaveMap =
type Point = (usize, usize);
type Row = Vec<u8>;
type Grid = Vec<Row>;

pub fn main() {
    let lines = parse(utils::get_input(9, true));
    {
        let solved = solve_first(&lines);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

fn solve_first(coords: &Grid) -> u64 {
    let mut total_risk = 0u64;
    for (y, row) in coords.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if (x != 0 && point >= &row[x - 1])
                || (x + 1 != row.len() && point >= &row[x + 1])
                || (y != 0 && point >= &coords[y - 1][x])
                || (y + 1 != coords.len() && point >= &coords[y + 1][x])
            {
                continue;
            }
            total_risk += (point + 1) as u64
        }
    }
    total_risk
}

#[allow(dead_code)]
fn find_low_points(coords: &Grid) -> Vec<Point> {
    let mut low_points: Vec<Point> = Vec::new();
    for (y, row) in coords.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if (x != 0 && point >= &row[x - 1])
                || (x + 1 != row.len() && point >= &row[x + 1])
                || (y != 0 && point >= &coords[y - 1][x])
                || (y + 1 != coords.len() && point >= &coords[y + 1][x])
            {
                continue;
            }
            low_points.push((y, x))
        }
    }
    low_points
}

// fn is_low_point(x: usize, y: usize, val: u8, row: &[u8], coords: &[Vec<u8>]) -> Option<Point> {
//     match (x != 0 && val >= row[(x)-1]) ||
//     (x + 1 != row.len() && val >= row[x+1]) ||
//     (y != 0 && val >= coords[y-1][x]) ||
//     (y + 1 != coords.len() && val >= coords[y+1][x]) {
//         true => Some((y, x)),
//         false => None
//     }
// }

// fn find_low_points_fast(coords: &[Vec<u8>]) -> Vec<Point> {
//     coords.iter().enumerate().flat_map(move |(y, row)| row.iter().enumerate().filter_map(move |(x, p)| is_low_point(x, y, *p, row, coords))).collect()
// }

#[inline]
fn get(coords: &Grid, p: Point) -> u8 {
    coords[p.0][p.1]
}

#[inline]
fn u(p: Point) -> Point {
    assert!(p.0 != 0);
    (p.0 - 1, p.1)
}
#[inline]
fn d(p: Point) -> Point {
    (p.0 + 1, p.1)
}
#[inline]
fn l(p: Point) -> Point {
    assert!(p.1 != 0);
    (p.0, p.1 - 1)
}
#[inline]
fn r(p: Point) -> Point {
    (p.0, p.1 + 1)
}

fn get_unchecked_neighbors(
    p: Point,
    y_max: usize,
    x_max: usize,
    coords: &Grid,
    checked: &mut [BitVec],
) -> Vec<Point> {
    let mut v = Vec::new();
    if p.0 != 0 && !checked[p.0 - 1][p.1] && get(coords, u(p)) != 9 {
        let n = u(p);
        checked[n.0].set(n.1, true);
        v.push(n);
    }
    if p.0 + 1 != y_max && !checked[p.0 + 1][p.1] && get(coords, d(p)) != 9 {
        let n = d(p);
        checked[n.0].set(n.1, true);
        v.push(n);
    }
    if p.1 != 0 && !checked[p.0][p.1 - 1] && get(coords, l(p)) != 9 {
        let n = l(p);
        checked[n.0].set(n.1, true);
        v.push(n);
    }
    if p.1 + 1 != x_max && !checked[p.0][p.1 + 1] && get(coords, r(p)) != 9 {
        let n = r(p);
        checked[n.0].set(n.1, true);
        v.push(n);
    }
    v
}

fn check(coords: &Grid, p: Point, checked: &mut [BitVec]) -> usize {
    // println!("  Adding to basin: {:?} - {}", p, get(coords, p));

    let neighbors = get_unchecked_neighbors(p, coords.len(), coords[0].len(), coords, checked);
    match neighbors.len() {
        0 => 1,
        1 => check(coords, neighbors[0], checked) + 1,
        2 => check(coords, neighbors[0], checked) + check(coords, neighbors[1], checked) + 1,
        3 => {
            check(coords, neighbors[0], checked)
                + check(coords, neighbors[1], checked)
                + check(coords, neighbors[2], checked)
                + 1
        }
        4 => {
            check(coords, neighbors[0], checked)
                + check(coords, neighbors[1], checked)
                + check(coords, neighbors[2], checked)
                + check(coords, neighbors[3], checked)
                + 1
        }
        _ => {
            panic!("invalid length")
        }
    }
}

fn check_low_point(coords: &Grid, p: Point, checked: &mut [BitVec]) -> usize {
    // println!("Basin: {:?}: {}", p, get(coords, p));
    checked[p.0].set(p.1, true);
    check(coords, p, checked)
}

#[allow(dead_code)]
fn check_low_point_fast(coords: &Grid, p: Point, checked: &mut [BitVec]) -> usize {
    // println!("Basin: {:?}: {}", p, get(coords, p));
    checked[p.0].set(p.1, true);
    check(coords, p, checked)
}

#[allow(dead_code)]
fn find_basins(coords: &Grid, low_points: &[Point]) -> Vec<usize> {
    let mut checked = vec![BitVec::from_elem(100, false); 100];

    low_points
        .iter()
        .map(|&lp| check_low_point(coords, lp, &mut checked))
        .collect()
}

#[allow(dead_code)]
fn find_basins_fast(coords: &Grid, low_points: &[Point]) -> Vec<usize> {
    let mut checked = vec![BitVec::from_elem(100, false); 100];

    low_points
        .iter()
        .map(|&lp| check_low_point_fast(coords, lp, &mut checked))
        .collect()
}

#[allow(dead_code)]
fn solve_second_test(coords: &Grid) -> usize {
    let low_points = find_low_points(coords);
    let mut basins = find_basins(coords, &low_points);
    basins.sort_unstable();

    // println!("{:?}", basins);
    basins.iter().rev().take(3).product()
}

fn solve_second(coords: &Grid) -> usize {
    let mut checked = vec![BitVec::from_elem(100, false); 100];
    let mut heap = BinaryHeap::new();

    for (y, row) in coords.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if (x != 0 && point >= &row[x - 1])
                || (x + 1 != row.len() && point >= &row[x + 1])
                || (y != 0 && point >= &coords[y - 1][x])
                || (y + 1 != coords.len() && point >= &coords[y + 1][x])
            {
                continue;
            }
            heap.push(check_low_point(coords, (y, x), &mut checked));
        }
    }

    // println!("{:?}", basins);
    heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap()
}

fn parse(lines: Vec<String>) -> Grid {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // fn clone() {
    //     let mut checked = Vec::new();
    //     let bv = BitVec::from_elem(100, false);
    //     for _ in 0..100 {
    //         checked.push(bv.clone())
    //     }
    //     checked.push(bv);
    // }
    // fn normal() {
    //     let mut checked = Vec::new();
    //     for _ in 0..100 {
    //         checked.push(BitVec::from_elem(100, false))
    //     }
    // }
    // fn array() {
    //     let mut checked = vec![BitVec::from_elem(100, false)];
    // }

    // #[bench]
    // fn bench_bit_vecs_other(b: &mut Bencher) -> () {
    //     b.iter(|| array());
    // }
    #[bench]
    fn bench_partial(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(9, true));
        b.iter(|| solve_second_test(&lines));
    }

    #[bench]
    fn bench_fast(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(9, true));

        // start profiling
        // let guard = pprof::ProfilerGuard::new(100).unwrap();
        // let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread"]).build().unwrap();

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
