use crate::utils::{generate_grid, read_into_vec_of_strings};
use std::collections::{HashMap, HashSet, VecDeque};

type Position = (i32, i32);

pub fn run_a(file_path: &str, with_discount: bool) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let grid = generate_grid(input_string);

    let groups = find_connected_groups(&grid);

    let mut final_price: i32 = 0;

    for group in groups {
        let price_for_group = calculate_price_for_group(group, with_discount);

        final_price += price_for_group;
    }
    println!("FINAL RESULT {final_price}");
    final_price
}


/// Finds all groups of connected characters in the grid
fn find_connected_groups(grid: &HashMap<Position, char>) -> Vec<HashSet<Position>> {
    let mut visited = HashSet::new();
    let mut groups = Vec::new();

    for &pos in grid.keys() {
        if !visited.contains(&pos) {
            let group = bfs_collect_group(grid, pos, &mut visited);
            groups.push(group);
        }
    }

    groups
}

/// Performs a BFS to collect all connected positions of the same character
fn bfs_collect_group(
    grid: &HashMap<Position, char>,
    start: Position,
    visited: &mut HashSet<Position>,
) -> HashSet<Position> {
    let mut queue = VecDeque::new();
    let mut group = HashSet::new();

    if let Some(&char_to_find) = grid.get(&start) {

        queue.push_back(start);
        visited.insert(start);

        while let Some((y, x)) = queue.pop_front() {
            group.insert((y, x));

            // Add valid neighbors (horizontal and vertical)
            for (next_y, next_x) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                let neighbor = (next_y, next_x);
                if grid.get(&neighbor) == Some(&char_to_find) && !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
    }

    group
}


fn calculate_price_for_group(group: HashSet<Position>, with_discount: bool) -> i32 {
    let mut fence_pieces: i32 = 0;
    let mut x_positions: Vec<i32> = vec![];
    let mut y_positions: Vec<i32> = vec![];
    for position in &group {
        y_positions.push(position.0);
        x_positions.push(position.1);
    }
    let area = group.len() as i32;

    // without discount - each piece counts
    if !with_discount {
        for position in &group {
            let y = position.0;
            let x = position.1;

            for (next_y, next_x) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                if !group.contains(&(next_y, next_x)) {
                    fence_pieces += 1;
                }
            }
        }
    }

    // with discount - each side counts as 1
    else {
        // fence_positions: [((y, x), horizontal=0,vertical=1), ...]
        let mut horizontal_fence_positions: Vec<(i32, i32)> = vec![];
        let mut vertical_fence_positions: Vec<(i32, i32)> = vec![];
        for position in &group {
            let y = position.0;
            let x = position.1;

            for (next_y, next_x) in [(y - 1, x), (y + 1, x)] {
                if !group.contains(&(next_y, next_x)) {
                    horizontal_fence_positions.push((next_y, next_x));
                }
            }
            for (next_y, next_x) in [(y, x - 1), (y, x + 1)] {
                if !group.contains(&(next_y, next_x)) {
                    vertical_fence_positions.push((next_y, next_x));
                }
            }
        }

        fence_pieces = count_rectangle_lines(horizontal_fence_positions,
                                             vertical_fence_positions);

    }

    let result_for_group = fence_pieces * area;

    result_for_group
}

