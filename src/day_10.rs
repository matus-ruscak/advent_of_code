use crate::utils::{generate_grid_digits, read_into_vec_of_strings};
use std::collections::{HashMap, HashSet};

pub fn run(file_path: &str, part: &str) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let grid = generate_grid_digits(input_string);

    let mut starting_points: Vec<(i32, i32)> = vec![];
    for (position, height) in &grid {
        if *height == 0 {
            starting_points.push(*position);
        }
    }
    let mut final_sum: i32 = 0;

    for starting_point in starting_points {
        let current_position = starting_point;
        let current_height = *grid.get(&current_position).unwrap();
        let mut visited = HashSet::new();
        visited.insert(current_position);

        let mut repeat_flag: bool = true;
        if part == "part_1" {
            repeat_flag = false;
        }

        let starting_point_sum = find_paths(
            &grid,
            current_position,
            current_height,
            &mut visited,
            repeat_flag
        );

        final_sum += starting_point_sum;

    }

    println!("Final result is {final_sum}");
    final_sum
}

fn find_paths(
    grid: &HashMap<(i32, i32), i32>,
    current_position: (i32, i32),
    current_height: i32,
    visited: &mut HashSet<(i32, i32)>,
    repeat_flag: bool
) -> i32 {
    if current_height == 9 {
        return 1
    }

    let mut count = 0;
    let possible_vectors: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dx, dy) in possible_vectors.iter() {
        let next_pos = (current_position.0 + dx, current_position.1 + dy);
        if let Some(&next_digit) = grid.get(&next_pos) {
            // Check if the next digit is exactly 1 higher and if the position is not visited.
            if next_digit == current_height + 1 && !visited.contains(&next_pos) {
                if !repeat_flag {
                    visited.insert(next_pos);
                }
                count += find_paths(grid, next_pos, next_digit, visited, repeat_flag);
            }
        }
    }

    count
}