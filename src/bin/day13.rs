mod helpers;

use grid::Grid;

use crate::helpers::*;

use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 13;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 17);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, sample_grid());
    let result = part_2(input);
    println!("part 2: {:?}", result);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    position: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    grid: HashSet<(i32, i32)>,
    instructions: Vec<Instruction>,
}

fn format_input(input: Vec<String>) -> Entry {
    let mut grid = HashSet::new();
    let mut instructions = Vec::new();

    for mut line in input {
        if line.starts_with("fold along") {
            let equal_offset = line.find('=').unwrap_or(line.len());
            line.drain(..(equal_offset - 1));
            let arr = line.split("=").collect::<Vec<_>>();
            let direction = if arr[0] == "x" {
                Direction::X
            } else {
                Direction::Y
            };
            let position = arr[1].parse::<i32>().unwrap();
            let instruction = Instruction {
                direction,
                position,
            };
            instructions.push(instruction);
        } else if !line.is_empty() {
            let arr = line.split(",").collect::<Vec<_>>();
            grid.insert((
                arr[0].parse::<i32>().unwrap(),
                arr[1].parse::<i32>().unwrap(),
            ));
        }
    }

    Entry { grid, instructions }
}

fn part_1(input: Entry) -> u64 {
    // println!("{:?}", input);
    let result = fold(&input.instructions[0], input.grid);
    result.len() as u64
}

fn fold(instruction: &Instruction, grid: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    // find any pairs where x or y > position as they will be removed from the grid
    let (stay, removed): (HashSet<(i32, i32)>, HashSet<(i32, i32)>) = match instruction.direction {
        Direction::X => grid
            .into_iter()
            .partition(|&(x, _)| x < instruction.position),
        Direction::Y => {
            // find any pairs where y > position as they will be removed
            grid.into_iter()
                .partition(|&(_, y)| y < instruction.position)
        }
    };

    let mut new_grid = stay;

    // fold any points from teh removed section
    for &(x, y) in removed.iter() {
        let new_x = match instruction.direction {
            Direction::X => {
                let distance_from_fold = (x - instruction.position).abs();
                instruction.position - distance_from_fold
            }
            Direction::Y => x,
        };
        let new_y = match instruction.direction {
            Direction::X => y,
            Direction::Y => {
                let distance_from_fold = (y - instruction.position).abs();
                instruction.position - distance_from_fold
            }
        };
        new_grid.insert((new_x, new_y));
    }

    new_grid
}

fn part_2(input: Entry) -> Grid<String> {
    let result = input
        .instructions
        .into_iter()
        .fold(input.grid, |grid, ins| fold(&ins, grid));
    set_to_letters(result)
}

fn sample_grid() -> Grid<String> {
    let mut grid = Grid::init(5, 5, ".".to_string());
    let visible = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 0),
        (1, 4),
        (2, 0),
        (2, 4),
        (3, 0),
        (3, 4),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
    ];
    for &(x, y) in visible.iter() {
        grid[y][x] = "#".to_string();
    }
    grid
}

fn set_to_letters(set: HashSet<(i32, i32)>) -> Grid<String> {
    // println!("{:?}", set);
    let rows: usize = (set.iter().map(|&(_, y)| y).max().unwrap() + 1) as usize;
    let cols: usize = (set.iter().map(|&(x, _)| x).max().unwrap() + 1) as usize;

    let mut grid = Grid::init(rows, cols, ".".to_string());
    for &(x, y) in set.iter() {
        grid[y as usize][x as usize] = "#".to_string();
    }

    // print to terminal to see code
    println!();
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            let current = grid.get(r, c).unwrap();
            print!("{}", current);
        }
        println!();
    }
    println!();

    grid
}
