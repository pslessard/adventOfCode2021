extern crate pprof;
extern crate test;

static mut BIT_COUNTS: [u8; 256] = [0u8; 256];

pub fn main() {
    let mut bit_counts = [0u8; 256];
    for i in 0..256 {
        assert!(bit_counts.len() >= 255);
        bit_counts[i] = ((i & 1) as u8) + bit_counts[(i as usize) / 2]
    }
    unsafe { BIT_COUNTS = bit_counts }

    let lines = parse(utils::get_input(8, true));
    {
        let solved = solve_first(lines.clone());
        println!("{:?}", solved)
    }

    {
        let solved = solve_second(&lines);
        println!("{:?}", solved)
    }
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

#[inline]
fn is_unique_digit(size: usize) -> bool {
    matches!(size, 2 | 4 | 3 | 7)
}

type Display = u8;
type Displays = Vec<Display>;

fn parse_displays(display: &[String]) -> Displays {
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
    displays: &[(Vec<String>, Vec<String>)],
) -> impl std::iter::Iterator<Item = (Vec<u8>, Vec<u8>)> + '_ {
    displays
        .iter()
        .map(|pair| (parse_displays(&pair.0), parse_displays(&pair.1)))
}

type SeenDisplays = [Vec<Display>; 10];

fn set_initial_maps(
    display: Display,
    displays: &mut SeenDisplays,
    display_indices: &mut [usize; 10],
    segments: &mut [u8; 7],
    bit_counts: &[u8; 256],
) {
    assert!(displays.len() >= 9);
    assert!(display_indices.len() >= 9);
    match bit_counts[display as usize] {
        2 => {
            displays[1][0] = display;
            segments[2] &= display;
            segments[5] &= display;
        }
        3 => {
            displays[7][0] = display;
            segments[0] &= display;
            segments[2] &= display;
            segments[5] &= display;
        }
        4 => {
            displays[4][0] = display;
            segments[1] &= display;
            segments[2] &= display;
            segments[3] &= display;
            segments[5] &= display;
        }
        5 => {
            displays[2][display_indices[2]] = display;
            displays[3][display_indices[3]] = display;
            displays[5][display_indices[5]] = display;
            display_indices[2] += 1;
            display_indices[3] += 1;
            display_indices[5] += 1;
        }
        6 => {
            displays[0][display_indices[0]] = display;
            displays[6][display_indices[6]] = display;
            displays[9][display_indices[9]] = display;
            display_indices[0] += 1;
            display_indices[6] += 1;
            display_indices[9] += 1;
        }
        7 => {
            displays[8][0] = display;
        }
        _ => {
            panic!("invalid length")
        }
    }
}

fn remove_solved1(displays: &mut SeenDisplays, segments: u8, num: usize) {
    for (i, disp) in displays.iter_mut().enumerate() {
        if i != num {
            disp.retain(|disp| *disp != segments);
        }
    }
}

fn remove_solved3(displays: &mut SeenDisplays, nums0: usize, nums1: usize, nums2: usize) {
    assert!(displays[nums0].len() == 1);
    assert!(displays[nums1].len() == 1);
    assert!(displays[nums2].len() == 1);
    let segments = (displays[nums0][0], displays[nums1][0], displays[nums2][0]);

    for (i, disp) in displays.iter_mut().enumerate() {
        match i {
            d if d == nums0 => {
                disp.retain(|disp| *disp != segments.1 && *disp != segments.2);
            }
            d if d == nums1 => {
                disp.retain(|disp| *disp != segments.0 && *disp != segments.2);
            }
            d if d == nums2 => {
                disp.retain(|disp| *disp != segments.0 && *disp != segments.1);
            }
            _ => {
                disp.retain(|disp| *disp != segments.0 && *disp != segments.1 && *disp != segments.2);
            }
        }
    }
}

fn solve_6(displays: &mut SeenDisplays, segments: &mut [u8; 7]) {
    assert!(displays[1].len() == 1);
    let segments_1 = displays[1][0];

    displays[6].retain(|disp| ((*disp) & segments_1) != segments_1);

    assert!(displays[6].len() == 1);
    let segments_6 = displays[6][0];
    segments[5] = segments_1 & segments_6;
    segments[2] = segments_1 ^ segments[5];
}

fn solve_2(displays: &mut SeenDisplays, segments: &mut [u8; 7]) {
    displays[2].retain(|disp| ((*disp) & segments[5]) == 0);
}

fn solve_35(displays: &mut SeenDisplays, segments: &mut [u8; 7]) {
    displays[5].retain(|disp| ((*disp) & segments[2]) == 0);
}

fn solve_d(displays: &SeenDisplays, segments: &mut [u8; 7]) {
    assert!(displays[6].len() == 1);
    segments[3] &= displays[6][0];
}

fn solve_09(displays: &mut SeenDisplays, segments: &mut [u8; 7]) {
    assert!(displays[5].len() == 1);
    let segments_5 = displays[5][0];
    displays[0].retain(|disp| ((((*disp) ^ segments_5) ^ segments[2]) != 0));

    assert!(displays[0].len() == 1);
    remove_solved1(displays, displays[0][0], 0);
}

fn decode_display(seen: &[Display]) -> [u8; 256] {
    let one = vec![0u8];
    let three = vec![0u8; 3];
    let mut displays: SeenDisplays = [
        three.clone(),
        one.clone(),
        three.clone(),
        three.clone(),
        one.clone(),
        three.clone(),
        three.clone(),
        one.clone(),
        one,
        three,
    ];
    let mut display_indices = [0usize; 10];

    let mut segments = [0b1111111u8; 7];

    let bit_counts: [u8; 256];
    unsafe { bit_counts = BIT_COUNTS }

    for display in seen {
        set_initial_maps(*display, &mut displays, &mut display_indices, &mut segments, &bit_counts)
    }

    solve_6(&mut displays, &mut segments);
    // known: [a,c,f], [1,4,6,7,8]

    solve_2(&mut displays, &mut segments);
    // known: [a,c,f], [1,2,4,6,7,8]

    solve_35(&mut displays, &mut segments);
    // known: [a,c,f], [1,2,3,4,5,6,7,8]

    remove_solved3(&mut displays, 2, 5, 6);

    solve_d(&displays, &mut segments);
    // known: [a,c,d,f], [1,2,3,4,5,6,7,8]

    solve_09(&mut displays, &mut segments);
    // known: [a,c,d,f], [0,1,2,3,4,5,6,7,8,9]

    let mut conversions = [0u8; 256];
    for i in 0..displays.len() {
        conversions[displays[i][0] as usize] = i as u8;
    }
    conversions
}

fn solve_display(seen: &[Display], output: &Displays) -> usize {
    let conversions = decode_display(seen);

    output
        .iter()
        .map(|disp| conversions[*disp as usize] as usize)
        .fold(0usize, |value, digit| value * 10 + (digit as usize))
}

fn solve_second(lines: &[(Vec<String>, Vec<String>)]) -> usize {
    let displays = secondary_parse(lines);
    let outputs = displays.map(|display_set| solve_display(&display_set.0, &display_set.1));
    //     .fold(0, |l, r| l + r)
    let mut sum = 0usize;
    for out in outputs {
        sum += out
    }
    sum
}

fn parse_partial_line(line: &str) -> Vec<String> {
    line.split(' ').map(|s| s.to_string()).collect()
}

fn parse(lines: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    lines
        .iter()
        .map(|line| line.split(" | ").collect())
        .map(|list: Vec<&str>| (parse_partial_line(list[0]), parse_partial_line(list[1])))
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
    }
}
