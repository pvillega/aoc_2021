
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn input_data(day: i32) -> Vec<String> {
    let filename = format!("resources/input{}.txt", day);

    read_file(filename)
}

pub fn sample_data(day: i32) -> Vec<String> {
    let filename = format!("resources/sample{}.txt", day);

    read_file(filename)
}

fn read_file(filename: String) -> Vec<String> {
     // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>()
}
