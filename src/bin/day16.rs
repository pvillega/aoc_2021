mod helpers;

use crate::helpers::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 16;
    let mut sample = format_input(sample_data(aoc_day));
    let mut input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 20);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(&mut sample);
    assert_eq!(sample_result, 1);
    let result = part_2(&mut input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<String> {
    input[0]
        .chars()
        .into_iter()
        .flat_map(|s| match s {
            '0' => vec!["0", "0", "0", "0"],
            '1' => vec!["0", "0", "0", "1"],
            '2' => vec!["0", "0", "1", "0"],
            '3' => vec!["0", "0", "1", "1"],
            '4' => vec!["0", "1", "0", "0"],
            '5' => vec!["0", "1", "0", "1"],
            '6' => vec!["0", "1", "1", "0"],
            '7' => vec!["0", "1", "1", "1"],
            '8' => vec!["1", "0", "0", "0"],
            '9' => vec!["1", "0", "0", "1"],
            'A' => vec!["1", "0", "1", "0"],
            'B' => vec!["1", "0", "1", "1"],
            'C' => vec!["1", "1", "0", "0"],
            'D' => vec!["1", "1", "0", "1"],
            'E' => vec!["1", "1", "1", "0"],
            'F' => vec!["1", "1", "1", "1"],
            _ => vec!["0", "0", "0", "0"],
        })
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
}

const SUM: u64 = 0;
const PRODUCT: u64 = 1;
const MIN: u64 = 2;
const MAX: u64 = 3;
const LITERAL: u64 = 4;
const GT: u64 = 5;
const LT: u64 = 6;
const EQ: u64 = 7;

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    version: u64,
    type_id: u64,
    // payload may be different for each packet, a value or some children
    value: u64,
    children: Vec<Packet>,
    // store length to keep track
    length: u64,
}

fn part_1(mut input: Vec<String>) -> u64 {
    // println!("{:?}", input);
    let packet = parse_packet_versions(&mut input);
    // println!("{:?}", packet);
    // get versions sum
    sum_versions(&packet)
}

fn sum_versions(packet: &Packet) -> u64 {
    packet.version
        + (&packet.children)
            .into_iter()
            .map(|p| sum_versions(&p))
            .sum::<u64>()
}

fn parse_packet_versions(mut input: &mut Vec<String>) -> Packet {
    // println!("to_parse: {:?} {:?}", input.len(), &input);
    let mut headers = input.drain(..6).collect::<Vec<_>>();
    let mut processed_length = 0;
    let mut value = 0;
    let mut children = vec![];

    // process headers for packet
    let type_id_str = headers.drain(3..).collect::<String>();
    let type_id = to_int(&type_id_str);
    let version_str = headers.into_iter().take(3).collect::<String>();
    let version = to_int(&version_str);
    processed_length = 6;

    // process payload
    if type_id == LITERAL {
        // for a literal value, the payload is the value we just read
        let mut last_fragment = false;
        let mut value_str = String::new();
        while !last_fragment {
            last_fragment = input[0] == "0";

            let digit = input.drain(..5).skip(1).collect::<String>();
            value_str.push_str(&digit);

            processed_length += 5;
        }
        value = to_int(&value_str);
        println!("literal: {}", value);
    } else {
        // operator packet, split by type as per digit
        let type_id = input.drain(..1).collect::<String>();
        processed_length += 1;
        if type_id == "0" {
            // we have a fix number of bytes to process, find how many and cut the input to that length
            let subpacket_len_str = input.drain(..15).collect::<String>();
            let subpacket_len = to_int(&subpacket_len_str);
            processed_length += 15;
            println!("subpacket_len: {}", subpacket_len);

            let mut bits_parsed = 0;
            while bits_parsed < subpacket_len {
                // get a child
                let child = parse_packet_versions(&mut input);
                // println!("child: {:?}", &child);

                // update metrics
                bits_parsed += child.length;
                processed_length += child.length;

                children.push(child);
            }
        } else {
            // we have a number of packets, so we need to parse them
            let number_packets_str = input.drain(..11).collect::<String>();
            let number_packets = to_int(&number_packets_str) as usize;
            processed_length += 11;
            println!("number_packets: {}", number_packets);

            // fold over what's left to parse until we have hit the limit of packets to parse
            for _ in 0..number_packets {
                // get a child
                let child = parse_packet_versions(&mut input);
                // println!("child: {:?}", &child);

                // update metrics
                processed_length += child.length;

                children.push(child);
            }
        }
    };

    // return parsed data
    Packet {
        version,
        type_id,
        value,
        children,
        length: processed_length,
    }
}

fn to_int(s: &String) -> u64 {
    isize::from_str_radix(s, 2).unwrap().try_into().unwrap()
}

fn part_2(mut input: &mut Vec<String>) -> u64 {
    // println!("{:?}", input);
    let packet = parse_packet_versions(&mut input);
    // println!("{:?}", packet);
    // get sum
    sum_literals(&packet)
}

fn sum_literals(packet: &Packet) -> u64 {
    match packet.type_id {
        SUM => (&packet.children)
            .into_iter()
            .map(|p| sum_literals(&p))
            .sum(),
        PRODUCT => (&packet.children)
            .into_iter()
            .map(|p| sum_literals(&p))
            .product(),
        MIN => (&packet.children)
            .into_iter()
            .map(|p| sum_literals(&p))
            .min()
            .unwrap(),
        MAX => (&packet.children)
            .into_iter()
            .map(|p| sum_literals(&p))
            .max()
            .unwrap(),
        LITERAL => packet.value,
        GT => {
            let left = sum_literals(&packet.children[0]);
            let right = sum_literals(&packet.children[1]);
            if left > right {
                1
            } else {
                0
            }
        }
        LT => {
            let left = sum_literals(&packet.children[0]);
            let right = sum_literals(&packet.children[1]);
            if left < right {
                1
            } else {
                0
            }
        }
        EQ => {
            let left = sum_literals(&packet.children[0]);
            let right = sum_literals(&packet.children[1]);
            if left == right {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
