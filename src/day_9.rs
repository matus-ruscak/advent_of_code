use crate::utils::read_into_vec_of_strings;

// DISCLAIMER - couldn't solve by myself, got inspired by BurgundyDev's solution

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

pub fn run_b(file_path: &str) -> u64 {
    let input: &String = &read_into_vec_of_strings(file_path)[0];

    let mut filesystem: Vec<Option<u64>> = vec![];
    let mut curr_index: u64 = 0;

    let mut size: Vec<usize> = vec![0; input.chars().count()];
    let mut loc: Vec<usize> = vec![0; input.chars().count()];

    for (di, digit) in input.chars().enumerate()
    {
        if di % 2 == 0
        {

            loc[curr_index as usize] = filesystem.len();
            size[curr_index as usize] = digit.to_digit(10).unwrap() as usize;
            for _n in 0..digit.to_digit(10).unwrap()
            {
                filesystem.push(Some(curr_index));
            }
            curr_index+=1;
        } else {
            for _n in 0..digit.to_digit(10).unwrap()
            {
                filesystem.push(None);
            }
        }
    }
    // println!("{}", uncompressed);

    let mut big: usize = 0;

    while size[big] > 0 {
        big += 1
    }
    big -= 1;

    let mut to_move = big;
    while to_move > 0
    {
        let mut free_space: usize = 0;
        let mut first_free: usize = 0;

        while first_free < loc[to_move] && free_space < size[to_move]
        {
            first_free = first_free + free_space;
            free_space = 0;
            while filesystem[first_free] != None
            {
                first_free += 1
            }
            while first_free + free_space < filesystem.len() && filesystem[first_free + free_space] == None {
                free_space += 1
            }
        }

        if first_free >= loc[to_move]
        {
            to_move -= 1;
            continue;
        }

        for idx in first_free..first_free+size[to_move]
        {
            filesystem[idx] = Some(to_move as u64);
        }
        for idx in loc[to_move]..loc[to_move]+size[to_move]
        {
            filesystem[idx] = None;
        }
        to_move -= 1;
    }

    let mut result: u64 = 0;
    for (bi, block) in filesystem.iter().enumerate()
    {
        if block.is_some()
        {
            result += bi as u64 * block.unwrap();
        } else {
            continue;
        }
    }
    println!("Final result is {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = run_a("csv/day_9_test.txt");
        assert_eq!(1928, result);
    }
    #[test]
    fn test_b() {
        let result = run_b("csv/day_9_test.txt");
        assert_eq!(2858, result);
    }
}