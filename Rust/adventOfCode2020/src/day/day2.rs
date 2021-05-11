

pub struct PasswordRule{
    character: char,
    min_number: usize,
    max_number: usize,
}

pub struct Day2 {}

impl super::Day for Day2{
    type PuzzleInput = Vec<(String, PasswordRule)>;
    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .flat_map(|line| parse_password_data(line))
            .collect::<Vec<(String, PasswordRule)>>()
    }

    fn solve_part1(&self, input: Self::PuzzleInput) -> std::string::String {
        let number_of_valid_passwords = input.iter()
            .filter(|(password, rule)| is_valid_sled_password(password, rule))
            .count();
        number_of_valid_passwords.to_string()
    }

    fn solve_part2(&self, input: Self::PuzzleInput) -> std::string::String {
        let number_of_valid_passwords = input.iter()
            .filter(|(password, rule)| is_valid_toboggan_password(password, rule))
            .count();
        number_of_valid_passwords.to_string()
    }
}

fn parse_password_data(line: &str) -> Option<(String, PasswordRule)>{
    lazy_static! {
        static ref PASSWORD_RE: regex::Regex = regex::Regex::new(r"^(\d+)-(\d+) (\w): (.+)$").unwrap();
    }
    let captures: regex::Captures = PASSWORD_RE.captures(line)?;
    let password = captures[4].to_owned();
    let character = captures[3].chars().next()?;
    let min_number = captures[1].parse::<usize>().ok()?;
    let max_number = captures[2].parse::<usize>().ok()?;
    let rule = PasswordRule {character, min_number, max_number};

    Some((password, rule))
}

fn is_valid_sled_password(password: &str, rule: &PasswordRule)-> bool {
    let character_count = occurrence_count(password, &rule.character);
    character_count >= rule.min_number
        && character_count <= rule.max_number
}

fn occurrence_count(text: &str, character: &char) -> usize{
    return text.chars().filter(|c| c == character).count();
}

fn is_valid_toboggan_password(password: &str, rule: &PasswordRule)-> bool {
    let password_characters = password.chars().collect::<Vec<char>>();
    password_characters[rule.min_number - 1] == rule.character
            && password_characters[rule.max_number - 1] != rule.character
        || password_characters[rule.max_number - 1] == rule.character
            && password_characters[rule.min_number - 1] != rule.character
}

#[cfg(test)]
mod day2_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day2{});
        let problem_input = example_input();
        let expected_result = String::from("2");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day2{});
        let problem_input = example_input();
        let expected_result = String::from("1");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day2{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 2, part: 1}).unwrap();
        let expected_result = String::from("500");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day2{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 2, part: 2}).unwrap();
        let expected_result = String::from("313");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}