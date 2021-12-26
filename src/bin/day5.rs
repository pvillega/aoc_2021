mod helpers;
use crate::helpers::*;

use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let sample = format_input(sample_data(5));
    let input = format_input(input_data(5));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 5);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 12);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

fn format_input(input: Vec<String>) -> Vec<Segment> {
    input
        .iter()
        .map(|line| {
            // println!("{:?}", line);
            let points = line.split(" -> ").collect::<Vec<&str>>();
            // println!("{:?}", points);
            let start = points[0]
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let end = points[1]
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Segment {
                start: Point {
                    x: start[0],
                    y: start[1],
                },
                end: Point {
                    x: end[0],
                    y: end[1],
                },
            }
        })
        .collect::<Vec<_>>()
}

fn part_1(input: Vec<Segment>) -> i32 {
    // println!("{:?}", input);
    let mut point_map: HashMap<Point, i32> = HashMap::new();

    input.iter().flat_map(|s| get_points(*s, false)).for_each(|p| {
        let new_count = point_map.get(&p).unwrap_or(&0) + 1;
        point_map.insert(p, new_count);
    });

    point_map.values().filter(|&v| *v >= 2).count() as i32
}

fn get_points(s: Segment, consider_diagonals: bool) -> Vec<Point> {
    let mut points = Vec::new();
    let x = s.start.x;
    let y = s.start.y;

    if s.start.x == s.end.x {
        let (a, b) = if s.start.y < s.end.y {
            (s.start.y, s.end.y)
        } else {
            (s.end.y, s.start.y)
        };
        (a..=b)
        .for_each(|y| {
            points.push(Point { x, y });
        });
    } else if s.start.y == s.end.y {
        let (a, b) = if s.start.x < s.end.x {
            (s.start.x, s.end.x)
        } else {
            (s.end.x, s.start.x)
        };
        (a..=b).for_each(|x| {
            points.push(Point { x, y });
        });
    } else if consider_diagonals {
        // we have 4 potential diagonal combinations to consider
        if s.start.x < s.end.x {
            if s.start.y < s.end.y {
                (0..=(s.end.y-s.start.y)).for_each(|i| {
                    points.push(Point { x: s.start.x + i, y: s.start.y + i });
                });
            } else {
                (0..=(s.start.y-s.end.y)).for_each(|i| {
                    points.push(Point { x: s.start.x + i, y: s.start.y - i });
                });
            }
        } else {
            if s.start.y < s.end.y {
                (0..=(s.end.y-s.start.y)).for_each(|i| {
                    points.push(Point { x: s.start.x - i, y: s.start.y + i });
                });
            } else {
                (0..=(s.start.y-s.end.y)).for_each(|i| {
                    points.push(Point { x: s.start.x - i, y: s.start.y - i });
                });
            }
        };
    }

    points
}

fn part_2(input: Vec<Segment>) -> i32 {
    let mut point_map: HashMap<Point, i32> = HashMap::new();

    input.iter().flat_map(|s| get_points(*s, true)).for_each(|p| {
        let new_count = point_map.get(&p).unwrap_or(&0) + 1;
        point_map.insert(p, new_count);
    });

    point_map.values().filter(|&v| *v >= 2).count() as i32
}
