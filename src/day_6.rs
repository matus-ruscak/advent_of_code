use crate::utils::{generate_grid, read_into_vec_of_strings};
use std::collections::HashMap;

pub fn run_a(file_path: &str) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let x_max: i32 = input_string[0].len() as i32;
    let y_max: i32 = input_string.len() as i32;

    let grid = generate_grid(input_string);

    let starting_position = get_starting_position(&grid);

    let vector_rotate_map: HashMap<(i32, i32), (i32, i32)> = initialize_vector_rotation_map();

    let mut current_position = starting_position;
    let mut current_vector: (i32, i32) = (-1, 0);
    let mut final_result: i32 = 1;

    let mut all_positions: Vec<(i32, i32)> = vec![];

    loop {
        all_positions.push(current_position);
        let next_position = (current_position.0 + current_vector.0, current_position.1 + current_vector.1);

        let next_character = grid.get(&next_position).unwrap();
        if *next_character == '.' && (next_position.0 == y_max-1 || next_position.0 == 0 || next_position.1 == x_max-1 || next_position.1 == 0) {
            final_result += 1;
            break;
        }
        if *next_character == '#' {
            current_vector = *vector_rotate_map.get(&current_vector).unwrap();
        } else {
            current_position = next_position;
            if !all_positions.contains(&current_position) {
                final_result += 1;
            }
        }
    }
    println!("Final result is {final_result}");

    final_result
}

pub fn run_b(file_path: &str) -> i32 {
    println!("Warning! Takes 25 minutes on a reasonable, although not extra powerful, computer.");
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let x_max: i32 = input_string[0].len() as i32;
    let y_max: i32 = input_string.len() as i32;

    let grid = generate_grid(input_string);

    let starting_position = get_starting_position(&grid);

    let vector_rotate_map: HashMap<(i32, i32), (i32, i32)> = initialize_vector_rotation_map();

    let mut number_of_loops: i32 = 0;

    for i in 0..y_max {
        for j in 0.. x_max {
            let mut grid_copy = grid.clone();
            let current_character_to_test = grid.get(&(i, j)).unwrap();
            // If there's a # already, we continue
            if *current_character_to_test == '#' {
                continue
            }
            // Else proceed with the loop checking
            grid_copy.insert((i, j), '#');

            let mut current_position = starting_position;
            let mut current_vector: (i32, i32) = (-1, 0);


            let mut all_positions_and_vectors: Vec<((i32, i32), (i32, i32))> = vec![];

            loop {
                if all_positions_and_vectors.contains(&(current_position, current_vector)) {
                    number_of_loops += 1;
                    break;
                };
                all_positions_and_vectors.push((current_position, current_vector));

                let next_position = (current_position.0 + current_vector.0, current_position.1 + current_vector.1);

                let next_character = grid_copy.get(&next_position).unwrap();
                if *next_character == '.' && (next_position.0 == y_max-1 || next_position.0 == 0 || next_position.1 == x_max-1 || next_position.1 == 0) {
                    break;
                }

                if *next_character == '#' {
                    current_vector = *vector_rotate_map.get(&current_vector).unwrap();
                } else {
                    current_position = next_position;
                }
            }
        }
    }
    println!("Final result is {number_of_loops}");
    number_of_loops
}

fn get_starting_position(grid: &HashMap<(i32, i32), char>) -> (i32, i32) {
    let value_to_find = '^';
    let starting_position = grid.iter().find_map(|(key, &val)| {
        if val == value_to_find {
            Some(key)
        } else {
            None
        }
    });
    let starting_position_unwrapped = *starting_position.unwrap();

    starting_position_unwrapped

}
fn initialize_vector_rotation_map() -> HashMap<(i32, i32), (i32, i32)> {
    let mut vector_rotate_map: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    vector_rotate_map.insert((-1, 0), (0, 1));
    vector_rotate_map.insert((0, 1), (1, 0));
    vector_rotate_map.insert((1, 0), (0, -1));
    vector_rotate_map.insert((0, -1), (-1, 0));

    vector_rotate_map
}