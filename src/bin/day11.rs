mod helpers;
use grid::Grid;

use crate::helpers::*;

use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 11;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 1656);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 195);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Grid<u64> {
    let columns = input[0].len();
    let array = input
        .into_iter()
        .flat_map(|s| {
            s.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid::from_vec(array, columns)
}

fn part_1(input: Grid<u64>) -> u64 {
    // println!("{:?}", input);
    (1..=100)
        .into_iter()
        .fold((input, 0), |acc, _i| {
            let (new_grid, flashed) = do_step(acc.0);
            (new_grid, acc.1 + flashed)
        })
        .1
}

// does the step and returns how many octopus flashed
fn do_step(input: Grid<u64>) -> (Grid<u64>, u64) {
    let mut charged: Grid<u64> =
        Grid::from_vec(input.iter().map(|i| i + 1).collect(), input.cols());
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut has_flashed = true;

    while has_flashed {
        has_flashed = false;
        for r in 0..input.rows() {
            for c in 0..input.cols() {
                // if it needs to flash and it hasn't already
                if charged[r][c] > 9 && !flashed.contains(&(r, c)) {
                    has_flashed = true;
                    flashed.insert((r, c));

                    // flash neighbours
                    if r > 0 {
                        charged[r - 1][c] += 1;
                    };
                    if r < charged.rows() - 1 {
                        charged[r + 1][c] += 1;
                    };
                    if c > 0 {
                        charged[r][c - 1] += 1;
                    };
                    if c < charged.cols() - 1 {
                        charged[r][c + 1] += 1;
                    };
                    // diagonals
                    if r > 0 && c > 0 {
                        charged[r - 1][c - 1] += 1;
                    };
                    if r > 0 && c < charged.cols() - 1 {
                        charged[r - 1][c + 1] += 1;
                    };
                    if r < charged.rows() - 1 && c > 0 {
                        charged[r + 1][c - 1] += 1;
                    };
                    if r < charged.rows() - 1 && c < charged.cols() - 1 {
                        charged[r + 1][c + 1] += 1;
                    };
                }
            }
        }
    }

    // set grid to 0 on flashes
    for (r, c) in &flashed {
        charged[*r][*c] = 0;
    }

    (charged, flashed.len() as u64)
}

fn part_2(input: Grid<u64>) -> u64 {
    let mut sync_step = 0;
    let mut step = 1;
    let mut grid = input;

    while sync_step == 0 {
        let (new_grid, _) = do_step(grid);
        if new_grid.clone().into_vec().into_iter().all(|i| i == 0) {
            sync_step = step;
        }
        step += 1;
        grid = new_grid;
    }

    sync_step
}
