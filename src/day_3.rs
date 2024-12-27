use crate::utils::read_into_vec_of_strings;
use regex::Regex;

pub fn run_a(file_path: &str) -> i32 {
    let instructions = read_into_vec_of_strings(file_path);
    let mut result: i32 = 0;

    for instruction in instructions {
        let parsed_list: Vec<(i32, i32)> = extract_mul_tuples_simple(&instruction);
        for pair in parsed_list {
            result = result + (pair.0 * pair.1);
        }
    }
    println!("Final result is {result}");
    result
}

pub fn run_b(file_path: &str) -> i32 {
    let instructions = read_into_vec_of_strings(file_path);
    let mut result: i32 = 0;

    let mut switch: i32 = 1;

    for instruction in instructions {
        let parsed_list: Vec<String> = extract_mul_tuples_with_inst(&instruction);

        for item in parsed_list {
            if &item == "don't()" {
                switch = 0;
            } else if item == "do()" {
                switch = 1;
            } else {
                let re = Regex::new(r"mul\((-?\d+),\s*(-?\d+)\)").unwrap();

                // Apply the regex to find a match
                let result_tuple = re.captures(&item).and_then(|caps| {
                    let a: i32 = caps[1].parse().ok()?;
                    let b: i32 = caps[2].parse().ok()?;
                    Some((a, b))
                });

                result += switch * result_tuple.unwrap().0 * result_tuple.unwrap().1;
            }
        }
    }
    println!("Final result is {result}");
    result
}

fn extract_mul_tuples_simple(input: &str) -> Vec<(i32, i32)> {
    get_tuple_of_ints(input)
}

fn extract_mul_tuples_with_inst(input: &str) -> Vec<String> {
    let pattern = r"mul\((-?\d+),\s*(-?\d+)\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();

    // Collect matches into a vector of tuples
    re.captures_iter(input)
        .filter_map(|cap| {
            // Parse the matched groups into integers
            let a: String = cap[0].parse().ok()?;
            Some(a)
        })
        .collect()
}

fn get_tuple_of_ints(input: &str) -> Vec<(i32, i32)> {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            // Parse the matched groups into integers
            let a: i32 = cap[1].parse().ok()?;
            let b: i32 = cap[2].parse().ok()?;
            Some((a, b))
        })
        .collect()
}