mod helpers;

use crate::helpers::*;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 12;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 226);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 3509);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    input.into_iter().for_each(|s| {
        let arr = s.split("-").collect::<Vec<&str>>();
        let key = arr[0].to_string();
        let value = arr[1].to_string();

        // we add both directions to the graph
        map.entry(key.clone())
            .or_insert(HashSet::new())
            .insert(value.clone());
        map.entry(value).or_insert(HashSet::new()).insert(key);
    });

    map
}

const START: &str = "start";
const END: &str = "end";

fn part_1(input: HashMap<String, HashSet<String>>) -> u64 {
    // println!("{:?}", input);
    let can_visit: fn(&str, &HashMap<String, u32>) -> bool =
        |next, visited| !(next.to_lowercase() == *next && visited.contains_key(next));
    let paths = find_paths(&input, START, &Vec::new(), &HashMap::new(), can_visit);

    paths.len() as u64
}

fn find_paths(
    input: &HashMap<String, HashSet<String>>,
    current: &str,
    paths: &Vec<String>,
    visited: &HashMap<String, u32>,
    can_visit: fn(&str, &HashMap<String, u32>) -> bool,
) -> Vec<String> {
    // we make a copy of the visited nodes as otherwise a shared structure would make us miss paths
    let mut visited_copy = visited.clone();
    *visited_copy.entry(current.to_string()).or_insert(0) += 1;

    // keep track of path for debugging purposes
    let new_paths = if current == START {
        vec![(current.to_owned() + ",")]
    } else {
        paths
            .into_iter()
            .map(|p| p.to_owned() + current + ",")
            .collect::<Vec<String>>()
    };

    // terminate on end
    if current == END {
        return new_paths;
    }

    // otherwise build new paths
    let mut child_paths: Vec<String> = Vec::new();
    let next_paths = input.get(current).unwrap();
    next_paths.iter().for_each(|next| {
        if can_visit(next, &visited_copy) {
            child_paths = find_paths(input, next, &new_paths, &visited_copy, can_visit);
            // println!(">>{:?}", child_paths);
        }
    });

    child_paths
}

fn part_2(input: HashMap<String, HashSet<String>>) -> u64 {
    let can_visit: fn(&str, &HashMap<String, u32>) -> bool = |next, visited| {
        let not_start = next != START;
        let not_lowercase = next.to_lowercase() != next;

        let single_lowercase_two_visits = visited
            .into_iter()
            .filter(|&pair| &pair.0.to_lowercase() == pair.0 && pair.1 == &2)
            .count()
            <= 1;
        // we need this to ensure we are not looping between upper and lowercase and having a single lowercase with 400 visits
        let no_lowercase_more_than_two = visited
            .into_iter()
            .filter(|&pair| &pair.0.to_lowercase() == pair.0 && pair.1 > &2)
            .count()
            == 0;
        let lowercase_allowed = next.to_lowercase() == next
            && no_lowercase_more_than_two && single_lowercase_two_visits;

        not_start && (not_lowercase || lowercase_allowed)
    };
    let paths = find_paths(&input, START, &Vec::new(), &HashMap::new(), can_visit);

    paths.len() as u64
}
