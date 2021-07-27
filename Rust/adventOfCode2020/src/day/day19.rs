
use crate::parser_generator;


pub struct MessageData{
    grammar_text: String,
    messages: Vec<String>,
}

pub struct Day19 {}

impl super::Day for Day19{
    type PuzzleInput = MessageData;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let sections: Vec<&str> = text.split("\n\n").collect();
        let grammar_text = sections[0].to_owned();
        let messages = sections[1].lines().map(|line| line.to_owned()).collect();
        MessageData { grammar_text, messages }
    }

    fn solve_part1(&self, data: Self::PuzzleInput) -> std::string::String {
        let grammar = parser_generator::parse_grammar(&data.grammar_text, None);
        let parser = parser_generator::Parser::new(grammar);
        let possibilities_to_parse_messages = data.messages.iter()
            .map(|message| parser.parse(message, 0));
        let number_of_parsable_messages = possibilities_to_parse_messages
            .filter(|possibilities| !possibilities.is_empty())
            .count();
        number_of_parsable_messages.to_string()
    }

    fn solve_part2(&self, data: Self::PuzzleInput) -> std::string::String {
        let correction_rules = "8: 42 | 42 8
11: 42 31 | 42 11 31";
        let grammar = parser_generator::parse_grammar(&data.grammar_text, Some(correction_rules));
        let parser = parser_generator::Parser::new(grammar);
        let possibilities_to_parse_messages = data.messages.iter()
            .map(|message| parser.parse(message, 0));
        let number_of_parsable_messages = possibilities_to_parse_messages
            .filter(|possibilities| !possibilities.is_empty())
            .count();
        number_of_parsable_messages.to_string()
    }
}




#[cfg(test)]
mod day19_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#)
    }

    fn example_input2() -> String{
        String::from(
r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#)
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day19{});
        let problem_input = example_input();
        let expected_result = 2.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example2_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day19{});
        let problem_input = example_input2();
        let expected_result = 3.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example2_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day19{});
        let problem_input = example_input2();
        let expected_result = 12.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day19{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 19, part: 1}).unwrap();
        let expected_result = String::from("144");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day19{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 19, part: 2}).unwrap();
        let expected_result = String::from("260");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}