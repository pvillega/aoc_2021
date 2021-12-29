mod helpers;

use crate::helpers::*;

use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 14;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 1588);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 2188189693529);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    template: Vec<String>,
    pair_insertions: HashMap<String, String>,
}

fn format_input(mut input: Vec<String>) -> Entry {
    let template = input[0].chars().map(|c| c.to_string()).collect::<Vec<_>>();

    let pair_insertions = input
        .drain(2..)
        .map(|line| {
            let mut parts = line.split(" -> ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            (key, value)
        })
        .collect::<HashMap<_, _>>();

    Entry {
        template,
        pair_insertions,
    }
}

fn part_1(input: Entry) -> u64 {
    // println!("{:?}", input);
    let steps = 10;
    let result = (1..=steps)
        .into_iter()
        .fold(input.template, |acc, _| step(acc, &input.pair_insertions));
    calculate_value(result)
}

fn step(template: Vec<String>, insertions: &HashMap<String, String>) -> Vec<String> {
    let mut result = template.clone();
    let mut to_insert: Vec<(usize, String)> = Vec::new();

    template.windows(2).enumerate().for_each(|(i, pair)| {
        let key = pair.join("");
        match insertions.get(&key) {
            Some(value) => to_insert.push((i + 1, value.to_string())),
            None => {}
        }
    });

    to_insert.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    to_insert.into_iter().for_each(|(i, s)| {
        result.insert(i, s);
    });
    // println!("{:?} {:?}", result.len(),  result);
    result
}

fn calculate_value(elements: Vec<String>) -> u64 {
    let mut map: HashMap<String, u64> = HashMap::new();

    elements.into_iter().for_each(|e| {
        *map.entry(e).or_insert(0) += 1;
    });

    let max = map.values().max().unwrap();
    let min = map.values().min().unwrap();

    max - min
}

fn part_2(input: Entry) -> u128 {
    // needs a new implementation as the array of elements wouldn't fit in memory
    let steps = 40;

    let mut template_as_map: HashMap<String, u128> = HashMap::new();
    input
        .template
        .windows(2)
        .for_each(|pair| {
            let key = pair.join("");
            *template_as_map.entry(key).or_insert(0) += 1;
        });

    let insertions_as_map: HashMap<String, Vec<String>> = input
        .pair_insertions
        .iter()
        .map(|(k, v)| {
            let value1 = k.chars().nth(0).unwrap().to_string() + v;
            let value2 = v.to_string() + &k.chars().nth(1).unwrap().to_string();
            (k.to_owned(), vec![value1, value2])
        })
        .collect();
    // println!("{:?}",  insertions_as_map);

    let result = (1..=steps)
        .into_iter()
        .fold(template_as_map, |acc, _| step_map(acc, &insertions_as_map));
    calculate_value_from_map(result)
}

fn step_map(
    template: HashMap<String, u128>,
    insertions: &HashMap<String, Vec<String>>,
) -> HashMap<String, u128> {
    let mut result: HashMap<String, u128> = HashMap::new();

    template
        .into_iter()
        .for_each(|(key, count)| match insertions.get(&key) {
            Some(values) => {
                values.iter().for_each(|new_pair| {
                    *result.entry(new_pair.to_string()).or_insert(0) += count;
                });
            }
            None => {}
        });

    // println!("{:?}",  result);
    result
}

fn calculate_value_from_map(elements: HashMap<String, u128>) -> u128 {
    let mut map: HashMap<String, u128> = HashMap::new();

    // every pair is split into its components
    elements.into_iter().for_each(|e| {
        let mut iterator = e.0.chars().into_iter();
        let k1 = iterator.next().unwrap().to_string();
        let k2 = iterator.next().unwrap().to_string();

        *map.entry(k1).or_insert(0) += e.1;
        *map.entry(k2).or_insert(0) += e.1;
    });
    // println!("{:?}",  map);

    // let max = (*map.values().max().unwrap() as f64/2_f64).ceil() as u64;
    let max = map.values().max().unwrap();
    let min = map.values().min().unwrap();

    // every letter is counted twice due to a letter belonging to 2 pairs, so divide by 2! Take in account int division
    let diff = max - min;
    if diff % 2 == 0 {
        diff / 2
    } else {
        (diff + 1) / 2
    }
}
// 3700829748840
