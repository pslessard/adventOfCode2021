use smallvec::SmallVec;

extern crate pprof;
extern crate test;

static mut BIT_COUNTS: [u8; 256] = [0u8; 256];

type Display = u8;
type Displays10 = SmallVec<[Display; 10]>;
type Displays4 = SmallVec<[Display; 4]>;
type SeenDisplays = [SmallVec<[Display; 3]>; 10];
type ParsedPair = (SmallVec<[String; 10]>, SmallVec<[String; 4]>);

pub fn main() {
    let mut bit_counts = [0u8; 256];
    for i in 0..256 {
        assert!(bit_counts.len() >= 255);
        bit_counts[i] = ((i & 1) as u8) + bit_counts[(i as usize) / 2]
    }
    unsafe { BIT_COUNTS = bit_counts }

    let lines = parse(utils::get_input(8, true));
    {
        let solved = solve_first(lines.iter().map(|pair| (pair.0.clone().into_vec(), pair.1.clone().into_vec())).collect());
        println!("{:?}", solved)
    }

    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
}

#[inline]
fn is_unique_digit(size: usize) -> bool {
    matches!(size, 2 | 4 | 3 | 7)
}

fn solve_first(lines: Vec<(Vec<String>, Vec<String>)>) -> usize {
    let output_iter = lines.iter().map(|line| line.1.clone());
    let output_sizes = output_iter.flat_map(|output| {
        output
            .iter()
            .map(|s: &String| s.len())
            .collect::<Vec<usize>>()
    });

    output_sizes
        .filter(|size| is_unique_digit(*size))
        .count()
}

fn parse_displays10(display: &[String]) -> Displays10 {
    display
        .iter()
        .map(|string| {
            string
                .bytes()
                .map(|ch| (1u8 << (ch & 0b111)))
                .fold(0, |l, r| l | r)
                // .fold(0, |val, ch| val | (1u8 << ((ch as u8) & 0b111)))
        })
        .collect()
}

fn parse_displays4(display: &[String]) -> Displays4 {
    display
        .iter()
        .map(|string| {
            string
                .bytes()
                .map(|ch| (1u8 << (ch & 0b111)))
                .fold(0, |l, r| l | r)
                // .fold(0, |val, ch| val | (1u8 << ((ch as u8) & 0b111)))
        })
        .collect()
}

fn secondary_parse(
    displays: &[ParsedPair],
) -> impl std::iter::Iterator<Item = (Displays10, Displays4)> + '_ {
    displays
        .iter()
        .map(|pair| (parse_displays10(&pair.0), parse_displays4(&pair.1)))
}

fn set_initial_maps(
    display: Display,
    displays: &mut SeenDisplays,
    display_indices: &mut [usize; 2],
    conversions: &mut [u8; 256],
    segments: &mut [u8; 7],
    bit_counts: &[u8; 256],
) {
    assert!(displays.len() >= 9);
    assert!(display_indices.len() == 2);
    match bit_counts[display as usize] {
        2 => {
            displays[1][0] = display;
            conversions[display as usize] = 1;
            segments[2] &= display;
            segments[5] &= display;
        }
        3 => {
            displays[7][0] = display;
            conversions[display as usize] = 7;
            segments[0] &= display;
            // These can be exclusively determined from 2
            // segments[2] &= display;
            // segments[5] &= display;
        }
        4 => {
            displays[4][0] = display;
            conversions[display as usize] = 4;
            segments[1] &= display;
            segments[3] &= display;
            // These can be exclusively determined from 2
            // segments[2] &= display;
            // segments[5] &= display;
        }
        5 => {
            let idx = display_indices[0];
            displays[2][idx] = display;
            displays[3][idx] = display;
            displays[5][idx] = display;
            display_indices[0] += 1;
        }
        6 => {
            let idx = display_indices[1];
            displays[0][idx] = display;
            displays[6][idx] = display;
            displays[9][idx] = display;
            display_indices[1] += 1;
        }
        7 => {
            displays[8][0] = display;
            conversions[display as usize] = 8;
        }
        _ => {
            panic!("invalid length")
        }
    }
}

fn remove_solved06(displays: &mut SeenDisplays) {
    assert!(displays[6].len() == 1);
    let seg6 = displays[6][0];
    assert!(displays[0].len() == 1);
    let seg0 = displays[0][0];

    displays[9].retain(|disp| *disp != seg0 && *disp != seg6);
}

fn remove_solved25(displays: &mut SeenDisplays) {
    assert!(displays[5].len() == 1);
    assert!(displays[2].len() == 1);
    let seg2 = displays[2][0];
    let seg5 = displays[5][0];

    displays[3].retain(|disp| *disp != seg2 && *disp != seg5);
}

fn solve_6(displays: &mut SeenDisplays, conversions: &mut [u8; 256], segments: &mut [u8; 7]) {
    assert!(displays[1].len() == 1);
    let segments_1 = displays[1][0];

    displays[6].retain(|disp| ((*disp) & segments_1) != segments_1);

    assert!(displays[6].len() == 1);
    let segments_6 = displays[6][0];
    conversions[segments_6 as usize] = 6;

    segments[5] = segments_1 & segments_6;
    segments[2] = segments_1 ^ segments[5];
}

