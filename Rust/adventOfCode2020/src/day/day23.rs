use std::cmp::max;
use std::convert::TryFrom;






pub struct Day23 {}

impl super::Day for Day23{
    type PuzzleInput = Vec<usize>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.trim().chars()
            .filter_map(|c| if let Some(digit) = c.to_digit(10) { usize::try_from(digit).ok() } else { None })
            .collect()
    }

    fn solve_part1(&self, cups: Self::PuzzleInput) -> std::string::String {
        let number_of_rounds = 100;
        solve_part1_impl(&cups, number_of_rounds)
    }

    fn solve_part2(&self, start_cups: Self::PuzzleInput) -> std::string::String {
        let number_of_cups = 1000000;
        let number_of_rounds = 10000000;
        let game_results = play_crab_game(&start_cups, number_of_cups, number_of_rounds);

        let start_cup = 1;
        let next_cup = game_results[start_cup - 1] + 1;
        let second_next_cup = game_results[next_cup - 1] + 1;
        let result = next_cup * second_next_cup;
        result.to_string()
    }
}

fn solve_part1_impl(cups: &[usize], number_of_rounds: usize) -> String{
    let number_of_cups = 9;
    let game_results = play_crab_game(cups, number_of_cups, number_of_rounds);

    let start_cup = 1;
    let sequence_length = cups.len() - 1;
    let cups = cup_sequence(&game_results, start_cup, sequence_length);
    let cup_texts: Vec<String> = cups.iter()
        .map(|cup| cup.to_string())
        .collect();
    cup_texts.join("")
}

/// Returns the final linked cups list after playing.
fn play_crab_game(start_cups: &[usize], number_of_cups: usize, number_of_rounds: usize) -> Vec<usize>{
    let mut linked_cups = linked_cup_list(start_cups, number_of_cups);
    let start_cup = start_cups[0] - 1;
    play_crab_game_impl(&mut linked_cups, start_cup, number_of_rounds);
    linked_cups
}

/// Returns a list where the value at each index shows which index is next in the cup order.
/// The indices are always one less than the corresponding labels.
///
/// # Arguments
///
/// - `start_cups` - Cups that start the loop.
/// - `number of cups` - Total number of cups in the loop. Will be ignored if smaller than the number of start cups.
fn linked_cup_list(start_cups: &[usize], number_of_cups: usize) -> Vec<usize>{
    let mut linked_list = Vec::with_capacity(max(start_cups.len(), number_of_cups));
    let start_cup = start_cups[0] - 1;
    let next_after_start_cups = if number_of_cups <= start_cups.len() { start_cup } else { start_cups.len() };
    let mut start_items: Vec<_> = start_cups.iter()
        .enumerate()
        .map(|(index, cup)| (cup - 1, if index == start_cups.len() - 1 { next_after_start_cups } else { start_cups[index + 1] - 1}))
        .collect();
    start_items.sort_by(|(cup1, _), (cup2, _)| cup1.cmp(cup2));
    for (_index, cup) in start_items.iter(){
        linked_list.push(*cup);
    }
    if number_of_cups > start_cups.len() {
        for further_cup in (start_cups.len()+1)..number_of_cups {
            linked_list.push(further_cup);
        }
        linked_list.push(start_cup);
    }
    linked_list
}


fn play_crab_game_impl(linked_cup_list: &mut [usize], start_cup: usize, number_of_rounds: usize){
    let mut current_cup = start_cup;
    let number_of_cups = linked_cup_list.len();
    for _ in 0..number_of_rounds{
        current_cup = play_crab_game_round(linked_cup_list, current_cup, number_of_cups);
    }
}

/// Returns the current cup after the round.
fn play_crab_game_round(linked_cup_list: &mut [usize], current_cup: usize, number_of_cups: usize) -> usize {
    let mut destination = if current_cup == 0 {number_of_cups - 1} else {current_cup - 1};
    let first_moved_cup = linked_cup_list[current_cup];
    let mid_moved_cup = linked_cup_list[first_moved_cup];
    let last_moved_cup = linked_cup_list[mid_moved_cup];

    let next_current_cup = linked_cup_list[last_moved_cup];
    linked_cup_list[current_cup] = next_current_cup;

    while destination == first_moved_cup
            || destination == mid_moved_cup
            || destination == last_moved_cup {
        destination = if destination == 0 { number_of_cups - 1 } else { destination - 1 };
    }

    let after_destination_cup = linked_cup_list[destination];
    linked_cup_list[destination] = first_moved_cup;
    linked_cup_list[last_moved_cup] = after_destination_cup;

    next_current_cup
}

fn cup_sequence(linked_cup_list: &[usize], start_cup: usize, sequence_length: usize) -> Vec<usize>{
    let mut cup_sequence = Vec::with_capacity(sequence_length);
    let mut current_cup = start_cup;
    for _ in 0..sequence_length{
        current_cup = linked_cup_list[current_cup - 1] + 1;
        cup_sequence.push(current_cup);
    }
    cup_sequence
}


#[cfg(test)]
mod day23_tests {
    use super::*;
    use crate::input;
    use crate::day;
    use rstest::rstest;

    fn example_input() -> String{
        String::from(
r#"389125467"#)
    }

    #[rstest]
    #[case(0, String::from("25467389"))]
    #[case(1, String::from("54673289"))]
    #[case(2, String::from("32546789"))]
    #[case(3, String::from("34672589"))]
    #[case(4, String::from("32584679"))]
    #[case(5, String::from("36792584"))]
    #[case(6, String::from("93672584"))]
    #[case(7, String::from("92583674"))]
    #[case(8, String::from("58392674"))]
    #[case(9, String::from("83926574"))]
    #[case(10, String::from("92658374"))]
    fn basic_examples_part1(#[case] number_of_rounds: usize, #[case] expected_result: String) {
        let day: Box<dyn day::Day<PuzzleInput = Vec<usize>>> = Box::new(Day23{});
        let problem_input = example_input();
        let cups = day.parse_input(problem_input);
        let actual_result = super::solve_part1_impl(&cups, number_of_rounds);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day23{});
        let problem_input = example_input();
        let expected_result = 67384529.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day23{});
        let problem_input = example_input();
        let expected_result = 149245887792usize.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day23{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 23, part: 1}).unwrap();
        let expected_result = String::from("82934675");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day23{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 23, part: 2}).unwrap();
        let expected_result = String::from("474600314018");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}