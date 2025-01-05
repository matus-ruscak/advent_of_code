use crate::utils::read_into_vec_of_strings;

pub fn run_a(file_path: &str) -> i64 {
    let input_string: &String = &read_into_vec_of_strings(file_path)[0];

    let mut filesystem: Vec<Option<i64>> = vec![];

    for i in 0..input_string.len() {
        let char = input_string.chars().nth(i).unwrap();
        let digit: i64 = char.to_digit(10).unwrap() as i64;
        if i % 2 == 0 {
            for _ in 0..digit {
                filesystem.push(Some((i/2) as i64));
            }
        } else {
            for _ in 0..digit {
                filesystem.push(None);
            }
        }
    }

    let mut first_free: usize = 0;
    while filesystem[first_free] != None {
        first_free += 1;
    }

    let mut last_populated: usize = filesystem.len() - 1;

    while filesystem[last_populated] == None {
        last_populated -= 1;
    }

    while first_free < last_populated {
        filesystem[first_free] = filesystem[last_populated];
        filesystem[last_populated] = None;

        while filesystem[first_free] != None {
            first_free += 1;
        }
        while filesystem[last_populated] == None {
            last_populated -= 1;
        }
    }

    let mut final_number = 0;

    for i in 0..filesystem.len() {
        if filesystem[i].is_some() {
            final_number += i as i64 * filesystem[i].unwrap();
        }
    }

    println!("Final result is {final_number}");
    final_number
}