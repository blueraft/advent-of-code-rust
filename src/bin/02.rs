use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Color {
    Blue,
    Green,
    Red,
}

fn find_game_highest_color_combo(line: &str) -> HashMap<Color, u32> {
    line.split(";")
        .map(|c| {
            c.split_terminator(",")
                .map(|num_and_color| {
                    let mut num_color_split = num_and_color.split_whitespace();
                    let num = num_color_split.next().unwrap().parse::<u32>().unwrap();
                    let name = num_color_split.next().unwrap().trim();
                    match name {
                        "blue" => (Color::Blue, num),
                        "red" => (Color::Red, num),
                        "green" => (Color::Green, num),
                        _ => panic!("Unknown color"),
                    }
                })
                .fold(HashMap::new(), |mut map, (color, count)| {
                    *map.entry(color).or_insert(0) += count;
                    map
                })
        })
        .fold(HashMap::new(), |mut acc_map, curr_map| {
            for (color, count) in curr_map {
                if let Some(current) = acc_map.get_mut(&color) {
                    if count > *current {
                        *current = count;
                    }
                } else {
                    let _ = *acc_map.entry(color).or_insert(count);
                }
            }
            acc_map
        })
}

fn game_is_possible(colors: &HashMap<Color, u32>) -> bool {
    let blue = colors.get(&Color::Blue).unwrap_or(&0).to_owned();
    let red = colors.get(&Color::Red).unwrap_or(&0).to_owned();
    let green = colors.get(&Color::Green).unwrap_or(&0).to_owned();
    (red <= 12) && (green <= 13) && (blue <= 14)
}

fn power_set(colors: &HashMap<Color, u32>) -> u32 {
    let blue = colors.get(&Color::Blue).unwrap_or(&1).to_owned();
    let red = colors.get(&Color::Red).unwrap_or(&1).to_owned();
    let green = colors.get(&Color::Green).unwrap_or(&1).to_owned();
    blue * red * green
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let mut split_line = line.split_terminator(':');
            let game_id = split_line
                .next()
                .and_then(|g| g.split_whitespace().nth(1))
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let colors = find_game_highest_color_combo(split_line.next().unwrap());
            (game_id, colors)
        })
        .filter(|(_, colors)| game_is_possible(colors))
        .map(|(game_id, _)| game_id)
        .sum::<u32>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let mut split_line = line.split_terminator(':');
            let _ = split_line.next();
            let colors = find_game_highest_color_combo(split_line.next().unwrap());
            power_set(&colors)
        })
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
