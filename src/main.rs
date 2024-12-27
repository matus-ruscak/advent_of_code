fn main() {
    assert_eq!(11, advent_of_code::day_1::run_a("csv/day_1_test.txt"));
    advent_of_code::day_1::run_a("csv/day_1_actual.txt");

    assert_eq!(31, advent_of_code::day_1::run_b("csv/day_1_test.txt"));
    advent_of_code::day_1::run_b("csv/day_1_actual.txt");

    assert_eq!(2, advent_of_code::day_2::run_a("csv/day_2_test.txt"));
    advent_of_code::day_2::run_a("csv/day_2_actual.txt");

    assert_eq!(4, advent_of_code::day_2::run_b("csv/day_2_test.txt"));
    advent_of_code::day_2::run_b("csv/day_2_actual.txt");
}
