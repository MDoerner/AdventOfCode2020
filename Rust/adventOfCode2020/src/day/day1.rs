pub struct Day1 {}

impl super::Day for Day1{
    type PuzzleInput = Vec<i32>;
    fn parse_input(&self, input: String) -> Vec<i32> {
        input.lines()
            .map(|line| line.parse::<i32>())
            .filter_map(Result::ok)
            .collect::<Vec<i32>>()
    }

    fn solve_part1(&self, input: Vec<i32>) -> String {
        let summing_pair: Option<(i32, i32)> = find_summing_pair(input, 2020);
        match summing_pair{
            Some((l,h)) => (l * h).to_string(),
            None => String::from("No matching pair found.")
        }
    }

    fn solve_part2(&self, input: Vec<i32>) -> String {
        let summing_triple: Option<(i32, i32, i32)> = find_summing_triple(input, 2020);
        match summing_triple{
            Some((l, m, h)) => (l * m * h).to_string(),
            None => String::from("No matching triple found.")
        }
    }
}

fn find_summing_pair(candidates: Vec<i32>, desired_sum: i32) -> Option<(i32, i32)>{
    let mut sorted_numbers: Vec<i32> = candidates;
    sorted_numbers.sort_unstable();
    find_summing_pair_in_sorted(&sorted_numbers, desired_sum, 0)
}

pub fn find_summing_pair_in_sorted(sorted_candidates: &[i32], desired_sum: i32, lowest_index: usize) -> Option<(i32, i32)>{
    if sorted_candidates.len() < 2 {
        return None;
    }

    let mut lower_index: usize = lowest_index;
    let mut higher_index: usize = sorted_candidates.len() - 1;
    let mut current_lower_value: i32 = sorted_candidates[lower_index];
    let mut current_higher_value: i32 = sorted_candidates[higher_index];
    while lower_index < higher_index{
        let current_sum = current_lower_value + current_higher_value;
        match current_sum.cmp(&desired_sum){
            std::cmp::Ordering::Greater => {
                higher_index -= 1;
                current_higher_value = sorted_candidates[higher_index];
            },
            std::cmp::Ordering::Less => {
                lower_index += 1;
                current_lower_value = sorted_candidates[lower_index];
            },
            std::cmp::Ordering::Equal => return Some((current_lower_value, current_higher_value)),
        }
    }

    None
}

fn find_summing_triple(candidates: Vec<i32>, desired_sum: i32) -> Option<(i32, i32, i32)>{
    let mut sorted_numbers: Vec<i32> = candidates;
    sorted_numbers.sort_unstable();
    find_summing_triple_in_sorted(&sorted_numbers, desired_sum)
}

fn find_summing_triple_in_sorted(sorted_candidates: &[i32], desired_sum: i32) -> Option<(i32, i32, i32)>{
    if sorted_candidates.len() < 3 {
        return None;
    }

    let mut lowest_index: usize = 0;
    while sorted_candidates.len() >= lowest_index + 2 {
        let lowest_value: i32 = sorted_candidates[lowest_index];
        let other_values: Option<(i32, i32)> = find_summing_pair_in_sorted(sorted_candidates, desired_sum - lowest_value, lowest_index + 1);
        match other_values {
            Some((mid_value, highest_value)) => return Some((lowest_value, mid_value, highest_value)),
            None => lowest_index += 1
        }
    }

    None
}

#[cfg(test)]
mod day1_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from("1721
979
366
299
675
1456")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day1{});
        let problem_input = example_input();
        let expected_result = String::from("514579");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day1{});
        let problem_input = example_input();
        let expected_result = String::from("241861950");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day1{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 1, part: 1}).unwrap();
        let expected_result = String::from("928896");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day1{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 1, part: 2}).unwrap();
        let expected_result = String::from("295668576");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}