fn count_rectangle_lines(horizontal_fence_positions: Vec<(i32, i32)>,
                         vertical_fence_positions: Vec<(i32, i32)>) -> i32 {
    let mut horizontal_y_points: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut vertical_x_points: HashMap<i32, Vec<i32>> = HashMap::new();

    for point in &horizontal_fence_positions {
        let point_y = point.0;
        let point_x = point.1;

        if horizontal_y_points.contains_key(&point_y) {
            let mut curr_vec = horizontal_y_points.get(&point_y).unwrap().clone();
            curr_vec.push(point_x);
            horizontal_y_points.insert(point_y, curr_vec);
        } else {
            let mut curr_vec: Vec<i32> = vec![];
            curr_vec.push(point_x);
            horizontal_y_points.insert(point_y, curr_vec);
        }

    }

    for point in &vertical_fence_positions {
        let point_y = point.0;
        let point_x = point.1;

        if vertical_x_points.contains_key(&point_x) {
            let mut curr_vec = vertical_x_points.get(&point_x).unwrap().clone();
            curr_vec.push(point_y);
            vertical_x_points.insert(point_x, curr_vec);
        } else {
            let mut curr_vec: Vec<i32> = vec![];
            curr_vec.push(point_y);
            vertical_x_points.insert(point_x, curr_vec);
        }
    }

    // Sort them
    for (_key, vec) in &mut horizontal_y_points {
        vec.sort();
    }

    for (_key, vec) in &mut vertical_x_points {
        vec.sort();
    }

    // Now count the actual lines
    // if length is 1 -> it's a line
    // if all items inside are ordered -> it's a line
    // otherwise it's not
    let mut fence_count: i32 = 0;

    for (_, x_points) in horizontal_y_points {
        if x_points.len() == 1 {
            fence_count += 1;
        } else if is_sequential(&x_points) {
            fence_count += 1;
        } else if all_duplicates(&x_points){
            fence_count += 2;
        } else {
            'inner_loop: for i in 0..x_points.len() {
                if i == x_points.len()-1 {
                    fence_count += 1;
                }
                else if x_points[i+1] - x_points[i] == 1 {
                    continue 'inner_loop;
                } else {
                    fence_count += 1;
                }
            }
        }
    }

    for (_, y_points) in vertical_x_points {
            if y_points.len() == 1 {
                fence_count += 1;
            } else if is_sequential(&y_points) {
                fence_count += 1;
            } else if all_duplicates(&y_points){
                fence_count += 2;
            } else {
                'inner_loop: for i in 0..y_points.len() {
                    if i == y_points.len()-1 {
                        fence_count += 1;
                    }
                    else if y_points[i+1] - y_points[i] == 1 {
                        continue 'inner_loop;
                    } else {
                        fence_count += 1;
                    }
                }
            }
        }

    fence_count
}

fn is_sequential(vec: &Vec<i32>) -> bool {
    let mut sorted_vec = vec.clone(); // Clone the vector to avoid modifying the original
    sorted_vec.sort();               // Sort the vector

    for i in 1..sorted_vec.len() {
        if sorted_vec[i] - sorted_vec[i - 1] != 1 {
            return false; // Return false if the difference is not 1
        }
    }

    true // Return true if all numbers increase by 1
}

fn all_duplicates(vec: &[i32]) -> bool {
    let mut count_map = HashMap::new();

    // Count occurrences of each element
    for &num in vec {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Check if all elements have exactly 2 occurrences
    count_map.values().all(|&count| count == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_1() {
        let result = run_a("csv/day_12_test_1.txt", false);
        assert_eq!(140, result);
    }

    #[test]
    fn test_a_2() {
        let result = run_a("csv/day_12_test_2.txt", false);
        assert_eq!(772, result);
    }

    #[test]
    fn test_a_3() {
        let result = run_a("csv/day_12_test_3.txt", false);
        assert_eq!(1930, result);
    }

    #[test]
    fn test_b_1() {
        let result = run_a("csv/day_12_test_1.txt", true);
        assert_eq!(80, result);
    }

    #[test]
    fn test_b_2() {
        let result = run_a("csv/day_12_test_4.txt", true);
        assert_eq!(236, result);
    }
    #[test]
    fn test_b_3() {
        let result = run_a("csv/day_12_test_5.txt", true);
        assert_eq!(368, result);
    }
    #[test]
    fn test_b_4() {
        let result = run_a("csv/day_12_test_3.txt", true);
        assert_eq!(1206, result);
    }
}