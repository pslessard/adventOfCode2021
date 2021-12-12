// use bit_vec::BitVec;

extern crate pprof;
extern crate test;

type Point = (usize, usize);
type IterationCount = u8;
type Unit = (IterationCount, u8);
type Row = Vec<Unit>;
type Grid = Vec<Row>;

pub fn main() {
    let lines = parse(utils::get_input(11, true));
    // {
    //     let solved = solve_first(&lines);
    //     println!("{:?}", solved)
    // }
    {
        let solved = solve_first_fast(&lines);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

fn get_unflashed_neighbors<'a>(
    coords: &'a Grid,
    iteration: IterationCount,
    p: Point,
) -> impl std::iter::Iterator<Item = Point> + 'a {
    #[derive(PartialEq)]
    enum Direction {
        None,
        UpLeft,
        Up,
        UpRight,
        Right,
        DownRight,
        Down,
        DownLeft,
        Left
    }

    let mut last = Direction::None;
    
    std::iter::from_fn(move || {
        if last == Direction::None {
            last = Direction::UpLeft;
            if p.0 != 0 && p.1 != 0 && !did_flash(iteration, coords, ul(p)) {
                return Some(ul(p));
            }
        }
        if last == Direction::UpLeft {
            last = Direction::Up;
            if p.0 != 0 && !did_flash(iteration, coords, u(p)) {
                return Some(u(p));
            }
        }
        if last == Direction::Up {
            last = Direction::UpRight;
            if p.0 != 0 && p.1 + 1 != coords[0].len() && !did_flash(iteration, coords, ur(p)) {
                return Some(ur(p));
            }
        }
        if last == Direction::UpRight {
            last = Direction::Right;
            if p.1 + 1 != coords[0].len() && !did_flash(iteration, coords, r(p)) {
                return Some(r(p));
            }
        }
        if last == Direction::Right {
            last = Direction::DownRight;
            if p.0 + 1 != coords.len() && p.1 + 1 != coords[0].len() && !did_flash(iteration, coords, dr(p)) {
                return Some(dr(p));
            }
        }
        if last == Direction::DownRight {
            last = Direction::Down;
            if p.0 + 1 != coords.len() && !did_flash(iteration, coords, d(p)) {
                return Some(d(p));
            }
        }
        if last == Direction::Down {
            last = Direction::DownLeft;
            if p.0 + 1 != coords.len() && p.1 != 0 && !did_flash(iteration, coords, dl(p)) {
                return Some(dl(p));
            }
        }
        if last == Direction::DownLeft {
            last = Direction::Left;
            if p.1 != 0 && !did_flash(iteration, coords, l(p)) {
                return Some(l(p));
            }
        }
        return None;
    })
}

fn run_cycle_fast(coords: &mut Grid, queue: &mut [Point], iteration: IterationCount) -> u64 {
    let mut flashes = 0u64;

    let mut idx = 0usize;

    // println!("adding initial to queue");

    for (y, row) in coords.iter_mut().enumerate() {
        for (x, _) in row.iter_mut().enumerate() {
            queue[idx] = (y, x);
            idx += 1;
        }
    }

    // println!("running through queue");

    while idx != 0 {
        idx -= 1;
        let p = queue[idx];

        let (last_flash, val) = get(coords, p);
        if val == 9 {
            for n in get_unflashed_neighbors(coords, iteration, p) {
                queue[idx] = n;
                idx += 1;
            }
            flashes += 1;
            coords[p.0][p.1] = (iteration, 0);
        }
        else if last_flash != iteration {
            coords[p.0][p.1] = (last_flash, val + 1);
        }
    }
    
    flashes
}

fn solve_first_fast(input: &Grid) -> u64 {
    let mut coords = input.clone();
    let mut flashes = 0u64;
    let mut queue = [(0, 0); 200];

    for i in 0..100 {
        flashes += run_cycle_fast(&mut coords, &mut queue, i + 1);
    }

    flashes
}

fn solve_second(input: &Grid) -> u64 {
    let mut coords = input.clone();
    let mut counter = 0u64;
    let mut queue = [(0, 0); 400];

    loop {
        counter += 1;
        if run_cycle_fast(&mut coords, &mut queue, counter as IterationCount) == (coords.len() * coords[0].len()) as u64 {
            return counter
        }
        // println!("{}", counter);
    }
}

#[inline]
fn get(coords: &Grid, p: Point) -> Unit {
    coords[p.0][p.1]
}

#[inline]
fn did_flash(iteration: IterationCount, coords: &Grid, p: Point) -> bool {
    iteration == get(coords, p).0
}

#[inline]
fn u(p: Point) -> Point {
    assert!(p.0 != 0);
    (p.0 - 1, p.1)
}
#[inline]
fn ul(p: Point) -> Point {
    assert!(p.0 != 0);
    assert!(p.1 != 0);
    (p.0 - 1, p.1 - 1)
}
#[inline]
fn ur(p: Point) -> Point {
    assert!(p.0 != 0);
    (p.0 - 1, p.1 + 1)
}
#[inline]
fn d(p: Point) -> Point {
    (p.0 + 1, p.1)
}
#[inline]
fn dl(p: Point) -> Point {
    assert!(p.1 != 0);
    (p.0 + 1, p.1 - 1)
}
#[inline]
fn dr(p: Point) -> Point {
    (p.0 + 1, p.1 + 1)
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

fn parse(lines: Vec<String>) -> Grid {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| (0, ch.to_digit(10).unwrap() as u8))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(11, false));

        // start profiling
        let guard = pprof::ProfilerGuard::new(100).unwrap();

        // run benchmark
        b.iter(|| solve_first_fast(&lines));

        // build flamegraph
        if let Ok(report) = guard.report().build() {
            use std::fs::File;
            let file = File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
        };

        // Put this into Cargo.toml if you want a useful flamegraph
        // [profile.release]
        // debug = true
    }

    #[bench]
    fn bench_fast(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(11, false));

        b.iter(|| solve_first_fast(&lines));
    }
}
