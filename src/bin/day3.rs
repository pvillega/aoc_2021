mod helpers;
use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sample = format_input(sample_data(3));
    let input = format_input(input_data(3));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 198);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 230);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<String> {
    input
}

fn part_1(input: Vec<String>) -> u32 {
    let mut gamma = vec![0; input[0].len()];
    let mut epsilon = vec![0; input[0].len()];
    input.iter().for_each(|s| {
        s.chars().enumerate().for_each(|(i, c)| {
            let v = c.to_digit(2).unwrap();
            gamma[i] += v;
            epsilon[i] += v;
        });
    });
    let gamma_rate = process_array(gamma, input.len(), |i, l| if i > l { "1" } else { "0" });
    let epsilon_rate = process_array(epsilon, input.len(), |i, l| if i < l { "1" } else { "0" });
    // println!("{:?}", gamma_rate);
    // println!("{:?}", epsilon_rate);
    gamma_rate * epsilon_rate
}

// we have added all teh 1 in each position of the array, now we see if they are the majority (sum > length/2)
// to decide if we make it a 0 or 1
fn process_array(arr: Vec<u32>, len: usize, cond: fn(u32, u32) -> &'static str) -> u32 {
    // decide if array position is 0 or 1
    let binary = arr
        .into_iter()
        .map(|i| cond(i, (len as u32) / 2))
        .collect::<Vec<_>>()
        .join("");
    to_int(binary)
}

fn part_2(input: Vec<String>) -> i32 {
    fn filter_input_oxygen(input: Vec<String>, pos: u32) -> String {
        let most_common = find_most_common_in_position(&input, pos);
        let filtered: Vec<String> = input
            .into_iter()
            .filter(|s| s.chars().nth(pos as usize).unwrap() == most_common)
            .collect();
        if filtered.len() == 1 {
            filtered[0].clone()
        } else {
            filter_input_oxygen(filtered, pos + 1)
        }
    }

    fn filter_input_co2(input: Vec<String>, pos: u32) -> String {
        let less_common = find_less_common_in_position(&input, pos);
        let filtered: Vec<String> = input
            .into_iter()
            .filter(|s| s.chars().nth(pos as usize).unwrap() == less_common)
            .collect();
        if filtered.len() == 1 {
            filtered[0].clone()
        } else {
            filter_input_co2(filtered, pos + 1)
        }
    }

    let oxygen_binary = filter_input_oxygen(input.clone(), 0);
    let oxygen = to_int(oxygen_binary);

    let co2_binary = filter_input_co2(input, 0);
    let co2 = to_int(co2_binary);

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    (oxygen * co2) as i32
}

fn to_int(s: String) -> u32 {
    isize::from_str_radix(&s, 2).unwrap().try_into().unwrap()
}

fn find_most_common_in_position(input: &Vec<String>, pos: u32) -> char {
    let mut counts = [0; 2];
    input.iter().for_each(|s| {
        let v = s
            .chars()
            .nth(pos as usize)
            .map(|c| c.to_digit(2).unwrap())
            .unwrap();
        counts[v as usize] += 1;
    });
    if counts[0] > counts[1] {
        '0'
    } else {
        '1'
    }
}

fn find_less_common_in_position(input: &Vec<String>, pos: u32) -> char {
    let mut counts = [0; 2];
    input.iter().for_each(|s| {
        let v = s
            .chars()
            .nth(pos as usize)
            .map(|c| c.to_digit(2).unwrap())
            .unwrap();
        counts[v as usize] += 1;
    });
    if counts[0] <= counts[1] {
        '0'
    } else {
        '1'
    }
}
