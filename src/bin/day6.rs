mod helpers;
use crate::helpers::*;

use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 6;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 5934);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 26984457539);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<u32> {
    input[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn part_1(input: Vec<u32>) -> u64 {
    // println!("{:?}", input);
    let days = 80;
    let mut mem_children: HashMap<u32, u64> = HashMap::new();
    let original_fish: u64 = input.len() as u64;
    let children: u64 = input
        .into_iter()
        .map(|i| simulate(i, 0, days, &mut mem_children))
        .sum();
    original_fish + children
}

fn simulate(initial_state: u32, start_date: u32, up_to_days: u32, mem_children: &mut HashMap<u32, u64>) -> u64 {
    // println!("{:?} {:?} {:?} {:?}", initial_state, start_date, up_to_days, mem_children);
    // build array with all the spawn days we will have children on
    let mut spawn_days: Vec<u32> = Vec::new();
    if start_date + initial_state < up_to_days {
        // if we can have children, add to vector the days in which we do so
        let mut left_days = start_date + initial_state + 1;
        spawn_days.push(left_days);
        while (left_days + 7) <= up_to_days {
            left_days += 7;
            spawn_days.push(left_days);
        }
    }
    let direct_children = spawn_days.len() as u64;

    // find second order children by calculating the number of children from our children on each particular day
    let mut second_order_children: u64 = 0;
    spawn_days.into_iter().for_each(|day| {
        if mem_children.contains_key(&day) {
            second_order_children += mem_children[&day];
        } else {
            // data missing, calculate how many fish spawn from a fish born on this day
            let count = simulate(8, day, up_to_days, mem_children);
            mem_children.insert(day, count);
            second_order_children += count;
        }
    });

    // println!(">{:?} {:?}", direct_children, second_order_children);
    direct_children + second_order_children
}

fn part_2(input: Vec<u32>) -> u64 {
    let days = 256;
    let mut mem_children: HashMap<u32, u64> = HashMap::new();
    let original_fish: u64 = input.len() as u64;
    let children: u64 = input
        .into_iter()
        .map(|i| simulate(i, 0, days, &mut mem_children))
        .sum();
    original_fish + children
}
