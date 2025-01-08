use crate::utils::{read_into_vec_of_strings, generate_grid};

pub fn run_a(file_path: &str) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let x_max: i32 = input_string[0].len() as i32;
    let y_max: i32 = input_string.len() as i32;

    let grid = generate_grid(input_string);

    let possible_vectors: Vec<(i32, i32)> = vec![(0, 1),
                                    (0, -1),
                                    (1, 0),
                                    (-1, 0),
                                    (1, 1),
                                    (1, -1),
                                    (-1, 1),
                                    (-1, -1)];

    let mut all_words = vec![];

    for y in 0..y_max {
        for x in 0..x_max {
            let current_position = (y as i32, x as i32);
            if *grid.get(&current_position).unwrap() == 'X' {
                for vector in &possible_vectors {
                    let mut word_vector = vec![current_position];
                    let mut next_position = current_position;
                    for _ in 0..3 {
                        let next_y_position = next_position.0 + vector.0;
                        let next_x_position = next_position.1 + vector.1;
                        if next_y_position < y_max && next_y_position >= 0 && next_x_position < x_max && next_x_position >= 0 {
                            next_position = (next_y_position, next_x_position);
                            word_vector.push(next_position);
                        }
                    }
                    if word_vector.len() == 4 {
                        all_words.push(word_vector);
                    }

                }
            }
        }
    }

    let mut final_result = 0;

    for word in all_words {
        let char_0 = *grid.get(&word[0]).unwrap();
        let char_1 = *grid.get(&word[1]).unwrap();
        let char_2 = *grid.get(&word[2]).unwrap();
        let char_3 = *grid.get(&word[3]).unwrap();

        if char_0 == 'X' && char_1 == 'M' && char_2 == 'A' && char_3 == 'S' {
            final_result += 1;
        }
    }

    println!("Final result is {final_result}");
    final_result
}

pub fn run_b(file_path: &str) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let x_max: i32 = input_string[0].len() as i32;
    let y_max: i32 = input_string.len() as i32;

    let grid = generate_grid(input_string);

    let possible_vectors: Vec<(i32, i32)> = vec![
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1)
    ];

    let mut all_words = vec![];

    for y in 1..y_max-1 {
        for x in 1..x_max-1 {
            let current_position = (y as i32, x as i32);
            if *grid.get(&current_position).unwrap() == 'A' {
                let mut x_vector = vec![];
                for vector in &possible_vectors {
                    let next_y_position = current_position.0 + vector.0;
                    let next_x_position = current_position.1 + vector.1;
                    x_vector.push((next_y_position, next_x_position));
                }
                all_words.push(x_vector);
            }
        }
    }

    let mut final_result = 0;

    for word in all_words {
        let char_top_left = *grid.get(&word[0]).unwrap();
        let char_bottom_right = *grid.get(&word[1]).unwrap();
        let char_top_right = *grid.get(&word[2]).unwrap();
        let char_bottom_left = *grid.get(&word[3]).unwrap();

        let condition_left:bool =  (char_top_left == 'M' && char_bottom_right == 'S') || (char_top_left == 'S' && char_bottom_right == 'M');
        let condition_right:bool = (char_top_right == 'M' && char_bottom_left == 'S') || (char_top_right == 'S' && char_bottom_left == 'M');

        if condition_left && condition_right {
            final_result += 1;
        }
    }

    println!("Final result is {final_result}");
    final_result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = run_a("csv/day_4_test.txt");
        assert_eq!(18, result);
    }
    #[test]
    fn test_b() {
        let result = run_b("csv/day_4_test.txt");
        assert_eq!(9, result);
    }
}