use crate::utils::read_into_vec_of_strings;

pub fn run_a(file_path: &str) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);

    let (rules, pages_to_be_produced) = prepare_rules_pages(input_string);

    let mut final_result: i32 = 0;

    'page_loop: for page in pages_to_be_produced {
        for rule in &rules {
            if page.contains(&rule.0) && page.contains(&rule.1) {
                let position_0 = page.iter().position(|s| s == &rule.0).unwrap();
                let position_1 = page.iter().position(|s| s == &rule.1).unwrap();

                if position_0 > position_1 {
                    continue 'page_loop;
                }

            }
        }
        // if it didn't break, add the middle item to the sum
        let middle_index = page.len() / 2;
        let middle_value = page.get(middle_index).copied().unwrap();
        final_result += middle_value;
    }

    println!("Final result is {final_result}");
    final_result
}

pub fn run_b(file_path: &str) -> i32 {
    let input_string: Vec<String> = read_into_vec_of_strings(file_path);
    let (rules, pages_to_be_produced) = prepare_rules_pages(input_string);
    let mut final_result: i32 = 0;

    for page in pages_to_be_produced {

        // Only reorder if it's not OK
        if !page_order_check(&page, &rules) {
            let mut updated_page = page.clone();

            // Swap the elements until the order is correct
            while !page_order_check(&updated_page, &rules) {
                for rule in &rules {
                    if updated_page.contains(&rule.0) && updated_page.contains(&rule.1) {
                        let position_0 = updated_page.iter().position(|s| s == &rule.0).unwrap();
                        let position_1 = updated_page.iter().position(|s| s == &rule.1).unwrap();
                        if position_0 > position_1 {
                            updated_page.swap(position_1, position_0);
                        }
                    }
                }
            }
            let middle_index = updated_page.len() / 2;
            let middle_value = updated_page.get(middle_index).copied().unwrap();
            final_result += middle_value;
        }
    }

    println!("Final result is {final_result}");
    final_result

}


fn prepare_rules_pages(input_string: Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let separator_pos = input_string.iter().position(|s| s.is_empty()).unwrap();
    let rules = input_string[..separator_pos].to_vec();
    let pages_to_be_produced = input_string[separator_pos + 1..].to_vec();

    // Transform the rules into tuples
    let rules: Vec<(i32, i32)> = rules.into_iter()
        .filter_map(|item| {
            let parts: Vec<&str> = item.split('|').collect();
            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    return Some((a, b));
                }
            }
            None
        })
        .collect();

    let pages_to_be_produced: Vec<Vec<i32>> = pages_to_be_produced.into_iter()
        .map(|item| {
            item.split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, pages_to_be_produced)
}

fn page_order_check(page: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut is_correct_order = true;

    for rule in rules {
        if page.contains(&rule.0) && page.contains(&rule.1) {
            let position_0 = page.iter().position(|s| s == &rule.0).unwrap();
            let position_1 = page.iter().position(|s| s == &rule.1).unwrap();
            if position_0 > position_1 {
                is_correct_order = false;
                break;
            }
        }
    }
    is_correct_order
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = run_a("csv/day_5_test.txt");
        assert_eq!(143, result);
    }
    #[test]
    fn test_b() {
        let result = run_b("csv/day_5_test.txt");
        assert_eq!(123, result);
    }
}