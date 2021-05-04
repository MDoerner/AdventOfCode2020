

pub struct Day10 {}

impl super::Day for Day10{
    type PuzzleInput = Vec<usize>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .map(|line| line.parse::<usize>())
            .filter_map(Result::ok)
            .collect()
    }

    fn solve_part1(&self, mut adapters: Self::PuzzleInput) -> std::string::String {
        adapters.sort_unstable();
        let mut joltage_gaps = [0, 0, 0];
        joltage_gaps[adapters[0]-1] += 1;
        for pair in adapters.windows(2){
            let joltage_gap = pair[1] - pair[0];
            joltage_gaps[joltage_gap - 1] += 1;
        }
        joltage_gaps[3-1] += 1;
        let result = joltage_gaps[0] * joltage_gaps[3-1];
        result.to_string()
    }

    fn solve_part2(&self, mut adapters: Self::PuzzleInput) -> std::string::String {
        adapters.sort_unstable();
        let result = number_of_possible_adapter_combinations(&adapters);
        result.to_string()
    }
}

fn number_of_possible_adapter_combinations(sorted_adapters: &[usize]) -> usize{
    if sorted_adapters.is_empty(){
        return 1;
    }

    let mut combinations_by_gap = [1, 1, 1];
    for adapter_pair in sorted_adapters.windows(2).rev(){
        let current_gap = adapter_pair[1] - adapter_pair[0];
        combinations_by_gap = combinations_by_previous_gap(combinations_by_gap, current_gap);
    }

    combinations_by_gap[sorted_adapters[0] - 1]
}

fn combinations_by_previous_gap(prior_combinations_by_previous_gap: [usize; 3], current_gap: usize) -> [usize; 3]{
    match current_gap{
        1 => [
            prior_combinations_by_previous_gap[0] + prior_combinations_by_previous_gap[1],
            prior_combinations_by_previous_gap[0] + prior_combinations_by_previous_gap[2],
            prior_combinations_by_previous_gap[0],
        ],
        2 => [
            prior_combinations_by_previous_gap[1] + prior_combinations_by_previous_gap[2],
            prior_combinations_by_previous_gap[1],
            prior_combinations_by_previous_gap[1],
        ],
        3 => [
            prior_combinations_by_previous_gap[2],
            prior_combinations_by_previous_gap[2],
            prior_combinations_by_previous_gap[2],
        ],
        _ => [0, 0, 0],
    }
}






#[cfg(test)]
mod day10_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"16
10
15
5
1
11
7
19
6
12
4")
    }

    fn example2_input() -> String{
        String::from(
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day10{});
        let problem_input = example_input();
        let expected_result = (7 * 5).to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example2_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day10{});
        let problem_input = example2_input();
        let expected_result = (22 * 10).to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day10{});
        let problem_input = example_input();
        let expected_result = 8.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example2_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day10{});
        let problem_input = example2_input();
        let expected_result = 19208.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day10{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 10, part: 1}).unwrap();
        let expected_result = String::from("2368");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day10{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 10, part: 2}).unwrap();
        let expected_result = String::from("1727094849536");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}