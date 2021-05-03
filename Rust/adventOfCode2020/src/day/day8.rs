use std::collections::HashSet;
use std::mem::replace;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction{
    NoOp(isize),
    Accumulate(isize),
    Jump(isize),
}

pub struct Day8 {}

impl super::Day for Day8{
    type PuzzleInput = Vec<Instruction>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .filter_map(|line| parsed_instruction(line))
            .collect()
    }

    fn solve_part1(&self, instructions: Self::PuzzleInput) -> std::string::String {
        let handheld = NonLoopingHandHeld {code: instructions};
        let stopping_accumulator = match handheld.execute(0, 0){
            ExecutionResult::Terminated(acc) => acc,
            ExecutionResult::AccessViolation(acc) => acc,
            ExecutionResult::LoopDetected(acc) => acc,
            ExecutionResult::Running => unreachable!("Execute cannot exit running!"),
        };
        stopping_accumulator.to_string()
    }

    fn solve_part2(&self, mut instructions: Self::PuzzleInput) -> std::string::String {
        for index in 0..instructions.len(){
            match instructions[index] {
                Instruction::NoOp(arg) => {
                    let new_instruction = Instruction::Jump(arg);
                    let original = replace(&mut instructions[index], new_instruction);
                    let handheld = NonLoopingHandHeld{ code: instructions.clone()};
                    if let ExecutionResult::Terminated(result) = handheld.execute(0, 0) {
                        return result.to_string();
                    }
                    instructions[index] = original;
                },
                Instruction::Jump(arg) => {
                    let new_instruction = Instruction::NoOp(arg);
                    let original = replace(&mut instructions[index], new_instruction);
                    let handheld = NonLoopingHandHeld{ code: instructions.clone()};
                    if let ExecutionResult::Terminated(result) = handheld.execute(0, 0) {
                        return result.to_string();
                    }
                    instructions[index] = original;
                },
                _ => ()
            }
        }
        String::from("Corruption not found!")
    }
}

fn parsed_instruction(instruction_text: &str) -> Option<Instruction>{
    lazy_static! {
        static ref INSTRUCTION_RE: regex::Regex = regex::Regex::new(r"(nop|acc|jmp) ((\+|-)\d+)").unwrap();
    }
    let captures: regex::Captures = INSTRUCTION_RE.captures(instruction_text)?;
    let argument = captures[2].parse::<isize>().ok()?;
    match &captures[1]{
        "nop" => Some(Instruction::NoOp(argument)),
        "acc" => Some(Instruction::Accumulate(argument)),
        "jmp" => Some(Instruction::Jump(argument)),
        _ => None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionResult{
    Terminated(isize),
    LoopDetected(isize),
    AccessViolation(isize),
    Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct HandHeldState{
    accumulator: isize,
    instruction_pointer: usize,
}

impl HandHeldState{
    fn execute_instruction(&mut self, instruction: &Instruction) -> ExecutionResult{
        match instruction{
            Instruction::NoOp(_) => self.instruction_pointer += 1,
            Instruction::Jump(jump_length) => {
                if *jump_length >= 0isize{
                    self.instruction_pointer += jump_length.unsigned_abs();
                } else {
                    let subtraction_value = jump_length.unsigned_abs();
                    if subtraction_value > self.instruction_pointer{
                        return ExecutionResult::AccessViolation(self.accumulator);
                    }
                    self.instruction_pointer -= subtraction_value;
                }
            },
            Instruction::Accumulate(value) => {
                self.accumulator += value;
                self.instruction_pointer += 1;
            }
        }
        ExecutionResult::Running
    }
}

struct NonLoopingHandHeld{
    code: Vec<Instruction>
}

impl NonLoopingHandHeld{
    fn execute(&self, entry_point: usize, initial_accumulator: isize) -> ExecutionResult{
        let mut state = HandHeldState{instruction_pointer: entry_point, accumulator: initial_accumulator};
        let mut visited_instruction: HashSet<usize> = HashSet::new();

        while visited_instruction.insert(state.instruction_pointer){
            if state.instruction_pointer == self.code.len(){
                return ExecutionResult::Terminated(state.accumulator);
            }
            if state.instruction_pointer > self.code.len(){
                return ExecutionResult::AccessViolation(state.accumulator);
            }
            let current_instruction = &self.code[state.instruction_pointer];
            match state.execute_instruction(current_instruction){
                ExecutionResult::Running => (),
                result => return result,
            }
        }

        ExecutionResult::LoopDetected(state.accumulator)
    }
}



#[cfg(test)]
mod day8_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day8{});
        let problem_input = example_input();
        let expected_result = String::from("5");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day8{});
        let problem_input = example_input();
        let expected_result = String::from("8");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day8{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 8, part: 1}).unwrap();
        let expected_result = String::from("1317");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day8{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 8, part: 2}).unwrap();
        let expected_result = String::from("1033");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}