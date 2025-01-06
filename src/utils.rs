use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_into_two_int_vectors_as_cols(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut vec_1: Vec<i32> = vec![];
    let mut vec_2: Vec<i32> = vec![];

    for line in reader.lines() {
        let line_content = line.unwrap();

        let line_split: Vec<&str> = line_content.split("   ").collect();
        let item_1: i32 = line_split[0].parse().unwrap();
        let item_2: i32 = line_split[1].parse().unwrap();

        vec_1.push(item_1);
        vec_2.push(item_2);
    }

    (vec_1, vec_2)
}

pub fn read_into_vec_int_as_lines(file_path: &str) -> Vec<Vec<i32>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut result: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line_content = line.unwrap();

        let line_split: Vec<i32> = line_content
            .split_whitespace()        // Split the string by whitespace
            .map(|s| s.parse().unwrap()) // Parse each substring as i32
            .collect();


        result.push(line_split);
    }
        result
}

pub fn read_into_vec_of_strings(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut result: Vec<String> = vec![];

    for line in reader.lines() {
        let line_content = line.unwrap();
        result.push(line_content);
    }

    result
}

pub fn generate_grid(input_string: Vec<String>) -> HashMap<(i32, i32), char> {
    let y_max = input_string.len();
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    // first digit is the y-axis, second is the x-axis
    for i in 0..y_max {
        let mut x_position: i32 = 0;
        for j in input_string[i].chars() {
            grid.insert((i as i32, x_position), j);
            x_position += 1;
        }
    }
    grid
}

pub fn generate_grid_digits(input_string: Vec<String>) -> HashMap<(i32, i32), i32> {
    let y_max = input_string.len();
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    // first digit is the y-axis, second is the x-axis
    for i in 0..y_max {
        let mut x_position: i32 = 0;
        for j in input_string[i].chars() {
            grid.insert((i as i32, x_position), j.to_digit(10).unwrap() as i32);
            x_position += 1;
        }
    }
    grid
}