mod helpers;
use crate::helpers::*;
use grid::*;
use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 9;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 15);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 1134);
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
    let mut risk: u64 = 0;

    for r in 0..input.rows() {
        for c in 0..input.cols() {
            let current = input.get(r, c).unwrap();
            // verify the surrounding elements. If we go out of bounds we default to true for ease in logic
            // NOTE: the implementation of the grid library only checks upper bounds, so we need to verify we don't as for a negative position.
            let lower_than_top = if r > 0 {
                input.get(r - 1, c).map(|x| x > current).unwrap_or(true)
            } else {
                true
            };
            let lower_than_bottom = input.get(r + 1, c).map(|x| x > current).unwrap_or(true);

            let lower_than_left = if c > 0 {
                input.get(r, c - 1).map(|x| x > current).unwrap_or(true)
            } else {
                true
            };
            let lower_than_right = input.get(r, c + 1).map(|x| x > current).unwrap_or(true);

            if lower_than_top && lower_than_bottom && lower_than_left && lower_than_right {
                risk += current + 1;
            }
        }
    }

    risk
}

fn part_2(input: Grid<u64>) -> u64 {
    let mut basins: Vec<u64> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // find basins
    for r in 0..input.rows() {
        for c in 0..input.cols() {
            // skip visited nodes as that means we already inspected the basin
            if visited.contains(&(r, c)) {
                continue;
            }
            let basin_size = find_basin_size(&input, r, c, &mut visited);
            basins.push(basin_size);
        }
    }

    // find top 3 and multiply them
    basins.sort_by(|a, b| a.cmp(b).reverse());
    basins.iter().take(3).product()
}

fn find_basin_size(
    input: &Grid<u64>,
    r: usize,
    c: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u64 {
    let current_value = input.get(r, c).unwrap();

    // hit a border or already visited
    if *current_value == 9 || visited.contains(&(r, c)) {
        0
    } else {
        visited.insert((r, c));
        let size_top = if r > 0 {
            find_basin_size(input, r - 1, c, visited)
        } else {
            0
        };
        let size_bottom = if r < input.rows() - 1 {
            find_basin_size(input, r + 1, c, visited)
        } else {
            0
        };
        let size_left = if c > 0 {
            find_basin_size(input, r, c - 1, visited)
        } else {
            0
        };
        let size_right = if c < input.cols() - 1 {
            find_basin_size(input, r, c + 1, visited)
        } else {
            0
        };

        1 + size_top + size_bottom + size_left + size_right
    }
}
