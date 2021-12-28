mod helpers;
use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 10;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 26397);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 288957);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .into_iter()
        .map(|s| s.chars().into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part_1(input: Vec<Vec<char>>) -> u64 {
    // println!("{:?}", input);
    input
        .into_iter()
        .map(|row| match find_illegal(row) {
            Some(ic) => score_illegal(ic),
            None => 0,
        })
        .sum()
}

fn find_illegal(row: Vec<char>) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();
    let mut answer: Option<char> = None;
    for c in row {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ']' => {
                if stack.pop().unwrap() != '[' {
                    answer = Some(c);
                }
            }
            ')' => {
                if stack.pop().unwrap() != '(' {
                    answer = Some(c);
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    answer = Some(c);
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    answer = Some(c);
                }
            }
            _ => {}
        }
    }
    answer
}

fn score_illegal(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn part_2(input: Vec<Vec<char>>) -> u64 {
    let mut scores = input
        .into_iter()
        .filter(|s| find_illegal(s.clone()).is_none())
        .map(|row| find_missing_chain(row))
        .map(|c| score_chain(c))
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn find_missing_chain(row: Vec<char>) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    for c in row {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => {
                stack.pop();
                ()
            }
            _ => {}
        }
    }
    stack.reverse();
    
    // build replacement
    stack.into_iter().map(|c| {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => ' ',
        }
    }).collect::<Vec<_>>()
}

fn score_chain(c: Vec<char>) -> u64 {
    c.into_iter().fold(0, |acc, c| (5 * acc) + score_char(c))
}

fn score_char(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}
