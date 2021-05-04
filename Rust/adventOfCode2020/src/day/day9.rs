
use super::day1;

pub struct Day9 {}

impl super::Day for Day9{
    type PuzzleInput = Vec<u32>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .map(|line| line.parse::<u32>())
            .filter_map(Result::ok)
            .collect()
    }

    fn solve_part1(&self, numbers: Self::PuzzleInput) -> std::string::String {
        let first_item_failing_validation = first_invalid_entry(&numbers, 0, 25);
        match first_item_failing_validation {
            Some(item) => item.to_string(),
            None => String::from("No invalid entry found!"),
        }
    }

    fn solve_part2(&self, numbers: Self::PuzzleInput) -> std::string::String {
        let maybe_first_item_failing_validation = first_invalid_entry(&numbers, 0, 25);
        if maybe_first_item_failing_validation.is_none(){
            return String::from("No invalid entry found!");
        }
        let first_item_failing_validation = maybe_first_item_failing_validation.unwrap();
        let maybe_first_range_summing_to_invalid_item = first_range_with_sum(&numbers, &first_item_failing_validation, 0);
        if maybe_first_range_summing_to_invalid_item.is_none(){
            return format!("No range summing to {} found!", first_item_failing_validation);
        }
        let (lower_index, high_index) = maybe_first_range_summing_to_invalid_item.unwrap();
        let range = &numbers[lower_index..high_index];
        let low_value = range.iter().min().unwrap();
        let high_value = range.iter().max().unwrap();
        let result = low_value + high_value;
        result.to_string()
    }
}

fn first_invalid_entry(numbers: &[u32], start_index: usize, preamble_length: usize) -> Option<u32>{
    if preamble_length == 0 {
        return None;
    }
    for comparison_window in numbers[start_index..].windows(preamble_length + 1){
        let current_value = &comparison_window[preamble_length];
        let comparison_range = &comparison_window[..preamble_length];
        if !is_valid_entry(current_value, comparison_range){
            return Some(*current_value);
        }
    }
    None
}

fn is_valid_entry(comparison_value: &u32, comparison_range: &[u32]) -> bool{
    let summing_pair = day1::find_summing_pair(comparison_range, comparison_value);
    summing_pair.is_some()
}

fn first_range_with_sum(numbers: &[u32], desired_sum: &u32, start_index: usize) -> Option<(usize, usize)>{
    let mut low_index = start_index;
    let mut high_index = start_index + 1;

    if high_index >= numbers.len(){
        return None;
    }

    let mut current_sum = numbers[low_index] + numbers[high_index];

    loop{
        match current_sum.cmp(desired_sum){
            std::cmp::Ordering::Equal => return Some((low_index, high_index)),
            std::cmp::Ordering::Less => {
                high_index += 1;
                if high_index >= numbers.len(){
                    return None;
                }
                current_sum += numbers[high_index];
            },
            std::cmp::Ordering::Greater => {
                current_sum -= numbers[low_index];
                low_index += 1;
            },
        }
        if low_index == high_index{
            high_index += 1;
            if high_index >= numbers.len(){
                return None;
            }
            current_sum += numbers[high_index];
        }
    }
}





#[cfg(test)]
mod day9_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::Day<PuzzleInput = Vec<u32>>> = Box::new(Day9{});
        let problem_input = example_input();
        let parsed_problem_input = day.parse_input(problem_input);
        let expected_result = 127;
        let actual_result = first_invalid_entry(&parsed_problem_input, 0, 5).unwrap();
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::Day<PuzzleInput = Vec<u32>>> = Box::new(Day9{});
        let problem_input = example_input();
        let parsed_problem_input = day.parse_input(problem_input);
        let (lower_bound, upper_bound) = first_range_with_sum(&parsed_problem_input, &127u32, 0).unwrap();
        let expected_upper_bound = 2;
        let expected_lower_bound = 5;
        assert_eq!(lower_bound, expected_upper_bound);
        assert_eq!(upper_bound, expected_lower_bound);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day9{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 9, part: 1}).unwrap();
        let expected_result = String::from("1721308972");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day9{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 9, part: 2}).unwrap();
        let expected_result = String::from("209694133");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}