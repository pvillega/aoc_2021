mod helpers;

use crate::helpers::*;
use grid::*;

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let aoc_day = 15;
    let sample = format_input(sample_data(aoc_day));
    let input = format_input(input_data(aoc_day));

    // part 1
    let sample_result = part_1(sample.clone());
    assert_eq!(sample_result, 40);
    let result = part_1(input.clone());
    println!("part 1: {}", result);

    // part 2
    let sample_result = part_2(sample);
    assert_eq!(sample_result, 315);
    let result = part_2(input);
    println!("part 2: {}", result);

    Ok(())
}

fn format_input(input: Vec<String>) -> Grid<u64> {
    let columns = input[0].len();
    let array = input
        .into_iter()
        .flat_map(|s| {
            s.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid::from_vec(array, columns)
}

fn part_1(input: Grid<u64>) -> u64 {
    // println!("{:?}", input);
    let map_weights = find_path_lower_risk(&input, (0, 0), (input.rows() - 1, input.cols() - 1));
    map_weights[input.rows() - 1][input.cols() - 1]
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Score {
    score: u64,
    position: (usize, usize),
}

// reverse order so that heap retruns smaller items first
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score).map(|c| c.reverse())
    }
}

// reverse order so that heap retruns smaller items first
impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

// trying to implement Dijkstra with a minor variation in here
fn find_path_lower_risk(
    input: &Grid<u64>,
    start: (usize, usize),
    target: (usize, usize),
) -> Grid<u64> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // initialise score grid with infinity for all positions except origin
    let mut scores: Grid<u64> = Grid::init(input.rows(), input.cols(), u64::MAX);
    scores[start.0][start.1] = 0;

    // store scores in a heap (ordered by score) as it will be needed by the algorithm to select the next position to visit
    let mut visit_next: BinaryHeap<Score> = BinaryHeap::new();
    for r in 0..scores.rows() {
        for c in 0..scores.cols() {
            visit_next.push(Score {
                score: scores[r][c],
                position: (r, c),
            });
        }
    }

    // start with initial position (has weight 0) and visit all neighbours
    while let Some(Score { score, position }) = visit_next.pop() {
        // println!("{:?} {:?}", position, score);
        let current_node = position;
        // exit if we are done
        if current_node == target {
            break;
        }

        // calculate potential neighbours
        let mut neighbours: Vec<(usize, usize)> = Vec::new();
        if current_node.0 > 0 {
            neighbours.push((current_node.0 - 1, current_node.1));
        }
        if current_node.0 < input.cols() - 1 {
            neighbours.push((current_node.0 + 1, current_node.1));
        }
        if current_node.1 > 0 {
            neighbours.push((current_node.0, current_node.1 - 1));
        }
        if current_node.1 < input.rows() - 1 {
            neighbours.push((current_node.0, current_node.1 + 1));
        }

        // update scores
        for next_node in neighbours {
            // skip visited nodes
            if visited.contains(&next_node) {
                continue;
            }
            // get score for node
            let temptative_score = score + input[next_node.0][next_node.1];
            let next_node_score = if temptative_score < scores[next_node.0][next_node.1] {
                temptative_score
            } else {
                scores[next_node.0][next_node.1]
            };

            // update grid and heap with score
            scores[next_node.0][next_node.1] = next_node_score;
            visit_next.push(Score {
                score: next_node_score,
                position: next_node,
            });
        }
        // mark current node as visited and remove it from visit_next set
        visited.insert(current_node);
        visit_next = visit_next
            .into_iter()
            .filter(|s| s.position != current_node)
            .collect::<BinaryHeap<Score>>();
    }

    scores
}

fn part_2(input: Grid<u64>) -> u64 {
    // println!("{:?}", input);
    let expanded_input = expand_input(&input);
    let map_weights = find_path_lower_risk_part_2(
        &expanded_input,
        (0, 0),
        (expanded_input.rows() - 1, expanded_input.cols() - 1),
    );
    map_weights[expanded_input.rows() - 1][expanded_input.cols() - 1]
}

fn expand_input(input: &Grid<u64>) -> Grid<u64> {
    let mut new_grid = Grid::init(input.rows() * 5, input.cols() * 5, 0);

    for r in 0..new_grid.rows() {
        for c in 0..new_grid.cols() {
            let input_r = r % input.rows();
            let input_c = c % input.cols();
            let modifier = ((r / input.rows()) + (c / input.cols())) as u64;
            // formula below is modification on normal modulo so that we circle on 1, not 0
            new_grid[r][c] = (input[input_r][input_c] + modifier - 1) % 9 + 1;
        }
    }

    new_grid
}

// trying to implement A* with a minor variation in here as Dijkstra is too slow for a bigger grid
// https://en.wikipedia.org/wiki/A*_search_algorithm
fn find_path_lower_risk_part_2(
    input: &Grid<u64>,
    start: (usize, usize),
    target: (usize, usize),
) -> Grid<u64> {
    // heuristic for A*, we use simple distance to estimate cheaper nodes
    fn h(node: (usize, usize), dest: (usize, usize)) -> u64 {
        let x = dest.0 - node.0;
        let y = dest.1 - node.1;
        (x + y) as u64
    }

    // set a heap with the start node and the heuristic for the node
    let mut open_set: BinaryHeap<Score> = BinaryHeap::new();
    open_set.push(Score {
        score: h(start, target),
        position: start,
    });

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // initialise score grid with infinity for all positions except origin
    let mut g_score: Grid<u64> = Grid::init(input.rows(), input.cols(), u64::MAX);
    g_score[start.0][start.1] = 0;

    while let Some(Score { score, position }) = open_set.pop() {
        let current_node = position;
        // exit if we are done
        if current_node == target {
            break;
        }

        // remove current node from visit set
        open_set = open_set
            .into_iter()
            .filter(|s| s.position != current_node)
            .collect::<BinaryHeap<Score>>();

        // calculate potential neighbours
        let mut neighbours: Vec<(usize, usize)> = Vec::new();
        if current_node.0 > 0 {
            neighbours.push((current_node.0 - 1, current_node.1));
        }
        if current_node.0 < input.cols() - 1 {
            neighbours.push((current_node.0 + 1, current_node.1));
        }
        if current_node.1 > 0 {
            neighbours.push((current_node.0, current_node.1 - 1));
        }
        if current_node.1 < input.rows() - 1 {
            neighbours.push((current_node.0, current_node.1 + 1));
        }

        for next_node in neighbours {
            // check score recorded in grid and cost of moving to next node
            let tentative_g_score =
                g_score[current_node.0][current_node.1] + input[next_node.0][next_node.1];
            // if we have improved on neighbour cost, update score and came_from
            if tentative_g_score < g_score[next_node.0][next_node.1] {
                came_from.insert(next_node, current_node);
                g_score[next_node.0][next_node.1] = tentative_g_score;
                // add neighbour to nodes to explore, along heuristic
                open_set.push(Score {
                    score: tentative_g_score + h(next_node, target),
                    position: next_node,
                });
            }
        }
    }

    g_score
}
