mod helpers;
use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sample = format_input(sample_data(2));
    let input = format_input(input_data(2));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 150);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 900);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<(String, i32)> {
    input
        .iter()
        .map(|s| -> (String, i32) {
            let split = s.split_ascii_whitespace().collect::<Vec<&str>>();
            let command = split[0].to_string();
            let value = split[1].parse::<i32>().unwrap();
            (command, value)
        })
        .collect()
}

fn part_1(input: Vec<(String, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    input.iter().for_each(|(c, i)| {
        if c == "forward" {
            horizontal += i;
        } else if c == "down" {
            vertical += i;
        } else if c == "up" {
            vertical -= i;
        }
    });
    horizontal * vertical
}

fn part_2(input: Vec<(String, i32)>) -> i64 {
    let mut horizontal: i64 = 0;
    let mut vertical: i64 = 0;
    let mut aim: i64 = 0;
    input.iter().for_each(|(c, i)| {
        let i = *i as i64;
        if c == "forward" {
            horizontal += i;
            vertical += aim * i;
        } else if c == "down" {
            aim += i;
        } else if c == "up" {
            aim -= i;
        }
    });
    horizontal * vertical
}
