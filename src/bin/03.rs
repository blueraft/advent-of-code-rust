use std::collections::HashMap;

advent_of_code::solution!(3);

#[derive(PartialEq, Eq, Clone)]
enum Symbol {
    All,
    Star,
}

fn left_idx(char_idx: usize) -> usize {
    if char_idx == 0 {
        0
    } else {
        char_idx - 1
    }
}

fn right_idx(char_idx: usize, line_len: usize) -> usize {
    if char_idx < line_len {
        char_idx + 1
    } else {
        char_idx
    }
}
fn check_line_above_or_below(
    line_to_check: &str,
    char_idx: usize,
    symbol: Symbol,
) -> Option<usize> {
    let chars = line_to_check.chars();
    let left_idx = left_idx(char_idx);
    let right_idx = right_idx(char_idx, line_to_check.len());
    for (idx, char) in line_to_check.chars().enumerate() {
        if (idx == left_idx) || (idx == right_idx) || (idx == char_idx) {
            if symbol == Symbol::All && !char.is_alphanumeric() && char != '.' {
                return Some(idx);
            } else if symbol == Symbol::Star && char == '*' {
                return Some(idx);
            }
        }
    }
    None
}

fn adjacent_to_a_symbol(
    input: &str,
    line_idx: usize,
    char_idx: usize,
    symbol: Symbol,
) -> Option<Point> {
    let mut lines = input.lines();
    let line_above_contains_symbol = {
        if line_idx > 0 {
            let line_above = input.lines().nth(line_idx - 1).unwrap();
            if let Some(idx) = check_line_above_or_below(line_above, char_idx, symbol.clone()) {
                Some(Point {
                    x: line_idx - 1,
                    y: idx,
                })
            } else {
                None
            }
        } else {
            None
        }
    };
    let line_below_contains_symbol = {
        if let Some(line_below) = input.lines().nth(line_idx + 1) {
            if let Some(idx) = check_line_above_or_below(line_below, char_idx, symbol.clone()) {
                Some(Point {
                    x: line_idx + 1,
                    y: idx,
                })
            } else {
                None
            }
        } else {
            None
        }
    };
    let current_line_contains_symbol = {
        let line = input.lines().nth(line_idx).unwrap();
        let left = left_idx(char_idx);
        let right = right_idx(char_idx, line.len());
        for (idx, char) in line.chars().enumerate() {
            if (idx == left) || (idx == right) || (idx == char_idx) {
                if symbol == Symbol::All && !char.is_alphanumeric() && char != '.' {
                    return Some(Point {
                        x: line_idx,
                        y: idx,
                    });
                } else if symbol == Symbol::Star && char == '*' {
                    return Some(Point {
                        x: line_idx,
                        y: idx,
                    });
                }
            }
        }
    };
    if line_below_contains_symbol.is_some() {
        line_below_contains_symbol
    } else if line_above_contains_symbol.is_some() {
        line_above_contains_symbol
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for (line_idx, line) in input.lines().enumerate() {
        let mut current_num: Vec<u32> = Vec::new();
        let mut adjacent = false;
        for (char_idx, char) in line.chars().enumerate() {
            if let Some(num) = char.to_digit(10) {
                current_num.push(num);
                if adjacent_to_a_symbol(input, line_idx, char_idx, Symbol::All).is_some() {
                    adjacent = true;
                }
            } else {
                if !current_num.is_empty() && adjacent {
                    let num = current_num
                        .clone()
                        .into_iter()
                        .fold(0, |acc, digit| acc * 10 + digit);
                    numbers.push(num);
                }
                current_num.clear();
                adjacent = false;
            }
        }
        if !current_num.is_empty() && adjacent {
            let num = current_num
                .clone()
                .into_iter()
                .fold(0, |acc, digit| acc * 10 + digit);
            numbers.push(num);
        }
    }
    Some(numbers.iter().sum())
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut star_map: HashMap<Point, Vec<u32>> = HashMap::new();
    for (line_idx, line) in input.lines().enumerate() {
        let mut current_num: Vec<u32> = Vec::new();
        let mut current_star_idx: Option<Point> = None;
        for (char_idx, char) in line.chars().enumerate() {
            if let Some(num) = char.to_digit(10) {
                current_num.push(num);
                if let Some(point) = adjacent_to_a_symbol(input, line_idx, char_idx, Symbol::Star) {
                    current_star_idx = Some(point)
                }
            } else {
                if let Some(star_idx) = current_star_idx {
                    if !current_num.is_empty() {
                        let num = current_num
                            .clone()
                            .into_iter()
                            .fold(0, |acc, digit| acc * 10 + digit);

                        if let Some(vec) = star_map.get_mut(&star_idx) {
                            vec.push(num);
                        } else {
                            star_map.insert(star_idx, vec![num]);
                        }
                    }
                }
                current_num.clear();
                current_star_idx = None;
            }
        }
        if let Some(star_idx) = current_star_idx {
            if !current_num.is_empty() {
                let num = current_num
                    .clone()
                    .into_iter()
                    .fold(0, |acc, digit| acc * 10 + digit);

                if let Some(vec) = star_map.get_mut(&star_idx) {
                    vec.push(num);
                } else {
                    star_map.insert(star_idx, vec![num]);
                }
            }
        }
    }
    let sum = star_map
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u32>())
        .sum();
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
