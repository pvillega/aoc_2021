mod helpers;
use crate::helpers::*;

use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 8;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 26);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 61229);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug, Clone)]
struct Entry {
    input: Vec<String>,
    output: Vec<String>,
}

fn format_input(input: Vec<String>) -> Vec<Entry> {
    input
        .into_iter()
        .map(|s| {
            let arr = s.split('|').collect::<Vec<&str>>();
            let inp = arr[0]
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let out = arr[1]
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            Entry {
                input: inp,
                output: out,
            }
        })
        .collect::<Vec<_>>()
}

fn part_1(input: Vec<Entry>) -> u64 {
    // println!("{:?}", input);
    input
        .into_iter()
        .map(|entry| {
            entry
                .output
                .into_iter()
                .filter(|s| is_one(s) || is_four(s) || is_seven(s) || is_eight(s))
                .count() as u64
        })
        .sum()
}

fn part_2(input: Vec<Entry>) -> u64 {
    input
        .into_iter()
        .map(|entry| {
            // calculate digits for this row
            let coded_numbers = decode_numbers(entry.input);
            // convert output to number
            let number_as_string = entry
                .output
                .into_iter()
                .map(|s| {
                    let element_sorted = sort_string(s);

                    (&coded_numbers)
                        .into_iter()
                        .position(|s| *s == element_sorted)
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join("");

            number_as_string.parse::<u64>().unwrap()
        })
        .sum()
}

fn decode_numbers(input: Vec<String>) -> Vec<String> {
    let mut coded_numbers = vec!["".to_string(); 10];
    let sorted_chars_input = input
        .into_iter()
        .map(|s| sort_string(s))
        .collect::<Vec<_>>();
    // find 1,4,7,8 - easy numbers
    for s in sorted_chars_input.iter() {
        if is_one(s) {
            coded_numbers[1] = s.clone();
        } else if is_four(s) {
            coded_numbers[4] = s.clone();
        } else if is_seven(s) {
            coded_numbers[7] = s.clone();
        } else if is_eight(s) {
            coded_numbers[8] = s.clone();
        }
    }
    let one_set = coded_numbers[1].chars().collect::<HashSet<_>>();
    let four_set = coded_numbers[4].chars().collect::<HashSet<_>>();
    let seven_set = coded_numbers[7].chars().collect::<HashSet<_>>();

    // second iteration for the harder numbers
    for s in sorted_chars_input.iter() {
        // find 2,3,5 as they have length 5
        let input_set = s.chars().collect::<HashSet<_>>();
        if s.len() == 5 {
            if input_set.intersection(&four_set).count() == 2 {
                coded_numbers[2] = s.clone();
            } else if input_set.intersection(&seven_set).count() == 3 {
                coded_numbers[3] = s.clone();
            } else {
                coded_numbers[5] = s.clone();
            }
        }
        // find 0, 6, 9 as they have length 6
        if s.len() == 6 {
            let input_set = s.chars().collect::<HashSet<_>>();
            if input_set.intersection(&one_set).count() == 1 {
                coded_numbers[6] = s.clone();
            } else if input_set.intersection(&four_set).count() == 4 {
                coded_numbers[9] = s.clone();
            } else {
                coded_numbers[0] = s.clone();
            }
        }
    }

    coded_numbers
}

fn sort_string(s: String) -> String {
    let mut sorted = s.chars().collect::<Vec<char>>();
    sorted.sort();
    String::from_iter(&sorted)
}

fn is_one(s: &str) -> bool {
    s.len() == 2
}

fn is_four(s: &str) -> bool {
    s.len() == 4
}

fn is_seven(s: &str) -> bool {
    s.len() == 3
}

fn is_eight(s: &str) -> bool {
    s.len() == 7
}
