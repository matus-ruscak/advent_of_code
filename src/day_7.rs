use crate::utils::read_into_vec_of_strings;
use itertools::Itertools;

pub fn run_a(file_path: &str) -> i128 {
    run(file_path, &["MULTIPLY", "ADD"])
}

pub fn run_b(file_path: &str) -> i128 {
    run(file_path, &["MULTIPLY", "ADD", "CONCATENATE"])
}

fn run(file_path: &str, operations: &[&str]) -> i128 {
    let input_data = read_into_vec_of_strings(file_path);
    let result_formula_vec = preprocess_data(&input_data);

    let mut total_sum: i128 = 0;

    for (expected_result, numbers) in result_formula_vec {
        let math_combinations = generate_combinations(operations, numbers.len() - 1);

        if let Some(valid_result) = evaluate_combinations(&numbers, &math_combinations, expected_result, operations.contains(&"CONCATENATE")) {
            total_sum += valid_result;
        }
    }

    println!("Final result is {total_sum}");
    total_sum
}

fn preprocess_data(input_data: &[String]) -> Vec<(i128, Vec<i128>)> {
    input_data.iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let result: i128 = parts[0].parse().unwrap();
            let numbers: Vec<i128> = parts[1]
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            (result, numbers)
        })
        .collect()
}

fn generate_combinations<'a>(items: &'a [&'a str], n: usize) -> Vec<Vec<&'a str>> {
    std::iter::repeat(items)
        .take(n)
        .multi_cartesian_product()
        .map(|v| v.into_iter().copied().collect())
        .collect()
}

fn evaluate_combinations(
    numbers: &[i128],
    math_combinations: &[Vec<&str>],
    expected_result: i128,
    allow_concatenate: bool,
) -> Option<i128> {
    for math_combination in math_combinations {
        let mut current_result = numbers[0];
        for (i, &operation) in math_combination.iter().enumerate() {
            current_result = match operation {
                "ADD" => current_result + numbers[i + 1],
                "MULTIPLY" => current_result * numbers[i + 1],
                "CONCATENATE" if allow_concatenate => {
                    let concatenated = format!("{}{}", current_result, numbers[i + 1]);
                    concatenated.parse().unwrap_or_default()
                }
                _ => continue,
            };
        }

        if current_result == expected_result {
            return Some(current_result);
        }
    }
    None
}
