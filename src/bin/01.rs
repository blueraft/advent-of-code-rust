advent_of_code::solution!(1);

fn find_first_num<I>(chars: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    for c in chars {
        if let Some(c) = c.to_digit(10) {
            return Some(c);
        }
    }
    None
}

fn compare_substring_to_letter_num(input: &str) -> Option<u32> {
    match input.len() {
        3 => match input {
            "one" => Some(1),
            "two" => Some(2),
            "six" => Some(6),
            _ => None,
        },
        4 => match input {
            "four" => Some(4),
            "five" => Some(5),
            "nine" => Some(9),
            _ => None,
        },
        5 => match input {
            "three" => Some(3),
            "seven" => Some(7),
            "eight" => Some(8),
            _ => None,
        },
        _ => None,
    }
}

fn find_nums(line: &str) -> (Option<u32>, Option<u32>) {
    if line.len() == 0 {
        return (None, None);
    }
    let mut start = 0;
    let mut end = line.len();
    let mut first_num: Option<u32> = None;
    let mut last_num: Option<u32> = None;
    let mut chars = line.chars();
    let mut rev_chars = line.chars().rev();
    loop {
        if start == line.len() && end == 0 {
            break;
        }

        if start < line.len() && first_num.is_none() {
            if line.len() - start >= 5 {
                if let Some(num) = compare_substring_to_letter_num(&line[start..start + 5]) {
                    first_num = Some(num);
                }
            }
            if line.len() - start >= 4 {
                if let Some(num) = compare_substring_to_letter_num(&line[start..start + 4]) {
                    first_num = Some(num);
                }
            }
            if line.len() - start >= 3 {
                if let Some(num) = compare_substring_to_letter_num(&line[start..start + 3]) {
                    first_num = Some(num);
                }
            }
            if let Some(num) = chars.next().and_then(|c| c.to_digit(10)) {
                first_num = Some(num);
            }
        }

        if last_num.is_none() {
            if end >= 5 {
                if let Some(num) = compare_substring_to_letter_num(&line[end - 5..end]) {
                    last_num = Some(num);
                }
            }
            if end >= 4 {
                if let Some(num) = compare_substring_to_letter_num(&line[end - 4..end]) {
                    last_num = Some(num);
                }
            }
            if end >= 3 {
                if let Some(num) = compare_substring_to_letter_num(&line[end - 3..end]) {
                    last_num = Some(num);
                }
            }
            if let Some(num) = rev_chars.next().and_then(|c| c.to_digit(10)) {
                last_num = Some(num);
            }
        }

        if start < line.len() {
            start += 1;
        }
        if end > 0 {
            end -= 1;
        }
    }
    (first_num, last_num)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.split('\n') {
        sum += match find_first_num(line.chars()) {
            Some(num) => num * 10,
            None => 0,
        };
        sum += match find_first_num(line.chars().rev()) {
            Some(num) => num,
            None => 0,
        };
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.split('\n') {
        let (first_num, last_num) = find_nums(line);
        if let Some(num) = first_num {
            sum += num * 10;
        }
        if let Some(num) = last_num {
            sum += num;
        }
    }
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
