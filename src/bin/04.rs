use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

fn conver_str_of_nums_to_set(line: &str) -> HashSet<u32> {
    line.split_whitespace()
        .filter_map(|n| {
            if let Ok(num) = n.parse::<u32>() {
                Some(num)
            } else {
                None
            }
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let mut split_line = line.split_terminator('|');
            let winning_numbers = conver_str_of_nums_to_set(split_line.next().unwrap());
            let nums_you_have = conver_str_of_nums_to_set(split_line.next().unwrap());
            winning_numbers
                .intersection(&nums_you_have)
                .enumerate()
                .fold(0, |acc, (i, _)| if i == 0 { 1 } else { acc * 2 })
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_copies_map: HashMap<usize, u32> =
        input.lines().enumerate().map(|(i, _)| (i, 1)).collect();

    for (i, line) in input.lines().enumerate() {
        let mut split_line = line.split_terminator('|');
        let winning_numbers = conver_str_of_nums_to_set(split_line.next().unwrap());
        let nums_you_have = conver_str_of_nums_to_set(split_line.next().unwrap());
        let intersection = winning_numbers.intersection(&nums_you_have);
        let current_instance_count = num_copies_map.get(&i).unwrap().clone();
        for (j, _) in intersection.enumerate() {
            let idx = i + j + 1;
            if let Some(num) = num_copies_map.get_mut(&idx) {
                *num += current_instance_count;
            }
        }
    }

    let sum = num_copies_map.values().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
