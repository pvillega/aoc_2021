mod helpers;
use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sample = format_input(sample_data(1));
    let input = format_input(input_data(1));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 7);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 5);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<i32> {
    input.iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn part_1(input: Vec<i32>) -> usize {
    let windows = input.windows(2);
    windows.filter(|w| w[0] < w[1]).count()
}

fn part_2(input: Vec<i32>) -> usize {
    let windows = input.windows(3).into_iter().map(|w| w.iter().sum()).collect::<Vec<i32>>();
    windows.windows(2).filter(|w| w[0] < w[1]).count()
}
