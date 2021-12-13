use bit_vec::BitVec;
use bit_reverse::ParallelReverse;
use std::iter;

extern crate pprof;
extern crate test;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Axis { Y, X }
type Fold = (Axis, usize);
type Row = BitVec;
type Grid = Vec<Row>;
type Input = (Vec<Row>, Vec<Fold>);
type Point = (usize, usize);
type Range = Point;

pub fn main() {
    let lines = parse(utils::get_input(13, true));
    unsafe {
        println!("{:?}", &lines.0[0].storage());
        // println!("{:?}", &lines.0[0].iter().map(|byte| byte.swap_bits()).rev().collect::<Vec<u8>>());
    }
    {
        let solved = solve_first(&lines.0, &lines.1);
        println!("{:?}", solved)
    }
    {
        let solved = solve_second(&lines.0, &lines.1);
        print_grid(&solved)
    }
}

fn fold_up(grid: &mut Grid, fold: usize) {
    let from = grid.split_off(fold).split_off(1);

    for i in 0..fold {
        let top = grid.len();
        grid[top - i - 1].or(&from[i]);
    }
}


fn fold_down(grid: &mut Grid, fold: usize) {
    let mut into = grid.split_off(fold).split_off(1);
    for i in 0..fold {
        let top = grid.len();
        into[i].or(&grid[top - i - 1]);
    }
    *grid = into;
}

fn execute_horizontal_fold(grid: &mut Grid, fold: usize) {
    if (grid.len() - fold) > fold {
        fold_down(grid, fold)
    }
    else {
        fold_up(grid, fold)
    }
}


fn fold_left(grid: &mut Grid, fold: usize, current_range: Range) -> Range {
    let pivot = current_range.0 + fold;
    for y in 0..grid.len() {
        let row: &mut Row = &mut grid[y];
        let mut from = row.split_off(fold).split_off(1);
        let padding = iter::repeat(false).take(row.len() - from.len());
        from = padding.chain(from.iter().rev()).collect();
        row.or(&from);
    }
    (current_range.0, pivot)
}


fn fold_right(grid: &mut Grid, fold: usize, current_range: Range) -> Range {
    let pivot = current_range.0 + fold;
    for y in 0..grid.len() {
        let row: &mut Row = &mut grid[y];
        let mut into = row.split_off(fold).split_off(1);
        let padding = iter::repeat(false).take(into.len() - row.len());
        *row = padding.chain(row.iter().rev()).collect();
        into.or(&row);
        *row = into;
    }
    (pivot + 1, current_range.1)
}

fn execute_vertical_fold(grid: &mut Grid, fold: usize, current_range: Range) -> Range {
    let width = current_range.1 - current_range.0;
    if (width - fold)-1 > fold {
        fold_right(grid, fold, current_range)
    }
    else {
        fold_left(grid, fold, current_range)
    }
}

fn execute_fold(grid: &mut Grid, fold: Fold, current_range: Range) -> Range {
    match fold.0 {
        Axis::X => { execute_horizontal_fold(grid, fold.1); current_range }
        Axis::Y => { execute_vertical_fold(grid, fold.1, current_range) }
    }
}

fn get_num_dots(grid: &Grid, current_range: Range) -> usize {
    let mut count = 0usize;
    for y in 0..grid.len() {
        for x in current_range.0..current_range.1 {
            if grid[y][x] {
                count += 1;
            }
        }
    }
    count
}

fn solve_first(in_grid: &Vec<Row>, folds: &[Fold]) -> usize {
    let mut grid = in_grid.clone();
    let mut range = (0, grid[0].len());

    range = execute_fold(&mut grid, folds[0], range);

    get_num_dots(&grid, range)
}

fn print_grid(grid: &[String]) {
    for row in grid.iter().rev() {
        println!("{}", row);
    }
}

fn bit_to_char(bit: bool) -> char {
    if bit {
        '#'
    }
    else {
        ' '
    }
}

fn solve_second(in_grid: &Vec<Row>, folds: &[Fold]) -> Vec<String> {
    let mut grid = in_grid.clone();
    let mut range = (0, grid[0].len());

    for fold in folds {
        range = execute_fold(&mut grid, *fold, range);
    }
    grid.iter().map(|bitvec| bitvec.iter().skip(range.0).take(range.1 - range.0).map(|bit| bit_to_char(bit)).collect::<String>()).collect::<Vec<String>>()
}

fn parse(lines: Vec<String>) -> Input {
    let mut dots = Vec::new();
    let mut folds = Vec::new();

    let mut w = 0usize;
    let mut h = 0usize;

    for line in lines {
        if line.contains(',') {
            let indices: Vec<usize> = line.split(',').map(|string| string.parse::<usize>().unwrap()).collect();
            assert_eq!(indices.len(), 2);
            let x = indices[0];
            let y = indices[1];

            if x > w {
                w = x
            }
            if y > h {
                h = y
            }
            dots.push((x, y));
        }
        else if line.contains('=') {
            let split: Vec<&str> = line.split(' ').collect();
            assert_eq!(split.len(), 3);
            let (ax_str, idx_str) = split[2].split_once('=').unwrap();
            let axis = match ax_str {
                "x" => Axis::Y,
                "y" => Axis::X,
                _ => { panic!("invalid axis"); }
            };
            let idx = idx_str.parse::<usize>().unwrap();
            folds.push((axis, idx));
        }
    }

    let mut grid = vec![BitVec::from_elem(w+1, false); h+1];
    for p in dots {
        grid[p.1].set(p.0, true);
    }

    (grid, folds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn thirteen(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(13, true));

        // start profiling
        let guard = pprof::ProfilerGuard::new(100).unwrap();

        // run benchmark
        b.iter(|| solve_second(&lines.0, &lines.1));

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
}
