advent_of_code::solution!(9);
use itertools::Itertools;

fn get_all_seqs(line: Vec<i32>) -> Vec<Vec<i32>> {
    let mut all_seqs: Vec<Vec<i32>> = Vec::new();
    all_seqs.push(line);
    loop {
        let last = all_seqs.last().unwrap();
        if !(last.iter().any(|v| v != &0)) {
            break;
        }
        let mut new_vec = Vec::new();
        for (prev, next) in last.iter().tuple_windows() {
            new_vec.push(next - prev);
        }
        all_seqs.push(new_vec);
    }
    all_seqs
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let line_nums = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let all_seqs = get_all_seqs(line_nums);
            all_seqs.iter().map(|seq| seq.last().unwrap()).sum::<i32>()
        })
        .sum::<i32>() as u32;
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let line_nums = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let all_seqs = get_all_seqs(line_nums);
            all_seqs
                .iter()
                .map(|seq| seq.first().unwrap())
                .rev()
                .fold(0, |mut res, v| {
                    res = *v - res;
                    res
                })
        })
        .sum::<i32>() as u32;
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
