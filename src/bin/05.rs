use std::collections::HashMap;

use rayon::prelude::*;

advent_of_code::solution!(5);

enum MapType {
    Seeds,
    SeedsToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn convert_range_map_to_hash_map(line: &str, map: &mut HashMap<u64, (u64, u64)>) {
    let mut line_split = line.split_whitespace();
    let destination = line_split.next().unwrap().parse::<u64>().unwrap();
    let source = line_split.next().unwrap().parse::<u64>().unwrap();
    let length = line_split.next().unwrap().parse::<u64>().unwrap();
    map.insert(source, (destination, length));
}

fn find_mapped_value(key: &u64, value_map: &HashMap<u64, (u64, u64)>) -> u64 {
    let values: Vec<u64> = value_map
        .iter()
        .filter_map(|(s, (d, l))| {
            if s <= key && key < &(s + l) {
                let diff = key - s;
                Some(d + diff)
            } else {
                None
            }
        })
        .collect();
    if let Some(num) = values.first() {
        *num
    } else {
        *key
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut seeds_to_soil: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut soil_to_fertilizer: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut fertilizer_to_water: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut water_to_light: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut light_to_temperature: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut temperature_to_humidity: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut humidity_to_location: HashMap<u64, (u64, u64)> = HashMap::new();

    let mut line_type: MapType = MapType::Seeds;
    for line in input.lines() {
        match line_type {
            MapType::Seeds => {
                if line.is_empty() {
                    line_type = MapType::SeedsToSoil;
                } else {
                    let s: Vec<u64> = line
                        .strip_prefix("seeds:")
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    seeds.extend(s);
                }
            }
            MapType::SeedsToSoil => {
                if line.is_empty() {
                    line_type = MapType::SoilToFertilizer;
                    println!("Finished");
                } else if !line.contains("seed") {
                    convert_range_map_to_hash_map(line, &mut seeds_to_soil);
                }
            }
            MapType::SoilToFertilizer => {
                if line.is_empty() {
                    line_type = MapType::FertilizerToWater;
                    println!("Finished");
                } else if !line.contains("fertilizer") {
                    convert_range_map_to_hash_map(line, &mut soil_to_fertilizer);
                }
            }
            MapType::FertilizerToWater => {
                if line.is_empty() {
                    line_type = MapType::WaterToLight;
                    println!("Finished");
                } else if !line.contains("fertilizer") {
                    convert_range_map_to_hash_map(line, &mut fertilizer_to_water);
                }
            }
            MapType::WaterToLight => {
                if line.is_empty() {
                    line_type = MapType::LightToTemperature;
                    println!("Finished");
                } else if !line.contains("water") {
                    convert_range_map_to_hash_map(line, &mut water_to_light);
                }
            }
            MapType::LightToTemperature => {
                if line.is_empty() {
                    line_type = MapType::TemperatureToHumidity;
                    println!("Finished");
                } else if !line.contains("light") {
                    convert_range_map_to_hash_map(line, &mut light_to_temperature);
                }
            }
            MapType::TemperatureToHumidity => {
                if line.is_empty() {
                    line_type = MapType::HumidityToLocation;
                    println!("Finished");
                } else if !line.contains("temperature") {
                    convert_range_map_to_hash_map(line, &mut temperature_to_humidity);
                }
            }
            MapType::HumidityToLocation => {
                if line.is_empty() {
                    break;
                } else if !line.contains("location") {
                    convert_range_map_to_hash_map(line, &mut humidity_to_location);
                }
            }
        }
    }
    println!("Finished mapping");
    let lowest: Option<u64> = seeds
        .iter()
        .map(|s| {
            let soil = find_mapped_value(s, &seeds_to_soil);
            let fertilizer = find_mapped_value(&soil, &soil_to_fertilizer);
            let water = find_mapped_value(&fertilizer, &fertilizer_to_water);
            let light = find_mapped_value(&water, &water_to_light);
            let temperature = find_mapped_value(&light, &light_to_temperature);
            let humidity = find_mapped_value(&temperature, &temperature_to_humidity);
            find_mapped_value(&humidity, &humidity_to_location)
        })
        .min();
    lowest
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut seeds_to_soil: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut soil_to_fertilizer: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut fertilizer_to_water: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut water_to_light: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut light_to_temperature: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut temperature_to_humidity: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut humidity_to_location: HashMap<u64, (u64, u64)> = HashMap::new();

    let mut line_type: MapType = MapType::Seeds;
    for line in input.lines() {
        match line_type {
            MapType::Seeds => {
                if line.is_empty() {
                    line_type = MapType::SeedsToSoil;
                } else {
                    let seed_iter: Vec<&str> = line
                        .strip_prefix("seeds:")
                        .unwrap()
                        .split_whitespace()
                        .collect();
                    for slice in seed_iter.chunks(2) {
                        println!("{slice:?}");
                        let start = slice[0].parse::<u64>().unwrap();
                        let range = slice[1].parse::<u64>().unwrap();
                        seeds.extend(start..start + range - 1);
                    }
                }
            }
            MapType::SeedsToSoil => {
                if line.is_empty() {
                    line_type = MapType::SoilToFertilizer;
                } else if !line.contains("seed") {
                    convert_range_map_to_hash_map(line, &mut seeds_to_soil);
                }
            }
            MapType::SoilToFertilizer => {
                if line.is_empty() {
                    line_type = MapType::FertilizerToWater;
                } else if !line.contains("fertilizer") {
                    convert_range_map_to_hash_map(line, &mut soil_to_fertilizer);
                }
            }
            MapType::FertilizerToWater => {
                if line.is_empty() {
                    line_type = MapType::WaterToLight;
                } else if !line.contains("fertilizer") {
                    convert_range_map_to_hash_map(line, &mut fertilizer_to_water);
                }
            }
            MapType::WaterToLight => {
                if line.is_empty() {
                    line_type = MapType::LightToTemperature;
                } else if !line.contains("water") {
                    convert_range_map_to_hash_map(line, &mut water_to_light);
                }
            }
            MapType::LightToTemperature => {
                if line.is_empty() {
                    line_type = MapType::TemperatureToHumidity;
                } else if !line.contains("light") {
                    convert_range_map_to_hash_map(line, &mut light_to_temperature);
                }
            }
            MapType::TemperatureToHumidity => {
                if line.is_empty() {
                    line_type = MapType::HumidityToLocation;
                } else if !line.contains("temperature") {
                    convert_range_map_to_hash_map(line, &mut temperature_to_humidity);
                }
            }
            MapType::HumidityToLocation => {
                if line.is_empty() {
                    break;
                } else if !line.contains("location") {
                    convert_range_map_to_hash_map(line, &mut humidity_to_location);
                }
            }
        }
    }

    let lowest: Option<u64> = seeds
        .par_iter()
        .map(|s| {
            let soil = find_mapped_value(s, &seeds_to_soil);
            let fertilizer = find_mapped_value(&soil, &soil_to_fertilizer);
            let water = find_mapped_value(&fertilizer, &fertilizer_to_water);
            let light = find_mapped_value(&water, &water_to_light);
            let temperature = find_mapped_value(&light, &light_to_temperature);
            let humidity = find_mapped_value(&temperature, &temperature_to_humidity);
            find_mapped_value(&humidity, &humidity_to_location)
        })
        .min();
    lowest
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
