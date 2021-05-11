use std::{collections::HashMap, mem};
use std::convert::TryFrom;

pub struct Day15 {}

impl super::Day for Day15{
    type PuzzleInput = Vec<u128>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.split(',')
            .filter_map(|number| number.parse::<u128>().ok())
            .collect()
    }

    fn solve_part1(&self, initial_numbers: Self::PuzzleInput) -> std::string::String {
        let number = number_spoken(&initial_numbers,2020 - 1);
        number.to_string()
    }

    fn solve_part2(&self, initial_numbers: Self::PuzzleInput) -> std::string::String {
        let number = number_spoken(&initial_numbers,30000000 - 1);
        number.to_string()
    }
}

fn number_spoken(initial_numbers: &[u128], position: u128) -> u128{
    if initial_numbers.len() == 0 && position == 0{
        return 0;
    }
    let maybe_position_pointer = usize::try_from(position);
    //If these two conditions to not hold, position is guaranteed to be larger than or equal to initial_numbers.len(),
    //since the first conversion can only fail because position is larger than any usize.
    if let Ok(position_pointer) = maybe_position_pointer{
        if position_pointer < initial_numbers.len(){
            return initial_numbers[position_pointer];
        }
    }
    let mut memory = initial_memory(initial_numbers);
    //The conversion cannot fail because we already know that the valid u128 position is larger at least as large as nitial_numbers.len().
    let start_position = u128::try_from(initial_numbers.len()).unwrap();
    let last_initial_number = match initial_numbers.last() {
        Some(number) => number.to_owned(),
        None => 0,
    };
    //This cannot be None since start_position is smaller than or equal to position.
    number_at_position(&mut memory, last_initial_number, start_position, position).unwrap()
}

fn initial_memory(initial_numbers: &[u128]) -> HashMap<u128, u128>{
    let mut memory = HashMap::new();
    if initial_numbers.is_empty(){
        return memory;
    }
    for (index, number) in initial_numbers
        .split_last().unwrap().1
        .iter()
        .enumerate()
        .filter_map(|(ind, value)| match u128::try_from(ind){
            Ok(i) => Some((i, value.to_owned())),
            Err(_) => None,
        }){
            memory.insert(number, index);
    }
    memory
}

fn number_at_position(memory: &mut HashMap<u128, u128>, initial_last_number: u128, start_position: u128, target_position: u128) -> Option<u128>{
    if target_position < start_position{
        return None;
    }

    let mut last_number = initial_last_number;
    for position in start_position..(target_position + 1){
        let current_number = match memory.get(&last_number){
            Some(previous_position) => (position- 1) -previous_position,
            None => 0,
        };
        memory.insert(last_number, position - 1);
        last_number = current_number;
    }

    Some(last_number)
}




#[cfg(test)]
mod day15_tests {
    use super::*;
    use crate::input;
    use crate::day;
    use rstest::rstest;

    #[rstest]
    #[case(1, 0)]
    #[case(2, 3)]
    #[case(3, 6)]
    #[case(4, 0)]
    #[case(5, 3)]
    #[case(6, 3)]
    #[case(7, 1)]
    #[case(8, 0)]
    #[case(9, 4)]
    #[case(10, 0)]
    fn number_spoken_tests(#[case] turn: u128, #[case] expected_result: u128) {
        let input = vec![0,3,6];
        let actual_result = number_spoken(&input, turn - 1);
        assert_eq!(actual_result, expected_result);
    }

    #[rstest]
    #[case(String::from("0,3,6"), 436)]
    #[case(String::from("1,3,2"), 1)]
    #[case(String::from("2,1,3"), 10)]
    #[case(String::from("1,2,3"), 27)]
    #[case(String::from("2,3,1"), 78)]
    #[case(String::from("3,2,1"), 438)]
    #[case(String::from("3,1,2"), 1836)]
    fn examples_part1(#[case] problem_input: String, #[case] expected_result: u128) {
        let day: Box<dyn day::DaySolver> = Box::new(Day15{});
        let expected_result = expected_result.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    /* Takes too long!!!!
    #[rstest]
    #[case(String::from("0,3,6"), 175594)]
    #[case(String::from("1,3,2"), 2578)]
    #[case(String::from("2,1,3"), 3544142)]
    #[case(String::from("1,2,3"), 261214)]
    #[case(String::from("2,3,1"), 6895259)]
    #[case(String::from("3,2,1"), 18)]
    #[case(String::from("3,1,2"), 362)]
    fn example_part2(#[case] problem_input: String, #[case] expected_result: u128) {
        let day: Box<dyn day::DaySolver> = Box::new(Day15{});
        let expected_result = expected_result.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
    */

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day15{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 15, part: 1}).unwrap();
        let expected_result = String::from("929");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    /* Takes too long!!!!
    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day15{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 15, part: 2}).unwrap();
        let expected_result = String::from("16671510");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
    */
}