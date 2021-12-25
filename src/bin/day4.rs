mod helpers;
use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sample = format_input(sample_data(4));
    let input = format_input(input_data(4));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 4512);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 1924);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug)]
struct Board {
    lines: Vec<Vec<i32>>,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            lines: self.lines.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Data {
    lines: Vec<i32>,
    boards: Vec<Board>,
}

fn format_input(mut input: Vec<String>) -> Data {
    let lines = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let boards = input
        .drain(2..)
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|slice| {
            // println!("{:?}", slice);
            slice
                .iter()
                .take(5)
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.trim().parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|lines| Board { lines })
        .collect::<Vec<_>>();

    Data { lines, boards }
}

fn part_1(input: Data) -> i32 {
    check_boards(input.boards, input.lines)
}

const MARK: i32 = -1;

fn check_boards(boards: Vec<Board>, numbers: Vec<i32>) -> i32 {
    let mut found: Option<Board> = None;
    if let Some((current, remaining_numbers)) = numbers.split_first() {
        let updated_boards: Vec<Board> = boards
            .iter()
            .map(|b| {
                let lines: Vec<Vec<i32>> = b
                    .lines
                    .iter()
                    .map(|l| {
                        l.iter()
                            .map(|i| if i == current { MARK } else { *i })
                            .collect()
                    })
                    .collect();
                let new_board = Board {
                    lines: lines.clone(),
                };
                if check_winner(lines) {
                    found = Some(new_board.clone())
                };
                new_board
            })
            .collect();

        match found {
            None => check_boards(updated_boards, remaining_numbers.to_vec()),
            Some(b) => {
                b.lines
                    .iter()
                    .map(|l| l.iter().filter(|i| **i != MARK).sum::<i32>())
                    .sum::<i32>()
                    * current
            }
        }
    } else {
        -1
    }
}

fn check_winner(lines: Vec<Vec<i32>>) -> bool {
    let has_row = lines.iter().any(|l| l.iter().sum::<i32>() == MARK * 5);
    let has_column = matrix_transpose(&lines)
        .iter()
        .any(|l| l.iter().sum::<i32>() == MARK * 5);
    has_row || has_column
}

fn matrix_transpose(lines: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed = Vec::new();
    for i in 0..lines[0].len() {
        let mut row = Vec::new();
        for j in 0..lines.len() {
            row.push(lines[j][i]);
        }
        transposed.push(row);
    }
    transposed
}

fn part_2(input: Data) -> i32 {
    check_loser_board(input.boards, input.lines)
}

fn check_loser_board(boards: Vec<Board>, numbers: Vec<i32>) -> i32 {
    let mut found: Option<Board> = None;
    if let Some((current, remaining_numbers)) = numbers.split_first() {
        let updated_boards: Vec<Board> = boards
            .iter()
            .map(|b| {
                let lines: Vec<Vec<i32>> = b
                    .lines
                    .iter()
                    .map(|l| {
                        l.iter()
                            .map(|i| if i == current { MARK } else { *i })
                            .collect()
                    })
                    .collect();
                let new_board = Board {
                    lines: lines.clone(),
                };
                // only store the winner if we are on the last board
                if check_winner(lines) && boards.len() == 1 {
                    found = Some(new_board.clone())
                };
                new_board
            })
            // remove winners for next iterations
            .filter(|b| !check_winner(b.lines.clone()))
            .collect();

        match found {
            None => check_loser_board(updated_boards, remaining_numbers.to_vec()),
            Some(b) => {
                b.lines
                    .iter()
                    .map(|l| l.iter().filter(|i| **i != MARK).sum::<i32>())
                    .sum::<i32>()
                    * current
            }
        }
    } else {
        -1
    }
}