fn solve_2(displays: &mut SeenDisplays, conversions: &mut [u8; 256], segments: &mut [u8; 7]) {
    displays[2].retain(|disp| ((*disp) & segments[5]) == 0);
    assert!(displays[2].len() == 1);
    conversions[displays[2][0] as usize] = 2;
}

fn solve_35(displays: &mut SeenDisplays, conversions: &mut [u8; 256], segments: &mut [u8; 7]) {
    displays[5].retain(|disp| ((*disp) & segments[2]) == 0);
    assert!(displays[5].len() == 1);
    conversions[displays[5][0] as usize] = 5;
}

fn solve_d(displays: &SeenDisplays, segments: &mut [u8; 7]) {
    assert!(displays[6].len() == 1);
    segments[3] &= displays[6][0];
}

fn solve_09(displays: &mut SeenDisplays, conversions: &mut [u8; 256], segments: &mut [u8; 7]) {
    assert!(displays[6].len() == 1);
    let segments_6 = displays[6][0];
    assert!(displays[5].len() == 1);
    let segments_5 = displays[5][0];

    displays[0].retain(|disp| *disp != segments_6 && ((*disp ^ segments_5) ^ segments[2]) != 0);

    assert!(displays[0].len() == 1);
    conversions[displays[0][0] as usize] = 0;
    remove_solved06(displays);
    assert!(displays[9].len() == 1);
    conversions[displays[9][0] as usize] = 9;
}

fn decode_display(seen: &[Display]) -> [u8; 256] {
    let get_one_vec = || {
        let mut one: SmallVec<[Display; 3]> = SmallVec::new();
        unsafe { one.set_len(1) }
        one
    };
    let get_three_vec = || {
        let mut three: SmallVec<[Display; 3]> = SmallVec::new();
        unsafe { three.set_len(3) }
        three
    };

    let mut displays: SeenDisplays = [
        get_three_vec(),
        get_one_vec(),
        get_three_vec(),
        get_three_vec(),
        get_one_vec(),
        get_three_vec(),
        get_three_vec(),
        get_one_vec(),
        get_one_vec(),
        get_three_vec()
    ];
    let mut display_indices = [0usize; 2];

    let mut conversions = [0u8; 256];

    let mut segments = [0b1111111u8; 7];

    let bit_counts: [u8; 256];
    unsafe { bit_counts = BIT_COUNTS }

    for display in seen {
        set_initial_maps(*display, &mut displays, &mut display_indices, &mut conversions, &mut segments, &bit_counts)
    }

    solve_6(&mut displays, &mut conversions, &mut segments);
    // known: [a,c,f], [1,4,6,7,8]

    solve_2(&mut displays, &mut conversions, &mut segments);
    // known: [a,c,f], [1,2,4,6,7,8]

    solve_35(&mut displays, &mut conversions, &mut segments);
    // known: [a,c,f], [1,2,3,4,5,6,7,8]

    remove_solved25(&mut displays);
    assert!(displays[3].len() == 1);
    conversions[displays[3][0] as usize] = 3;

    solve_d(&displays, &mut segments);
    // known: [a,c,d,f], [1,2,3,4,5,6,7,8]

    solve_09(&mut displays, &mut conversions, &mut segments);
    // known: [a,c,d,f], [0,1,2,3,4,5,6,7,8,9]

    conversions
}

fn solve_display(seen: &[Display], output: &Displays4) -> usize {
    let conversions = decode_display(seen);

    output
        .iter()
        .map(|disp| conversions[*disp as usize] as usize)
        .fold(0usize, |value, digit| value * 10 + (digit as usize))
}

fn solve_second(lines: &[ParsedPair]) -> usize {
    let displays = secondary_parse(lines);
    let outputs = displays.map(|display_set| solve_display(&display_set.0, &display_set.1));
    //     .fold(0, |l, r| l + r)
    let mut sum = 0usize;
    for out in outputs {
        sum += out
    }
    sum
}

fn parse_seen(line: &str) -> SmallVec<[String; 10]> {
    line.split(' ').map(|s| s.to_string()).collect()
}

fn parse_output(line: &str) -> SmallVec<[String; 4]> {
    line.split(' ').map(|s| s.to_string()).collect()
}

fn parse(lines: Vec<String>) -> Vec<ParsedPair> {
    lines
        .iter()
        .map(|line| line.split(" | ").collect())
        .map(|list: Vec<&str>| (parse_seen(list[0]), parse_output(list[1])))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) -> () {
        let lines = parse(utils::get_input(8, true));

        let mut bit_counts = [0u8; 256];
        for i in 0..256 {
            bit_counts[i] = ((i & 1) as u8) + bit_counts[(i as usize) / 2]
        }
        unsafe { BIT_COUNTS = bit_counts }

        // start profiling
        let guard = pprof::ProfilerGuard::new(100).unwrap();

        // run benchmark
        b.iter(|| solve_second(&lines));

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
