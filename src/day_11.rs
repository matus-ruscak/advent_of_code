use std::collections::HashMap;
use crate::utils::{read_into_vec_of_strings};

pub fn run_a(file_path: &str, number_of_loops: i32) -> usize {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let mut stones: Vec<u64> = input_string[0]
        .split_whitespace() // Split the string by whitespace
        .filter_map(|s| s.parse::<u64>().ok()) // Parse each part as i128, ignoring invalid parts
        .collect();

    for _i in 0..number_of_loops {
        let result_stones = process_stone(&stones);
        stones = result_stones;
    }

    let final_length = stones.len();
    println!("Final result is {final_length}");

    final_length
}

pub fn run_b(file_path: &str, number_of_loops: usize) -> u64 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let stones: Vec<u64> = input_string[0]
        .split_whitespace() // Split the string by whitespace
        .filter_map(|s| s.parse::<u64>().ok()) // Parse each part as i128, ignoring invalid parts
        .collect();

    let mut final_length: u64 = 0;

    let mut stone_cache: HashMap<(u64, usize), u64> = HashMap::new();

    for stone in stones {
        final_length += process_stone_recursively(stone, number_of_loops, &mut stone_cache);
    }

    println!("Final result is {final_length}");
    final_length
}

fn process_stone_recursively(stone: u64,
                             iterations: usize,
                             stone_cache: &mut HashMap<(u64, usize), u64>) -> u64 {

    if iterations == 0 {
        return 1;
    }

    let stone_cache_key = &(stone, iterations);
    if stone_cache.contains_key(stone_cache_key) {
        return *stone_cache.get(&stone_cache_key).unwrap();
    }

    let count = if stone == 0 {
        process_stone_recursively(1, iterations - 1, stone_cache)
    } else if stone.to_string().len() % 2 == 0 {
        let first_number = split_number(stone).0;
        let second_number = split_number(stone).1;
        process_stone_recursively(first_number, iterations - 1, stone_cache)
            + process_stone_recursively(second_number, iterations - 1, stone_cache)
    } else {
        process_stone_recursively(stone * 2024, iterations - 1, stone_cache)
    };

    stone_cache.insert(*stone_cache_key, count);

    count
}

fn process_stone(stones: &Vec<u64>) -> Vec<u64> {
    let mut processed_stones: Vec<u64> = vec![];

    for i in 0..stones.len() {
        let current_stone = stones[i];

        if current_stone == 0 {
            processed_stones.push(1);
        } else if current_stone.to_string().len() % 2 == 0 {
            let first_number = split_number(current_stone).0;
            let second_number = split_number(current_stone).1;
            processed_stones.push(first_number);
            processed_stones.push(second_number);
        } else {
            processed_stones.push(current_stone * 2024);
        }

    }
    processed_stones
}

fn split_number(n: u64) -> (u64, u64) {
    let num_digits = n.to_string().len() as u32; // Calculate the total number of digits
    let split_position = (num_digits/ 2) as u32;

    // Ensure split_position is valid
    if split_position == 0 || split_position >= num_digits {
        panic!("Invalid split ratio; must result in non-zero parts.");
    }

    let divisor = 10u64.pow(num_digits - split_position); // Dynamically calculate divisor
    let left_part = n / divisor; // Left part
    let right_part = n % divisor; // Right part
    (left_part, right_part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = run_a("csv/day_11_test.txt", 25);
        assert_eq!(55312, result);
    }
}