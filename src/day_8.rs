use std::collections::{HashSet, HashMap};
use crate::utils::{read_into_vec_of_strings, generate_grid};

pub fn run_a(file_path: &str) -> usize {
    run(file_path, false)
}

pub fn run_b(file_path: &str) -> usize {
    run(file_path, true)
}

fn run(file_path: &str, extended_mode: bool) -> usize {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);
    let (x_max, y_max) = (input_string[0].len() as i32, input_string.len() as i32);

    let grid = generate_grid(input_string);
    let (antennas, antenna_locations) = gather_antennas(&grid, x_max, y_max);

    let antinode_locations = if extended_mode {
        find_extended_antinode_locations(&antennas, &antenna_locations, x_max, y_max)
    } else {
        find_basic_antinode_locations(&antennas, &antenna_locations, x_max, y_max)
    };

    let antinode_num = antinode_locations.len();
    println!("Final result is {antinode_num}");
    antinode_num
}

fn gather_antennas(
    grid: &HashMap<(i32, i32), char>,
    x_max: i32,
    y_max: i32,
) -> (HashMap<(i32, i32), char>, Vec<(i32, i32)>) {
    let mut antennas = HashMap::new();
    let mut antenna_locations = vec![];

    for y in 0..y_max {
        for x in 0..x_max {
            if let Some(&character) = grid.get(&(y, x)) {
                if character != '.' {
                    antennas.insert((y, x), character);
                    antenna_locations.push((y, x));
                }
            }
        }
    }

    (antennas, antenna_locations)
}

fn find_basic_antinode_locations(
    antennas: &HashMap<(i32, i32), char>,
    antenna_locations: &[(i32, i32)],
    x_max: i32,
    y_max: i32,
) -> HashSet<(i32, i32)> {
    let mut antinode_locations = HashSet::new();

    for &location_a in antenna_locations {
        for &location_b in antenna_locations {
            if location_a != location_b && antennas[&location_a] == antennas[&location_b] {
                let vector = (location_a.0 - location_b.0, location_a.1 - location_b.1);
                let potential_locations = [
                    (location_a.0 - vector.0, location_a.1 - vector.1),
                    (location_a.0 + vector.0, location_a.1 + vector.1),
                    (location_b.0 - vector.0, location_b.1 - vector.1),
                    (location_b.0 + vector.0, location_b.1 + vector.1),
                ];

                for &potential_location in &potential_locations {
                    if is_within_bounds(potential_location, x_max, y_max)
                        && potential_location != location_a
                        && potential_location != location_b
                    {
                        antinode_locations.insert(potential_location);
                    }
                }
            }
        }
    }

    antinode_locations
}

fn find_extended_antinode_locations(
    antennas: &HashMap<(i32, i32), char>,
    antenna_locations: &[(i32, i32)],
    x_max: i32,
    y_max: i32,
) -> HashSet<(i32, i32)> {
    let mut antinode_locations = HashSet::new();

    for &location_a in antenna_locations {
        for &location_b in antenna_locations {
            if location_a != location_b && antennas[&location_a] == antennas[&location_b] {
                let vector = (location_a.0 - location_b.0, location_a.1 - location_b.1);

                extend_locations(location_a, vector, x_max, y_max, &mut antinode_locations);
                extend_locations(location_b, vector, x_max, y_max, &mut antinode_locations);
            }
        }
    }

    antinode_locations
}

fn extend_locations(
    start: (i32, i32),
    vector: (i32, i32),
    x_max: i32,
    y_max: i32,
    antinode_locations: &mut HashSet<(i32, i32)>,
) {
    let mut current = start;

    // Extend in the negative direction of the vector
    while let Some(new_location) = extend_step(current, (-vector.0, -vector.1), x_max, y_max) {
        antinode_locations.insert(new_location);
        current = new_location;
    }

    current = start;

    // Extend in the positive direction of the vector
    while let Some(new_location) = extend_step(current, vector, x_max, y_max) {
        antinode_locations.insert(new_location);
        current = new_location;
    }
}

fn extend_step(
    current: (i32, i32),
    vector: (i32, i32),
    x_max: i32,
    y_max: i32,
) -> Option<(i32, i32)> {
    let next = (current.0 + vector.0, current.1 + vector.1);
    if is_within_bounds(next, x_max, y_max) {
        Some(next)
    } else {
        None
    }
}

fn is_within_bounds(location: (i32, i32), x_max: i32, y_max: i32) -> bool {
    location.0 >= 0 && location.0 < y_max && location.1 >= 0 && location.1 < x_max
}
