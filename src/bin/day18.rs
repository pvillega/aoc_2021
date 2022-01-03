mod helpers;
use crate::helpers::*;
use crate::BranchExploded::*;
use crate::Exploded::*;
use std::{error::Error, fmt::Display};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 18;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 4140);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 3993);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pair {
    Leaf(u64),
    Branch(Box<Pair>, Box<Pair>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BranchExploded {
    LeftBranch,
    RightBranch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Exploded {
    Both(u64, u64),
    Left(u64, BranchExploded),
    Right(u64, BranchExploded),
    Exploded,
    DidNothing,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Leaf(n) => write!(f, "{}", n),
            Pair::Branch(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

fn format_input(input: Vec<String>) -> Vec<Pair> {
    input.into_iter().map(|s| parse_pair(s)).collect()
}

fn parse_pair(s: String) -> Pair {
    // The parsing assumes we always have a left side, a comma, and a right side.
    // Each side can be either another expression or a number, so we run it recursively
    fn inner_parse(chars: &mut dyn Iterator<Item = char>) -> Pair {
        match chars.next().unwrap() {
            '[' => {
                let first = inner_parse(chars);
                chars.next(); // ,
                let second = inner_parse(chars);
                chars.next(); // ]
                Pair::Branch(Box::new(first), Box::new(second))
            }
            num => Pair::Leaf(num.to_digit(10).unwrap() as u64),
        }
    }
    inner_parse(&mut s.chars())
}

fn part_1(input: Vec<Pair>) -> u64 {
    // println!("{:?}", input);
    let first_pair = input[0].clone();
    let sum = input
        .into_iter()
        .skip(1)
        .fold(first_pair, |acc, p| add(&acc, &p));

    magnitude(&sum)
}

fn part_2(input: Vec<Pair>) -> u64 {
    // println!("{:?}", input);
    let mut max_magnitude = 0;
    for p in input.clone() {
        for p2 in input.clone() {
            let sum = add(&p, &p2);
            let mag = magnitude(&sum);

            let sum2 = add(&p2, &p);
            let mag2 = magnitude(&sum2);

            let local_max = mag.max(mag2);
            if local_max > max_magnitude {
                max_magnitude = local_max;
            }
        }
    }
    max_magnitude
}

fn add(l: &Pair, r: &Pair) -> Pair {
    let mut new_pair = Pair::Branch(Box::new(l.clone()), Box::new(r.clone()));

    let mut keep_going = true;
    while keep_going {
        // we use the mutable value to modify the tree in place, as otherwise the recursion gets complicated
        // we iterate until neither action causes a change to the pair
        keep_going = explode(&mut new_pair) || splits(&mut new_pair);
    }

    new_pair
}

// If any pair is nested inside four pairs, the leftmost such pair explodes.
// mutating the input parameter to make it simpler to implement
fn explode(pair: &mut Pair) -> bool {
    // we need to push numbers up, both left and right side, so we need a tuple. We put it inside an option as some nodes won't
    // explode and we need a base case (None)
    fn check_level(pair: &mut Pair, level: usize) -> Exploded {
        match pair {
            Pair::Leaf(_) => DidNothing,
            Pair::Branch(left, right) => {
                if level == 4 {
                    // if explode, get numbers from leaves
                    let l = if let Pair::Leaf(n) = **left { n } else { 0 };
                    let r = if let Pair::Leaf(n) = **right { n } else { 0 };
                    // current pair is now a leaf with a 0 value due to the explosion
                    *pair = Pair::Leaf(0);
                    Both(l, r)
                } else {
                    // we need to keep traversing the tree. We start with the left side, then right if left didn't cause any change
                    match check_level(left, level + 1) {
                        Both(l, r) => {
                            *pair = Pair::Branch(
                                left.clone(),
                                Box::new(add_to_closest_on_left_side(right, r)),
                            );

                            Left(l, LeftBranch)
                        }
                        Left(l, lm) => {
                            // need to filter to ensure we are not hitting the left wall otherwise we add to the left
                            if lm == RightBranch {
                                *pair = Pair::Branch(
                                    Box::new(add_to_closest_on_right_side(left, l)),
                                    right.clone(),
                                );
                                Exploded
                            } else {
                                Left(l, lm)
                            }
                        }
                        Right(r, _) => {
                            *pair = Pair::Branch(
                                left.clone(),
                                Box::new(add_to_closest_on_left_side(right, r)),
                            );

                            Exploded
                        }
                        Exploded => Exploded,
                        DidNothing => match check_level(right, level + 1) {
                            Both(l, r) => {
                                *pair = Pair::Branch(
                                    Box::new(add_to_closest_on_right_side(left, l)),
                                    right.clone(),
                                );

                                Right(r, RightBranch)
                            }
                            Left(l, _) => {
                                *pair = Pair::Branch(
                                    Box::new(add_to_closest_on_right_side(left, l)),
                                    right.clone(),
                                );

                                Exploded
                            }
                            Right(r, lm) => {
                                if lm == LeftBranch {
                                    *pair = Pair::Branch(
                                        left.clone(),
                                        Box::new(add_to_closest_on_left_side(right, r)),
                                    );
                                    Exploded
                                } else {
                                    Right(r, lm)
                                }
                            }
                            Exploded => Exploded,
                            DidNothing => DidNothing,
                        },
                    }
                }
            }
        }
    }

    match check_level(pair, 0) {
        Exploded::DidNothing => false,
        _ => true,
    }
}

fn add_to_closest_on_left_side(pair: &Pair, n: u64) -> Pair {
    match pair {
        Pair::Leaf(l) => Pair::Leaf(l + n),
        Pair::Branch(left, right) => Pair::Branch(
            Box::new(add_to_closest_on_left_side(left, n)),
            right.clone(),
        ),
    }
}

fn add_to_closest_on_right_side(pair: &Pair, n: u64) -> Pair {
    match pair {
        Pair::Leaf(l) => Pair::Leaf(l + n),
        Pair::Branch(left, right) => Pair::Branch(
            left.clone(),
            Box::new(add_to_closest_on_right_side(right, n)),
        ),
    }
}

// If any regular number is 10 or greater, the leftmost such regular number splits.
// It will return true if we have done some modification, as we need to know if we need to loop again
// Using mutability to avoid having to deal with tuples on return
fn splits(pair: &mut Pair) -> bool {
    match pair {
        Pair::Branch(left, right) => {
            let splitted = splits(left) || splits(right);
            *pair = Pair::Branch(left.clone(), right.clone());
            splitted
        }
        Pair::Leaf(n) => {
            let value = *n;
            if value > 9 {
                *pair = Pair::Branch(
                    Box::new(Pair::Leaf(value / 2)),
                    Box::new(Pair::Leaf(value / 2 + value % 2)),
                );
                true
            } else {
                false
            }
        }
    }
}

fn magnitude(p: &Pair) -> u64 {
    match p {
        Pair::Leaf(n) => *n,
        Pair::Branch(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}
