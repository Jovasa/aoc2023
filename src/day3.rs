use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // Read the engine schematic from a file
    let input_file = "data/day3.txt";
    let engine_schematic = match fs::read_to_string(input_file) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error reading the input file");
            return;
        }
    };


    let symbol_surroundings = get_coords_of_non_digit_and_non_dot_characters(&engine_schematic);
    // Extract the part numbers from the engine schematic
    let part_numbers = extract_part_numbers_without_neighbouring_symbols(&engine_schematic, &symbol_surroundings);

    let digit_map = map_each_digit_to_the_starting_point_of_its_part_number(&engine_schematic);

    let mut sum_part_numbers = 0;
    for (coord, part_num) in &part_numbers {
        sum_part_numbers += part_num;
    }
    println!("sum of part numbers: {}", sum_part_numbers);

    let gears = find_gears(&engine_schematic, &digit_map, &part_numbers);
    println!("gears: {}", gears);
}


fn find_gears(engine_schematic: &str,
              digit_map: &HashMap<(usize, usize), (usize, usize)>,
              part_numbers: &HashMap<(usize, usize), u32>) -> usize {
    let mut gears = 0;

    for (y, line) in engine_schematic.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '*' { continue; }

            let surroundings = get_surrounding_coords(x, y);

            let mut part_number_starting_points = HashSet::new();

            for coord in surroundings {
                if let Some(starting_point) = digit_map.get(&coord) {
                    part_number_starting_points.insert(starting_point);
                }
            }
            if part_number_starting_points.len() == 2 {
                let mut part_number_sum = 1;
                for starting_point in part_number_starting_points {
                    part_number_sum *= part_numbers.get(starting_point).unwrap();
                }
                gears += part_number_sum as usize;
            }
        }
    }
    gears
}

fn extract_part_numbers_without_neighbouring_symbols(engine_schematic: &str, symbol_surroundings: &HashSet<(usize, usize)>) -> HashMap<(usize, usize), u32> {
    let mut part_numbers = HashMap::new();

    for (y, line) in engine_schematic.lines().enumerate() {
        let mut line_iter = line.chars().enumerate().into_iter();

        while let Some((x, c)) = line_iter.next() {
            if c.is_digit(10) {
                let mut part_number = c.to_digit(10).unwrap();
                let mut has_neighbouring_symbol = symbol_surroundings.contains(&(x, y));
                while let Some((x, c)) = line_iter.next() {
                    if c.is_digit(10) {
                        part_number *= 10;
                        part_number += c.to_digit(10).unwrap();
                        if symbol_surroundings.contains(&(x, y)) {
                            has_neighbouring_symbol = true;
                        }
                    } else {
                        break;
                    }
                }
                if has_neighbouring_symbol {
                    part_numbers.insert((x, y), part_number);
                }
            }
        }
    }

    part_numbers
}

fn map_each_digit_to_the_starting_point_of_its_part_number(engine_schematic: &str) -> HashMap<(usize, usize), (usize, usize)> {
    let mut part_numbers = HashMap::new();

    for (y, line) in engine_schematic.lines().enumerate() {
        let mut line_iter = line.chars().enumerate().into_iter();

        while let Some((x, c)) = line_iter.next() {
            if c.is_digit(10) {
                let start_point = (x, y);
                part_numbers.insert((x, y), start_point);
                while let Some((x, c)) = line_iter.next() {
                    if c.is_digit(10) {
                        part_numbers.insert((x, y), start_point);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    part_numbers
}

fn get_coords_of_non_digit_and_non_dot_characters(engine_schematic: &str) -> HashSet<(usize, usize)> {
    let mut coords = HashSet::new();

    for (y, line) in engine_schematic.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                coords.extend(get_surrounding_coords(x, y));
            }
        }
    }

    coords
}

fn get_surrounding_coords(x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut coords = HashSet::new();

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            coords.insert((i, j));
        }
    }

    coords
}