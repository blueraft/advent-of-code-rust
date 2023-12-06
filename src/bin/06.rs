advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.lines().into_iter();

    let values: Vec<(u32, u32)> = iter
        .next()
        .and_then(|time_line| {
            let distance_line = iter.next().unwrap();

            let time_values: Vec<u32> = time_line
                .strip_prefix("Time:")
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let distance_values: Vec<u32> = distance_line
                .strip_prefix("Distance:")
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Some(
                time_values
                    .into_iter()
                    .zip(distance_values.into_iter())
                    .collect(),
            )
        })
        .unwrap_or_else(|| Vec::new());

    let sum = values
        .into_iter()
        .map(|(t, d)| {
            let mut combinations = 0;
            for speed in (0..t) {
                let traveled = speed * (t - speed);
                if traveled > d {
                    combinations += 1;
                }
            }
            combinations
        })
        .product();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.lines().into_iter();

    let values: (u64, u64) = iter
        .next()
        .and_then(|time_line| {
            let distance_line = iter.next().unwrap();

            let time_values: u64 = time_line
                .strip_prefix("Time:")
                .unwrap()
                .replace(" ", "")
                .parse()
                .unwrap();

            let distance_values: u64 = distance_line
                .strip_prefix("Distance:")
                .unwrap()
                .replace(" ", "")
                .parse()
                .unwrap();

            Some((time_values, distance_values))
        })
        .unwrap();

    let mut combinations: u32 = 0;
    for speed in (0..values.0) {
        let traveled = speed * (values.0 - speed);
        if traveled > values.1 {
            combinations += 1;
        }
    }
    Some(combinations)
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
