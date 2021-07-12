use crate::formula_parsing::{Formula};
use crate::formula_parsing::formula_parser::{self, FormulaParser};

pub struct Day18 {}

impl super::Day for Day18{
    type PuzzleInput = Vec<Formula>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .map(|line| Formula(line.to_owned()))
            .collect()
    }

    fn solve_part1(&self, formulas: Self::PuzzleInput) -> std::string::String {
        let parsing_strategy = formula_parser::LeftToRightEvaluationBinaryOpParsingStrategy::new();
        let parser = formula_parser::RightToLeftParser::new(parsing_strategy);
        let result: i128 = formulas.iter()
            .filter_map(|formula| parser.parse(formula))
            .map(|expr| expr.evaluate())
            .sum();
        result.to_string()
    }

    fn solve_part2(&self, formulas: Self::PuzzleInput) -> std::string::String {
        let parsing_strategy = formula_parser::PlusBeforeMultEvaluationBinaryOpParsingStrategy::new();
        let parser = formula_parser::RightToLeftParser::new(parsing_strategy);
        let result: i128 = formulas.iter()
            .filter_map(|formula| parser.parse(formula))
            .map(|expr| expr.evaluate())
            .sum();
        result.to_string()
    }
}





#[cfg(test)]
mod day18_tests {
    use super::*;
    use crate::input;
    use crate::day;
    use rstest::rstest;

    #[rstest]
    #[case(String::from("1 + 2 * 3 + 4 * 5 + 6"), 71)]
    #[case(String::from("1 + (2 * 3) + (4 * (5 + 6))"), 51)]
    #[case(String::from("2 * 3 + (4 * 5)"), 26)]
    #[case(String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437)]
    #[case(String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240)]
    #[case(String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632)]
    fn examples_part1(#[case] problem_input: String, #[case] expected_result: i128) {
        let day: Box<dyn day::DaySolver> = Box::new(Day18{});
        let expected_result_text = expected_result.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result_text);
    }

    #[rstest]
    #[case(String::from("1 + 2 * 3 + 4 * 5 + 6"), 231)]
    #[case(String::from("1 + (2 * 3) + (4 * (5 + 6))"), 51)]
    #[case(String::from("2 * 3 + (4 * 5)"), 46)]
    #[case(String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445)]
    #[case(String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060)]
    #[case(String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340)]
    fn examples_part2(#[case] problem_input: String, #[case] expected_result: i128) {
        let day: Box<dyn day::DaySolver> = Box::new(Day18{});
        let expected_result_text = expected_result.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result_text);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day18{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 18, part: 1}).unwrap();
        let expected_result = String::from("280014646144");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day18{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 18, part: 2}).unwrap();
        let expected_result = String::from("9966990988262");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}