advent_of_code::solution!(8);
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Node {
    root: String,
    left: String,
    right: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().into_iter();
    let initial_instructions: Vec<Side> = lines
        .next()
        .unwrap()
        .chars()
        .map(|s| match s {
            'L' => Side::Left,
            'R' => Side::Right,
            _ => panic!("Uknown side"),
        })
        .collect();
    // remove the blank line
    lines.next().unwrap();
    let nodes: Vec<Node> = lines
        .filter_map(|n| {
            if let Some((root, children)) = n.split_once("=") {
                if let Some((left, right)) = children.trim().split_once(',') {
                    Some(Node {
                        root: root.trim().to_owned(),
                        left: left
                            .trim_matches(|c| c == '(' || c == ')')
                            .trim()
                            .to_owned(),
                        right: right
                            .trim_matches(|c| c == '(' || c == ')')
                            .trim()
                            .to_owned(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut current_instructions = initial_instructions.clone().into_iter();
    let mut destination: String = "AAA".into();
    let mut steps = 0;
    loop {
        if destination == "ZZZ" {
            return Some(steps);
        } else {
            if let Some(side) = current_instructions.next() {
                let source: &Node = nodes
                    .iter()
                    .filter(|n| n.root == destination)
                    .collect::<Vec<&Node>>()
                    .first()
                    .unwrap();
                match side {
                    Side::Left => destination = source.left.clone(),
                    Side::Right => destination = source.right.clone(),
                }
                steps += 1;
            } else {
                current_instructions = initial_instructions.clone().into_iter();
            }
        }
    }
}

fn find_part_two_steps(
    nodes: Vec<Node>,
    mut destination: String,
    initial_instructions: Vec<Side>,
) -> Option<u32> {
    let mut steps: u32 = 0;
    let mut current_instructions = initial_instructions.clone().into_iter();
    println!("starting");
    loop {
        if destination.ends_with('Z') {
            println!("hello");
            return Some(steps);
        } else {
            if let Some(side) = current_instructions.next() {
                let source: &Node = nodes
                    .iter()
                    .filter(|n| n.root == destination)
                    .collect::<Vec<&Node>>()
                    .first()
                    .unwrap();
                match side {
                    Side::Left => destination = source.left.clone(),
                    Side::Right => destination = source.right.clone(),
                }
                steps += 1;
            } else {
                current_instructions = initial_instructions.clone().into_iter();
            }
        }
    }
}

// Find GCD
fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: u32, b: u32) -> u32 {
    // LCM = a*b / gcd
    return a * (b / gcd(a, b));
}

fn lcm_vec(numbers: Vec<u32>) -> u32 {
    if numbers.is_empty() {
        return 1; // LCM of an empty set is 1
    }

    let mut result = numbers[0];

    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().into_iter();
    let initial_instructions: Vec<Side> = lines
        .next()
        .unwrap()
        .chars()
        .map(|s| match s {
            'L' => Side::Left,
            'R' => Side::Right,
            _ => panic!("Uknown side"),
        })
        .collect();
    // remove the blank line
    lines.next().unwrap();
    let nodes: Vec<Node> = lines
        .filter_map(|n| {
            if let Some((root, children)) = n.split_once("=") {
                if let Some((left, right)) = children.trim().split_once(',') {
                    Some(Node {
                        root: root.trim().to_owned(),
                        left: left
                            .trim_matches(|c| c == '(' || c == ')')
                            .trim()
                            .to_owned(),
                        right: right
                            .trim_matches(|c| c == '(' || c == ')')
                            .trim()
                            .to_owned(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut destinations = nodes
        .iter()
        .filter(|n| n.root.ends_with('A') || n.root.ends_with('A'))
        .collect::<Vec<&Node>>();

    let steps: Vec<u32> = destinations
        .iter()
        .map(|&destination| {
            find_part_two_steps(
                nodes.clone(),
                destination.root.clone(),
                initial_instructions.clone(),
            )
            .unwrap()
        })
        .collect();
    let res = lcm_vec(steps);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    //
    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
