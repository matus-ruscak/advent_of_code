use std::collections::HashMap;
use crate::utils::read_into_two_int_vectors_as_cols;

pub fn run_a(file_path: &str) -> i32 {
    let (mut vec_1, mut vec_2) = read_into_two_int_vectors_as_cols(file_path);
    vec_1.sort();
    vec_2.sort();

    let mut final_result: i32 = 0;

    for i in 0..vec_1.len() {
        final_result += (vec_1[i] - vec_2[i]).abs();
    }

    println!("Final_result is: {final_result}");

    final_result
}

pub fn run_b(file_path: &str) -> i32 {
    let (mut vec_1, mut vec_2) = read_into_two_int_vectors_as_cols(file_path);

    vec_1.sort();
    vec_2.sort();

    let mut right_table_map: HashMap<i32, i32> = HashMap::new();

    for &num in &vec_2 {
        *right_table_map.entry(num).or_insert(0) += 1;
    }

    let mut final_result: i32 = 0;

    for left_item in vec_1.iter() {
        match right_table_map.get(left_item) {
            Some(value) => final_result += left_item * value,
            None => {},
        }
    }

    println!("Final result is {final_result}");

    final_result
}