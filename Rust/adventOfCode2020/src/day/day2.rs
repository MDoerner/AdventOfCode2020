use regex::Regex;

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
        return number_of_valid_passwords.to_string();
    }

    fn solve_part2(&self, input: Self::PuzzleInput) -> std::string::String {
        let number_of_valid_passwords = input.iter()
            .filter(|(password, rule)| is_valid_toboggan_password(password, rule))
            .count();
    return number_of_valid_passwords.to_string();
    }
}

fn parse_password_data(line: &str) -> Option<(String, PasswordRule)>{
    let re = Regex::new(r"(\d+)-(\d+) (\w): (.+)").unwrap();
    let captures: regex::Captures;
    match re.captures(line){
        Some(cap) => captures = cap,
        None => return None
    }
    let password = captures[4].to_owned();
    let character: char;
    let min_number: usize;
    let max_number: usize;
    match captures[3].chars().next(){
        Some(c) => character = c,
        None => return None
    }
    match captures[1].parse::<usize>(){
        Ok(n) => min_number = n,
        Err(_) => return None
    }
    match captures[2].parse::<usize>(){
        Ok(n) => max_number = n,
        Err(_) => return None
    }
    let rule = PasswordRule {character: character, min_number: min_number, max_number: max_number};

    return Some((password, rule));
}

fn is_valid_sled_password(password: &str, rule: &PasswordRule)-> bool {
    let character_count = occurrence_count(password, &rule.character);
    return character_count >= rule.min_number 
        && character_count <= rule.max_number
}

fn occurrence_count(text: &str, character: &char) -> usize{
    return text.chars().filter(|c| c == character).count();
}

fn is_valid_toboggan_password(password: &str, rule: &PasswordRule)-> bool {
    let password_characters = password.chars().collect::<Vec<char>>();
    return password_characters[rule.min_number - 1] == rule.character
            && password_characters[rule.max_number - 1] != rule.character
        || password_characters[rule.max_number - 1] == rule.character
            && password_characters[rule.min_number - 1] != rule.character;
}