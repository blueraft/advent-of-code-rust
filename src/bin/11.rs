use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

fn expand_grid(input: &str, repeat_factor: usize) -> Vec<Point> {
    let vec: Vec<&str> = input.lines().collect();
    let n_cols = vec.iter().map(|line| line.len()).max().unwrap_or(0);
    let n_rows = vec.len();
    let mut col_set: HashMap<usize, bool> = HashMap::new();
    let mut row_set: HashMap<usize, bool> = HashMap::new();

    for row_index in 0..n_rows {
        row_set.insert(row_index, false);
    }
    for col_index in 0..n_cols {
        col_set.insert(col_index, false);
    }

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char == '#' {
                col_set.insert(col_idx, true);
                row_set.insert(row_idx, true);
            }
        }
    }

    let mut nodes: Vec<Point> = Vec::new();
    let mut row_diff = 0;
    let mut col_diff = 0;

    for (row_idx, row) in input.lines().enumerate() {
        row_diff = match *row_set.entry(row_idx).or_insert(false) {
            true => row_diff,
            false => row_diff + repeat_factor,
        };
        let y = row_diff + row_idx;
        for (col_idx, char) in row.chars().enumerate() {
            col_diff = match *col_set.entry(col_idx).or_insert(false) {
                true => col_diff,
                false => col_diff + repeat_factor,
            };
            let x = col_idx + col_diff;
            if char == '#' {
                nodes.push(Point {
                    x: x as f32,
                    y: y as f32,
                });
            }
        }
        col_diff = 0;
    }
    nodes
}

fn calc_distance(p1: Point, p2: Point) -> f32 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

pub fn part_one(input: &str) -> Option<f32> {
    let nodes = expand_grid(input, 1);
    let mut distances = Vec::new();
    for (a, b) in nodes.iter().tuple_combinations() {
        distances.push(calc_distance(*a, *b))
    }
    let sum = distances.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<f32> {
    let nodes = expand_grid(input, 999_999);
    let mut distances = Vec::new();
    for (a, b) in nodes.iter().tuple_combinations() {
        distances.push(calc_distance(*a, *b))
    }
    let sum = distances.iter().sum();
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
