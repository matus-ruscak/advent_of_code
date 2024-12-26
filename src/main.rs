fn main() {
    assert_eq!(11, advent_of_code::day_1::day_1_a("csv/day_1_test.txt"));
    advent_of_code::day_1::day_1_a("csv/day_1_actual.txt");

    assert_eq!(31, advent_of_code::day_1::day_1_b("csv/day_1_test.txt"));
    advent_of_code::day_1::day_1_b("csv/day_1_actual.txt");
}
