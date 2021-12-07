use std::time::Instant;
use test::Bencher;
use std::fs::File;

extern crate pprof;
extern crate test;

static mut cache_checks: u16 = 0;
static mut cache_hits: u16 = 0;
static mut cache_misses: u16 = 0;

pub fn main() {
    let mut coords = parse(utils::get_input(7, true)[0].clone());
    let total_cost = solve(&mut coords);
    println!("{:?}", total_cost);

    unsafe { println!("cache checks: {} - hits: {}, misses: {}", cache_checks, cache_hits, cache_misses) }
}

fn get_nth_triangle(num: i64) -> i64 {
    num * (num + 1) / 2
}

fn get_cost(coord: &i16, median: i16) -> i64 {
    let part1 = false;
    match part1 {
        true => { (coord - median).abs() as i64 }
        false => { get_nth_triangle((coord - median).abs() as i64) }
    }
}

fn get_total_cost(coords: &Vec<i16>, value: i16) -> i64 {
    coords.iter().map(|coord| get_cost(coord, value)).fold(0i64, |l, r| l + r)
}

fn get_last(coords: &[i16]) -> i16 {
    coords.last().copied().unwrap()
}

fn get_or_calc_total_cost(coords: &Vec<i16>, value: i16, values: &mut Vec<i64>) -> i64 {
    // unsafe { cache_checks += 1; }
    match values[value as usize] {
        0 => {
            // unsafe { cache_misses += 1 };
            values[value as usize] = get_total_cost(coords, value);
            values[value as usize]
        }
        val => {
            // unsafe { cache_hits += 1; }
            val
        }
    }
}

// coords must be sorted
fn search_for_min(coords: &Vec<i16>, min: i16, mid: i16, max: i16, values: &mut Vec<i64>) -> i64 {
    assert!(max >= min);

    if max == min {
        return get_or_calc_total_cost(coords, min, values)
    }
    else if (max - min) <= 3 {
        // println!("{}, {}", min, max);
        return (min..max).map(|val| get_or_calc_total_cost(coords, val, values)).min().unwrap();
    }

    // println!("mid {} - {} /2 = {}", max, min, mid);
    let mid_cost = get_or_calc_total_cost(coords, mid, values);

    let up = ((max - mid) / 2) + mid;
    // println!("up {} - {} => {}", max, mid, up);
    let up_cost = get_or_calc_total_cost(coords, up, values);

    if up_cost < mid_cost {
        return search_for_min(coords, mid, up, max, values)
    }

    let down = ((mid - min) / 2) + min;
    // println!("down {}, {}, {}", mid, min, down);
    let down_cost = get_or_calc_total_cost(coords, down, values);

    if down_cost < mid_cost {
        return search_for_min(coords, min, down, mid, values)
    }
    else {
        search_for_min(coords, down, mid, up, values)
    }
}

fn solve(coords: &mut Vec<i16>) -> i64 {
    coords.sort_unstable();

    let mut values = vec![0i64; (get_last(coords)+1) as usize];

    let first = coords[0];
    let median = coords[coords.len() / 2];
    assert!(coords.len() > 2);
    let last = get_last(coords);

    // println!("{}, {}, {}", first, median, last);

    let bounds = match (last - median) - (first - median) {
        d if d < 0 => {
            // println!("Low is sparser: {}", (first + ((median - first) / 2)));
            let temp = first + ((median - first) / 2);
            (first, temp, median)
        }
        d if d > 0 => {
            // println!("High is sparser: {}", (median + ((last - median) / 2)));
            let temp = median + ((last - median) / 2);
            (median, temp, last)
        }
        _ => (first, median, last)
    };
    // println!("{:?}", bounds);

    search_for_min(&coords, bounds.0, bounds.1, bounds.2, &mut values)
    // search_for_min(&coords, first, median, last, &mut values)
}

fn parse(line: String) -> Vec<i16> {
    line.split(",").map(|num| num.parse().unwrap()).collect()
}

#[bench]
fn bench_simulation(b: &mut Bencher) -> () {
    let mut coords = parse(utils::get_input(7, true)[0].clone());
    
    // start profiling
    // let guard = pprof::ProfilerGuard::new(1000).unwrap();

    // run benchmark
    b.iter(|| solve(&mut coords));

    // if let Ok(report) = guard.report().build() {
    //     let file = File::create("flamegraph.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    // };
}
