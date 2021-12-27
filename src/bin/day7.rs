mod helpers;
use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 7;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 37);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 168);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<u64> {
    input[0]
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn part_1(input: Vec<u64>) -> u64 {
    // println!("{:?}", input);
    let mut input_clone = input.clone();
    let med = median(&mut input_clone);

    input
        .into_iter()
        .map(|i| if i >= med { i - med } else { med - i })
        .sum()
}

fn median(numbers: &mut [u64]) -> u64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn part_2(input: Vec<u64>) -> u64 {
    let min_pos = input.clone().into_iter().min().unwrap();
    let max_pos = input.clone().into_iter().max().unwrap();

    // we brute force it because, why not?
    let mut max_fuel: u64 = u64::MAX;
    (min_pos..=max_pos).into_iter().for_each(|i| {
        let current_sum: u64 = input
            .clone()
            .into_iter()
            .map(|h| if i >= h { i - h } else { h - i })
            .map(|h| h*(h+1)/2)
            .sum();
        if current_sum < max_fuel {
            max_fuel = current_sum;
        }
    });

    max_fuel
}
