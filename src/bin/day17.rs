mod helpers;
use crate::helpers::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 17;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 45);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 112);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Target {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Probe {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn format_input(mut input: Vec<String>) -> Target {
    let mut to_parse = input[0].drain(13..).collect::<String>();
    let x_pos = to_parse.chars().position(|c| c == 'x').unwrap();
    let comma_pos = to_parse.chars().position(|c| c == ',').unwrap();
    // calculate position according to removal of the other string
    let y_pos = to_parse.chars().position(|c| c == 'y').unwrap() - comma_pos;
    let str_len = to_parse.len() - comma_pos;

    let x_vals = to_parse
        .drain(x_pos..comma_pos)
        .skip(2)
        .collect::<String>()
        .split("..")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // println!("x_vals: {:?}", x_vals);

    let y_vals = to_parse
        .drain(y_pos..str_len)
        .skip(2)
        .collect::<String>()
        .split("..")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // println!("y_vals: {:?}", y_vals);

    Target {
        min_x: x_vals[0],
        max_x: x_vals[1],
        min_y: y_vals[0],
        max_y: y_vals[1],
    }
}

fn part_1(input: Target) -> i64 {
    // println!("{:?}", input);
    // select potential range of speeds, based on target area. Yeah, brute force it
    let mut initial_speeds: Vec<(i64, i64)> = Vec::new();
    // we will scan any positive x force as negative x doesn't make sense
    let range_x = input.max_x.max(input.min_x) + 10;
    // we scan y from the smallest in the range to three times that (multiply by -3 to remove negative signs)
    let range_y = input.max_y.min(input.min_y);
    for x in 0..range_x {
        for y in range_y..-3 * range_y {
            initial_speeds.push((x, y));
        }
    }

    let mut max_y = i64::MIN;
    for (vx, vy) in initial_speeds {
        let hits = does_it_hit(input, vx, vy);
        match hits {
            Some(y) => {
                max_y = max_y.max(y);
            }
            None => {
                // do nothing
            }
        }
    }

    max_y
}

// returns the max y if the probe hits the area
fn does_it_hit(target: Target, start_vx: i64, start_vy: i64) -> Option<i64> {
    let mut probe = Probe {
        x: 0,
        y: 0,
        vx: start_vx,
        vy: start_vy,
    };

    let mut max_y = 0;

    while !in_target_area(target, &probe) && !missed_target_area(target, &probe) {
        probe = step(&probe);
        if probe.y > max_y {
            max_y = probe.y;
        }
    }

    // println!("{:?} {:?} {:?} {:?}", max_y, probe, in_target_area(target, &probe), missed_target_area(target, &probe));

    if in_target_area(target, &probe) {
        Some(max_y)
    } else {
        None
    }
}

fn missed_target_area(target: Target, probe: &Probe) -> bool {
    (probe.vx > 0 && probe.x >= target.max_x) || (probe.vy < 0 && probe.y <= target.min_y)
}

fn in_target_area(target: Target, probe: &Probe) -> bool {
    probe.x >= target.min_x
        && probe.x <= target.max_x
        && probe.y >= target.min_y
        && probe.y <= target.max_y
}

fn step(probe: &Probe) -> Probe {
    Probe {
        x: probe.x + probe.vx,
        y: probe.y + probe.vy,
        vx: if probe.vx > 0 {
            probe.vx - 1
        } else if probe.vx < 0 {
            probe.vx + 1
        } else {
            0
        },
        vy: probe.vy - 1,
    }
}

fn part_2(input: Target) -> u64 {
    // println!("{:?}", input);
    // select potential range of speeds, based on target area. Yeah, brute force it
    let mut initial_speeds: Vec<(i64, i64)> = Vec::new();
    // we will scan any positive x force as negative x doesn't make sense
    let range_x = input.max_x.max(input.min_x) + 10;
    // we scan y from the smallest in the range to three times that (multiply by -3 to remove negative signs)
    let range_y = input.max_y.min(input.min_y);
    for x in 0..range_x {
        for y in range_y..-3 * range_y {
            initial_speeds.push((x, y));
        }
    }

    let mut count = 0;
    for (vx, vy) in initial_speeds {
        let hits = does_it_hit(input, vx, vy);
        if hits.is_some() {
            count += 1;
        }
    }

    count
}
