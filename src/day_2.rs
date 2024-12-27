use crate::utils::read_into_vec_int_as_lines;

pub fn run_a(file_path: &str) -> i32 {
    let list_of_reports: Vec<Vec<i32>> = read_into_vec_int_as_lines(file_path);

    let mut safe_report_count: i32 = 0;

    for report in list_of_reports {
        let mut diff_report: Vec<i32> = vec![];

        for i in 0..report.len() - 1 {
            let diff = report[i + 1] - report[i];
            diff_report.push(diff);
        }
        let is_safe: bool = full_check(&diff_report);

        if is_safe {
            safe_report_count += 1;
        }
    }
    println!("Final result is {safe_report_count}");
    safe_report_count
}

pub fn run_b(file_path: &str) -> i32 {
    let list_of_reports: Vec<Vec<i32>> = read_into_vec_int_as_lines(file_path);

    let mut safe_report_count: i32 = 0;

    for report in list_of_reports {
        let mut diff_report: Vec<i32> = vec![];

        for i in 0..report.len() - 1 {
            let diff = report[i + 1] - report[i];
            diff_report.push(diff);
        }
        let is_safe: bool = full_check(&diff_report);

        if is_safe {
            safe_report_count += 1;
        } else {
            safe_report_count = gracious_increase(report, safe_report_count);
        }
    }
    println!("Final result is {safe_report_count}");
    safe_report_count
}

fn full_check(diff_report: &Vec<i32>) -> bool {
    let is_single_direction = direction_check(&diff_report);
    let is_step_in_range = step_in_range_check(&diff_report);

    is_single_direction && is_step_in_range
}

fn direction_check(diff_report: &Vec<i32>) -> bool {
    let all_positive = diff_report.iter().all(|&x| x > 0);
    let all_negative = diff_report.iter().all(|&x| x < 0);

    all_positive || all_negative
}

fn step_in_range_check(diff_report: &Vec<i32>) -> bool {
    let valid_values = vec![1, 2, 3];
    let step_in_range = diff_report.iter().all(|&x| valid_values.contains(&x.abs()));

    step_in_range
}

fn gracious_increase(report: Vec<i32>, mut safe_report_count: i32) -> i32 {
    for j in 0..report.len() {
        let mut full_report_clone = report.clone();
        full_report_clone.remove(j);
        let mut diff_report: Vec<i32> = vec![];

        for i in 0..full_report_clone.len() - 1 {
            let diff = full_report_clone[i + 1] - full_report_clone[i];
            diff_report.push(diff);
        }
        let is_safe: bool = full_check(&diff_report);
        if is_safe {
            safe_report_count += 1;
            break;
        }
    }
    safe_report_count
}
