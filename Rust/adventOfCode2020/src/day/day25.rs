
pub struct DoorCard{
    card_public_key: u64,
    door_public_key: u64,
    base: u64,
    key_space_size: u64,
}

pub struct Day25 {}

impl super::Day for Day25{
    type PuzzleInput = DoorCard;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let keys: Vec<_> = text.lines().filter_map(|line| line.parse::<u64>().ok()).collect();
        DoorCard {
            card_public_key: keys[0],
            door_public_key: keys[1],
            base: 7,
            key_space_size: 20201227,
        }
    }

    fn solve_part1(&self, card: Self::PuzzleInput) -> std::string::String {
        let key = encryption_key(&card).unwrap();
        key.to_string()
    }

    fn solve_part2(&self, _: Self::PuzzleInput) -> std::string::String {
        unimplemented!();
    }
}

fn encryption_key(card: &DoorCard) -> Option<u64>{
    let subject_number = card.base;
    let key_space_size = card.key_space_size;
    let card_loop_size = brute_force_loop_size(card.card_public_key, subject_number, key_space_size)?;
    Some(transformed_key(card.door_public_key, card_loop_size, key_space_size))
}

fn brute_force_loop_size(key: u64, subject_number: u64, key_space_size: u64) -> Option<u64>{
    let mut comparison_key = 1;
    for loop_size in 0..key_space_size{
        if comparison_key == key{
            return Some(loop_size);
        }

        comparison_key = (comparison_key * subject_number) % key_space_size;
    }

    None
}

fn transformed_key(key: u64, loop_size: u64, key_space_size: u64) -> u64{
    let mut result_key = 1;
    for _ in 0..loop_size{
        result_key = (result_key * key) % key_space_size;
    }
    result_key
}


#[cfg(test)]
mod day25_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
r#"5764801
17807724"#)
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day25{});
        let problem_input = example_input();
        let expected_result = 14897079.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day25{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 25, part: 1}).unwrap();
        let expected_result = String::from("6421487");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}