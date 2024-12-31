use std::collections::HashSet;
use std::collections::HashMap;
use crate::utils::{read_into_vec_of_strings, generate_grid};

pub fn run_a(file_path: &str) -> usize {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let x_max: i32 = input_string[0].len() as i32;
    let y_max: i32 = input_string.len() as i32;

    let grid = generate_grid(input_string);
    // println!("{grid:?}");

    // Gather all antennas and their locations
    let mut antennas: HashMap<(i32, i32), char> = HashMap::new();
    let mut antenna_locations: Vec<(i32, i32)> = vec![];
    for y in 0..y_max {
        for x in 0..x_max {
            let character = grid.get(&(y, x)).unwrap();
            if *character != '.' {
                antennas.insert((y, x), *character);
                antenna_locations.push((y, x));
            }
        }
    }

    let mut antinode_locations = HashSet::new();

    // Check antenna pairs
    for location_a in &antenna_locations {
        for location_b in &antenna_locations {
            if location_a != location_b {
                let antenna_char_1 = antennas.get(&location_a);
                let antenna_char_2 = antennas.get(&location_b);

                if antenna_char_1 == antenna_char_2 {
                    let vector = (location_a.0 - location_b.0, location_a.1 - location_b.1);

                    let potential_location_1 = (location_a.0 - vector.0, location_a.1 - vector.1);
                    let potential_location_2 = (location_a.0 + vector.0, location_a.1 + vector.1);
                    let potential_location_3 = (location_b.0 - vector.0, location_b.1 - vector.1);
                    let potential_location_4 = (location_b.0 + vector.0, location_b.1 + vector.1);

                    let potential_locations = [potential_location_1, potential_location_2, potential_location_3, potential_location_4];

                    for potential_location in potential_locations {
                        if potential_location != *location_a && potential_location != *location_b && potential_location.0 >= 0 && potential_location.0 < y_max && potential_location.1 >= 0 && potential_location.1 < x_max {
                            antinode_locations.insert(potential_location);
                        }
                    }
                }
            }
        }
    }

    let antinode_num = antinode_locations.len();
    println!("Final result is {antinode_num}");

    antinode_num
}

pub fn run_b(file_path: &str) -> usize {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let x_max: i32 = input_string[0].len() as i32;
    let y_max: i32 = input_string.len() as i32;

    let grid = generate_grid(input_string);
    // println!("{grid:?}");

    // Gather all antennas and their locations
    let mut antennas: HashMap<(i32, i32), char> = HashMap::new();
    let mut antenna_locations: Vec<(i32, i32)> = vec![];
    for y in 0..y_max {
        for x in 0..x_max {
            let character = grid.get(&(y, x)).unwrap();
            if *character != '.' {
                antennas.insert((y, x), *character);
                antenna_locations.push((y, x));
            }
        }
    }

    let mut antinode_locations = HashSet::new();

    // Check antenna pairs
    for location_a in &antenna_locations {
        for location_b in &antenna_locations {
            if location_a != location_b {
                let antenna_char_1 = antennas.get(&location_a);
                let antenna_char_2 = antennas.get(&location_b);

                if antenna_char_1 == antenna_char_2 {
                    let vector = (location_a.0 - location_b.0, location_a.1 - location_b.1);

                    let mut potential_location = *location_a;
                    'minus_loop: loop {
                        potential_location = (potential_location.0 - vector.0, potential_location.1 - vector.1);
                        if potential_location.0 >= 0 && potential_location.0 < y_max && potential_location.1 >= 0 && potential_location.1 < x_max {
                            antinode_locations.insert(potential_location);
                        } else {
                            break 'minus_loop;
                        }
                    }
                    let mut potential_location = *location_a;
                    'plus_loop:loop {
                        potential_location = (potential_location.0 + vector.0, potential_location.1 + vector.1);
                        if potential_location.0 >= 0 && potential_location.0 < y_max && potential_location.1 >= 0 && potential_location.1 < x_max {
                            antinode_locations.insert(potential_location);
                        } else {
                            break 'plus_loop;
                        }
                    }
                }
            }
        }
    }

    let antinode_num = antinode_locations.len();
    println!("Final result is {antinode_num}");

    antinode_num
}